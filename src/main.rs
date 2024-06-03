mod config;
mod coze;

use anyhow::Result;
use clap::Parser;
use tokio;

#[derive(Parser)]
struct Args {}

fn read_git_diff() -> Result<String> {
    let status = std::process::Command::new("git")
        .arg("status")
        .output()
        .expect("failed to execute process");

    let diff = std::process::Command::new("git")
        .arg("diff")
        .arg("--cached")
        .output()
        .expect("failed to execute process");

    let status = String::from_utf8_lossy(&status.stdout);
    let diff = String::from_utf8_lossy(&diff.stdout);

    let res = format!("{}\n{}", status, diff);
    Ok(res)
}

#[tokio::main]
async fn main() {
    let _args = Args::parse();

    if let Some(config) = config::read_config() {
        let diff = read_git_diff().unwrap();
        let resp = coze::request_bot(&config.bot_id, &config.token, &diff).await;
        if let Ok(resp) = resp {
            let message = coze::parse_commit_message(resp).await.unwrap();
            println!("{}", message);
        }
    } else {
        println!("Please set your config file first.");
        println!("Default path: ~/.gitc");
    }
}
