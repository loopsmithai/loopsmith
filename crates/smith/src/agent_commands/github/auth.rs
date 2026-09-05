use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::formation::{self, CredentialDomain, KeyValueCredentialStore};
use crate::git::{app_auth, manifest_flow::credential_keys};

/// A guard that holds the temp GH_CONFIG_DIR and cleans up on drop.
pub struct GhAuthGuard {
    _temp_dir: tempfile::TempDir,
}

/// Cached token on disk.
#[derive(Serialize, Deserialize)]
struct CachedToken {
    token: String,
    expires_at: DateTime<Utc>,
}

/// Token cache path: ~/.cache/smith-agent/github/token-{org}-{id}.json
fn token_cache_path(org: &str, id: &str) -> PathBuf {
    let cache_dir = dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("smith-agent")
        .join("github");
    cache_dir.join(format!("token-{}-{}.json", org, id))
}

/// Try to load a cached token. Returns None if missing, expired, or within
/// 5 minutes of expiry (safety margin for the operation to complete).
fn load_cached_token(org: &str, id: &str) -> Option<String> {
    let path = token_cache_path(org, id);
    let content = std::fs::read_to_string(&path).ok()?;
    let cached: CachedToken = serde_json::from_str(&content).ok()?;

    let margin = chrono::Duration::minutes(5);
    if Utc::now() + margin < cached.expires_at {
        Some(cached.token)
    } else {
        let _ = std::fs::remove_file(&path);
        None
    }
}

/// Save a token to the cache.
fn save_cached_token(org: &str, id: &str, token: &str, expires_at: DateTime<Utc>) {
    let path = token_cache_path(org, id);
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let cached = CachedToken {
        token: token.to_string(),
        expires_at,
    };
    if let Ok(json) = serde_json::to_string(&cached) {
        let _ = std::fs::write(&path, json);
    }
}

/// Read the cached token expiry without returning the token itself.
fn load_cached_token_expiry(org: &str, id: &str) -> Option<String> {
    let path = token_cache_path(org, id);
    let content = std::fs::read_to_string(&path).ok()?;
    let cached: CachedToken = serde_json::from_str(&content).ok()?;
    Some(cached.expires_at.to_rfc3339())
}

/// Sets up GitHub App authentication.
///
/// Reads App credentials (client_id, private_key) from the shared app store,
/// installation_id from the per-org installation store, generates a JWT,
/// exchanges it for an installation token, and points GH_CONFIG_DIR at a
/// temp directory with hosts.yml.
///
/// Tokens are cached on disk with a 5-minute safety margin before expiry.
pub fn setup_github_auth(org: &str, id: &str) -> Result<GhAuthGuard> {
    let token = match load_cached_token(org, id) {
        Some(cached) => {
            eprintln!("✓ Using cached installation token");
            cached
        }
        None => {
            let formation = formation::local::create_local_formation(org).context(
                "Could not create formation for credential lookup. \
                 Hint: run 'smith init' first to register a GitHub App.",
            )?;

            // App credentials — shared across orgs
            let app_store = formation
                .credential_store(CredentialDomain::GitHubApp {
                    agent_id: id.to_string(),
                })
                .context("Could not open app credential store.")?;

            // Installation — per-org
            let install_store = formation
                .credential_store(CredentialDomain::GitHubInstallation {
                    org: org.to_string(),
                    agent_id: id.to_string(),
                })
                .context("Could not open installation credential store.")?;

            let client_id = app_store
                .retrieve(&credential_keys::client_id(id))?
                .context(
                    "GitHub App client_id not found in keyring. \
                     Hint: run 'smith init' to create and install a GitHub App.",
                )?;

            let private_key = app_store
                .retrieve(&credential_keys::private_key(id))?
                .context(
                    "GitHub App private_key not found in keyring. \
                     Hint: run 'smith init' to create and install a GitHub App.",
                )?;

            let installation_id_str = install_store
                .retrieve(&credential_keys::installation_id(id))?
                .context(
                    "GitHub App installation_id not found in keyring. \
                     Hint: run 'smith install --org <org>' to install the App on this org.",
                )?;

            let installation_id: u64 = installation_id_str
                .parse()
                .context("installation_id is not a valid number")?;

            eprintln!("→ Generating JWT from App credentials...");
            let jwt = app_auth::generate_jwt(&client_id, &private_key).context(
                "Failed to generate JWT from App credentials. \
                 Hint: the private key may be corrupted. Re-run 'smith init'.",
            )?;

            eprintln!("→ Exchanging JWT for installation token...");
            let inst_token =
                app_auth::exchange_for_installation_token(&jwt, installation_id).context(
                    "Failed to exchange JWT for installation token. \
                     Hint: the GitHub App may have been uninstalled. Check the App at github.com.",
                )?;

            eprintln!(
                "✓ Authenticated as GitHub App (installation {})",
                installation_id
            );

            save_cached_token(org, id, &inst_token.token, inst_token.expires_at);
            inst_token.token
        }
    };

    // Write token to a temp GH_CONFIG_DIR/hosts.yml
    let temp_dir =
        tempfile::tempdir().context("Failed to create temp directory for GH_CONFIG_DIR")?;

    let hosts_yml = temp_dir.path().join("hosts.yml");
    let hosts_content = format!(
        "github.com:\n  oauth_token: {}\n  user: x-access-token\n  git_protocol: https\n",
        token
    );
    std::fs::write(&hosts_yml, &hosts_content).context("Failed to write hosts.yml")?;

    std::env::set_var("GH_CONFIG_DIR", temp_dir.path());
    std::env::remove_var("GH_TOKEN");
    std::env::remove_var("GITHUB_TOKEN");

    Ok(GhAuthGuard {
        _temp_dir: temp_dir,
    })
}

