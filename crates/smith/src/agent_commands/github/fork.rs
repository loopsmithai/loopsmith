use anyhow::Result;

use super::gh_runner;

pub fn fork_repo(source: &str, org: &str) -> Result<()> {
    let output = gh_runner::gh(&[
        "repo", "fork", source,
        "--org", org, "--clone=false",
    ])?;
    eprintln!("{}", output.trim());

    // Fetch the fork details
    let source_repo = source.split('/').last().unwrap_or(source);
    let fork_name = format!("{}/{}", org, source_repo);
    let details = gh_runner::gh(&[
        "repo", "view", &fork_name,
        "--json", "fullName,url,sshUrl",
    ])?;
    println!("{}", details);
    Ok(())
}
