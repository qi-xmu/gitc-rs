mod config;
mod coze;
mod git;

use clap::Parser;

#[derive(Parser)]
struct Args {}

#[tokio::main]
async fn main() {
    let _args = Args::parse();
    // read config
    let config =
        config::read_config().expect("Please set your config file first. Default path: ~/.gitc");
    // git exist
    let git_base = git::get_git_base_path().expect("Git project not found.");
    let repo = git::get_reop(&git_base).expect("Git repo not found.");
    // git has changes
    if git::commit_or_not(&repo).is_ok() {
        let diff = git::read_git_diff().unwrap();
        // request bot for commit message
        let resp = coze::request_bot(&config.bot_id, &config.token, &diff)
            .await
            .expect("Request bot failed.");
        if let Ok(message) = coze::parse_commit_message(resp).await {
            // 询问是否提交
            loop {
                let mut input = String::new();
                println!("Commit message: {}", message);
                print!("Do you want to commit? (y/n) ");
                std::io::stdin().read_line(&mut input).unwrap();
                if input.trim() == "y" {
                    git::git_commit(&repo, &message).unwrap();
                    println!("Commit message: {}", message);
                } else if input.trim() == "n" {
                    println!("Commit canceled.");
                }
            }
        } else {
            println!("Empty message. Maybe config is wrong.");
        }
    } else {
        println!("No changes to commit.");
    }
}