/// Mint a token and write a persistent GH_CONFIG_DIR with hosts.yml.
/// Prints the config dir path in the envelope result so the caller can
/// `export GH_CONFIG_DIR=<path>` for git/gh use.
pub fn mint_token_to_config_dir(org: &str, id: &str) -> Result<()> {
    // setup_github_auth mints (or loads cached) and sets GH_CONFIG_DIR on a tempdir.
    // We need the token to write to a *persistent* location.
    let _guard = setup_github_auth(org, id)?;

    let token = load_cached_token(org, id)
        .context("Token was just minted but not found in cache")?;

    // Persistent well-known dir: ~/.config/smith-agent/gh-config/{org}/
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("smith-agent")
        .join("gh-config")
        .join(org);
    std::fs::create_dir_all(&config_dir)
        .context("Failed to create persistent GH_CONFIG_DIR")?;

    let hosts_yml = config_dir.join("hosts.yml");
    let hosts_content = format!(
        "github.com:\n  oauth_token: {}\n  user: x-access-token\n  git_protocol: https\n",
        token
    );
    std::fs::write(&hosts_yml, &hosts_content)
        .context("Failed to write hosts.yml")?;

    let config_path = config_dir.to_string_lossy().to_string();
    let status = serde_json::json!({
        "gh_config_dir": config_path,
        "org": org,
        "id": id,
        "hint": format!("export GH_CONFIG_DIR={}", config_path),
    });
    println!("{}", serde_json::to_string_pretty(&status)?);
    Ok(())
}

/// Print auth status as JSON. Called after setup_github_auth has already run
/// (via mod.rs), so the token is cached and credentials are validated.
pub fn print_auth_status(org: &str, id: &str) -> Result<()> {
    let formation = formation::local::create_local_formation(org)?;

    let app_store = formation.credential_store(CredentialDomain::GitHubApp {
        agent_id: id.to_string(),
    })?;
    let install_store = formation.credential_store(CredentialDomain::GitHubInstallation {
        org: org.to_string(),
        agent_id: id.to_string(),
    })?;

    let app_id = app_store.retrieve(&credential_keys::app_id(id))?;
    let client_id = app_store.retrieve(&credential_keys::client_id(id))?;
    let installation_id = install_store.retrieve(&credential_keys::installation_id(id))?;
    let has_private_key = app_store
        .retrieve(&credential_keys::private_key(id))?
        .is_some();

    let token_expires_at = load_cached_token_expiry(org, id);

    let status = serde_json::json!({
        "org": org,
        "id": id,
        "app_id": app_id,
        "client_id": client_id,
        "installation_id": installation_id,
        "has_private_key": has_private_key,
        "keyring_service_app": format!("loopsmith.app.{}", id),
        "keyring_service_installation": format!("loopsmith.{}.installation", org),
        "token_expires_at": token_expires_at,
    });

    println!("{}", serde_json::to_string_pretty(&status)?);
    Ok(())
}
