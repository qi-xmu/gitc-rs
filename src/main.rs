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
        let diff = git::read_git_diff().unwrap();
        let resp = coze::request_bot(&config.bot_id, &config.token, &diff).await;
        if let Ok(resp) = resp {
            let message = coze::parse_commit_message(resp).await.unwrap();
            println!("{}", message);

            // commit
            if let Ok(git_base) = git::get_git_base_path() {
                git::git_commit(&git_base, &message).unwrap();
            } else {
                println!("Git project not found.");
            }
        }
    } else {
        println!("Please set your config file first.");
        println!("Default path: ~/.gitc");
    }
}
