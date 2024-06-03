mod config;
mod coze;

use anyhow::Result;
use clap::Parser;
use tokio;

#[derive(Parser)]
struct Args {}

// const default_config_path

const TOKEN: &str = "pat_FufCiN8D7bi0UWTFqcTlvSTG5uqIniAUFcRScauKyTWsSr9KT0pqFqsg4TzG2Xtj";
const BOT_ID: &str = "7376178220695650356";

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

    // let query = "123456";

    let diff = read_git_diff().unwrap();

    // let resp = coze::request_bot(BOT_ID, TOKEN, query).await;
    // if let Ok(resp) = resp {
    //     let message = coze::parse_commit_message(resp).await.unwrap();
    //     println!("{}", message);
    // }
}
