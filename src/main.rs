use clap::Parser as _;

mod args;
mod config;
mod coze;
mod git;

#[tokio::main]
async fn main() {
    let args = args::Args::parse();
    // read config
    let config =
        config::read_config().expect("Please set your config file first. Default path: ~/.gitc");
    // git exist
    let git_base = git::git_base_path().expect("Git project not found.");
    let repo = git::get_reop(&git_base).expect("Git repo not found.");

    // check if git add
    if args.add_all {
        git::git_add(&repo).expect("Git add failed.");
        println!("Git add success.");
    }

    // git has changes
    if git::commit_or_not(&repo).is_ok() {
        let diff = git::git_diff_cached(false).unwrap();
        // request bot for commit message
        let mut message = coze::coze_commit_message(&config, &diff)
            .await
            .expect("Request bot failed.");
        // 询问是否提交

        while !args.yes {
            println!("* Commit message: \n{}\n", message);
            let ch = git::get_input_char();
            if ch == "y" {
                git::git_commit(&repo, &message).expect("Commit failed.");
                break;
            } else if ch == "r" {
                message = coze::coze_commit_message(&config, &diff)
                    .await
                    .expect("Request bot failed.");
                continue;
            } else if ch == "n" {
                println!("Commit canceled.");
                return;
            }
        }
    } else {
        println!("No changes to commit.");
    }
}
