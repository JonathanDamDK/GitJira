use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use regex::Regex;
use std::process::Command;

/// A CLI tool to format Git commit messages to match Jira conventions.
#[derive(Parser)]
#[command(name = "gj")]
#[command(about = "Jira-friendly Git commit tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Commit with a Jira-formatted message
    Commit {
        /// The commit message
        #[arg(short, long)]
        message: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Commit { message } => handle_commit(message),
    }
}

fn handle_commit(message: &str) -> Result<()> {
    // Step 1: Get the current branch name
    let branch = get_current_branch().context("Failed to get current Git branch")?;

    // Step 2: Extract Jira ticket from the branch name
    let jira_ticket = extract_jira_ticket(&branch)
        .ok_or_else(|| anyhow!("Failed to extract Jira ticket from branch name '{}'", branch))?;

    // Step 3: Format the commit message
    let formatted_message = format!("{} {}", jira_ticket, message);

    // Step 4: Execute the git commit command
    execute_git_commit(&formatted_message)?;

    println!("Commit message formatted to: {}", formatted_message);
    Ok(())
}


fn get_current_branch() -> Result<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .context("Failed to execute git rev-parse")?;

    if !output.status.success() {
        return Err(anyhow!(
            "git rev-parse failed with status: {}",
            output.status
        ));
    }

    let branch = String::from_utf8(output.stdout)
        .context("Failed to parse branch name as UTF-8")?
        .trim()
        .to_string();

    Ok(branch)
}

fn extract_jira_ticket(branch: &str) -> Option<String> {
    // Example branch name: NPCD-331-jonathan-incorporate-feedback-from-code-review
    // Regex to capture the Jira ticket (e.g., NPCD-331)
    let re = Regex::new(r"^([A-Z]+-\d+)").unwrap();
    re.captures(branch).and_then(|caps| caps.get(1).map(|m| m.as_str().to_string()))
}

fn execute_git_commit(message: &str) -> Result<()> {
    let status = Command::new("git")
        .args(["commit", "-m", message])
        .status()
        .context("Failed to execute git commit")?;

    if !status.success() {
        return Err(anyhow!("git commit failed with status: {}", status));
    }

    Ok(())
}
