mod config;
mod coze;
mod git;

use clap::Parser;

#[derive(Parser)]
struct Args {}

#[tokio::main]
async fn main() {
    let _args = Args::parse();

    if let Some(config) = config::read_config() {
        // git exist
        if let Ok(git_base) = git::get_git_base_path() {
            let repo = git::get_reop(&git_base).unwrap();
            // git has changes
            if git::commit_or_not(&repo).is_ok() {
                let diff = git::read_git_diff().unwrap();
                // request bot for commit message
                let resp = coze::request_bot(&config.bot_id, &config.token, &diff)
                    .await
                    .expect("Request bot failed.");
                if let Ok(message) = coze::parse_commit_message(resp).await {
                    git::git_commit(&repo, &message).unwrap();
                }
            } else {
                println!("No changes to commit.");
            }
        } else {
            println!("Git project not found.");
        }
    } else {
        println!("Please set your config file first.");
        println!("Default path: ~/.gitc");
    }
}
