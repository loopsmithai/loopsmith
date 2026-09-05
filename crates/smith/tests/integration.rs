use rexpect::spawn_bash;

/// Smoke-tests the `smith init` wizard against a local Gitea instance.
///
/// Requires a running Gitea on `localhost:3000` and a token for it in
/// `GITEA_TOKEN`. Skipped when that is unset, so the suite stays green on a
/// machine without the fixture. See `demo.sh`.
#[test]
fn test_smith_init_wizard_smoke() {
    let Ok(tok) = std::env::var("GITEA_TOKEN") else {
        eprintln!("skipping: GITEA_TOKEN not set (needs a local Gitea on :3000)");
        return;
    };
    let bin = env!("CARGO_BIN_EXE_smith");

    let mut p = spawn_bash(Some(60_000)).unwrap();

    p.send_line(&format!(
        "NO_COLOR=1 SMITH_GITHUB_API_BASE=http://localhost:3000/api/v1 SMITH_GITHUB_WEB_BASE=http://localhost:3000 {} init",
        bin
    )).unwrap();

    // Auth: should prompt for PAT (custom API base skips gh detection)
    p.exp_string("Personal Access Token").unwrap();
    p.send_line(&tok).unwrap();
    p.exp_string("Authenticated as").unwrap();

    // Org selection
    p.exp_string("Select GitHub organization").unwrap();
    p.send_line("").unwrap();

    // Repo selection
    p.exp_string("Select home repository").unwrap();
    p.send_line("").unwrap();

    // App name
    p.exp_string("GitHub App name").unwrap();
    p.send_line("").unwrap();

    // Should start manifest flow
    p.exp_string("Click this link").unwrap();

    drop(p);
}
