use anyhow::{bail, Context, Result};

use crate::formation::{self, CredentialDomain, KeyValueCredentialStore};
use crate::git::manifest_flow::credential_keys;
use crate::git::app_auth;

/// `smith install` — install an existing GitHub App on a new org.
///
/// Flow:
///   1. Read App credentials from shared keyring store
///   2. Generate JWT and query `GET /app` to get the App slug
///   3. Snapshot current installations
///   4. Show the user the installation URL
///   5. Poll `GET /app/installations` until a new one appears
///   6. Store installation_id in the per-org keyring entry
pub fn run(id: &str) -> Result<()> {
    cliclack::intro("Smith — Install GitHub App on a new org")?;

    // Read App credentials from shared store — use a dummy org for the app store
    // (app credentials are shared across orgs, the org param here is only used for
    // the installation store which we don't need yet)
    let formation = formation::local::create_local_formation("_")?;
    let app_store = formation.credential_store(CredentialDomain::GitHubApp {
        agent_id: id.to_string(),
    })?;

    let client_id = app_store
        .retrieve(&credential_keys::client_id(id))?
        .context(
            "GitHub App client_id not found. Run 'smith init' first to create a GitHub App.",
        )?;

    let private_key = app_store
        .retrieve(&credential_keys::private_key(id))?
        .context(
            "GitHub App private_key not found. Run 'smith init' first to create a GitHub App.",
        )?;

    let app_id_str = app_store
        .retrieve(&credential_keys::app_id(id))?
        .context("GitHub App app_id not found. Run 'smith init' first.")?;

    let api_base = std::env::var("SMITH_GITHUB_API_BASE")
        .unwrap_or_else(|_| "https://api.github.com".to_string());
    let web_base = std::env::var("SMITH_GITHUB_WEB_BASE")
        .unwrap_or_else(|_| "https://github.com".to_string());

    // Generate JWT and get the App slug
    cliclack::log::step("Querying GitHub App details...")?;
    let jwt = app_auth::generate_jwt(&client_id, &private_key)
        .context("Failed to generate JWT. The private key may be corrupted.")?;

    let rt = tokio::runtime::Runtime::new().context("Failed to create tokio runtime")?;

    let (app_slug, existing_installations) = rt.block_on(async {
        let client = reqwest::Client::new();

        // Get app slug
        let resp = client
            .get(format!("{api_base}/app"))
            .header("Authorization", format!("Bearer {jwt}"))
            .header("Accept", "application/vnd.github+json")
            .header("User-Agent", "smith")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await
            .context("Failed to query App details")?;

        if !resp.status().is_success() {
            let body = resp.text().await.unwrap_or_default();
            bail!("GitHub API returned error querying App: {body}");
        }

        let body: serde_json::Value = resp.json().await.context("Failed to parse App response")?;
        let slug = body["slug"]
            .as_str()
            .context("App response missing 'slug' field")?
            .to_string();

        // Snapshot existing installations
        let resp = client
            .get(format!("{api_base}/app/installations"))
            .header("Authorization", format!("Bearer {jwt}"))
            .header("Accept", "application/vnd.github+json")
            .header("User-Agent", "smith")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .send()
            .await
            .context("Failed to query existing installations")?;

        let existing: Vec<serde_json::Value> = if resp.status().is_success() {
            resp.json().await.unwrap_or_default()
        } else {
            Vec::new()
        };

        let existing_ids: std::collections::HashSet<u64> = existing
            .iter()
            .filter_map(|inst| inst["id"].as_u64())
            .collect();

        Ok::<(String, std::collections::HashSet<u64>), anyhow::Error>((slug, existing_ids))
    })?;

    let install_url = format!("{web_base}/apps/{app_slug}/installations/new");

    cliclack::log::info(format!("App: {} (id: {})", app_slug, app_id_str))?;

    if !existing_installations.is_empty() {
        cliclack::log::info(format!(
            "Already installed on {} org(s)",
            existing_installations.len()
        ))?;
    }

    cliclack::log::step(format!(
        "Open this URL to install the App on a new org:\n\n  {}\n\n\
         Select an org, choose repository access, and click Install.",
        install_url
    ))?;

    // Poll for a new installation
    let spinner = cliclack::spinner();
    spinner.start("Waiting for new installation (polling GitHub every 5s)...");

    let (org, installation_id) = rt.block_on(async {
        let client = reqwest::Client::new();
        let max_attempts = 60; // 5 minutes

        for attempt in 1..=max_attempts {
            let jwt = app_auth::generate_jwt(&client_id, &private_key)?;

            let resp = client
                .get(format!("{api_base}/app/installations"))
                .header("Authorization", format!("Bearer {jwt}"))
                .header("Accept", "application/vnd.github+json")
                .header("User-Agent", "smith")
                .header("X-GitHub-Api-Version", "2022-11-28")
                .send()
                .await
                .context("Failed to query installations")?;

            if resp.status().is_success() {
                let installations: Vec<serde_json::Value> =
                    resp.json().await.context("Failed to parse installations")?;

                for inst in &installations {
                    if let (Some(inst_id), Some(account)) =
                        (inst["id"].as_u64(), inst["account"]["login"].as_str())
                    {
                        if !existing_installations.contains(&inst_id) {
                            return Ok((account.to_string(), inst_id));
                        }
                    }
                }
            }

            if attempt == max_attempts {
                bail!(
                    "Timed out waiting for a new installation.\n\
                     Make sure you installed the App via:\n  {}",
                    install_url
                );
            }

            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }

        bail!("Unreachable")
    })?;

    spinner.stop(format!("Found installation on '{}': {}", org, installation_id));

    // Store the installation_id under the discovered org
    let formation = formation::local::create_local_formation(&org)?;
    let install_store = formation.credential_store(CredentialDomain::GitHubInstallation {
        org: org.clone(),
        agent_id: id.to_string(),
    })?;

    install_store
        .store(
            &credential_keys::installation_id(id),
            &installation_id.to_string(),
        )
        .context("Failed to store installation ID in keyring")?;

    cliclack::log::success(format!(
        "Installed! org='{}', installation_id={}",
        org, installation_id
    ))?;
    cliclack::outro(format!(
        "You can now use: smith-agent github --org {} <command>",
        org
    ))?;

    Ok(())
}
