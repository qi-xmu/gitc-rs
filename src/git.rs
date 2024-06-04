use std::io::Write;

use anyhow::Result;
use git2::Repository;

pub fn get_reop(path: &str) -> Result<Repository> {
    let repo = Repository::open(path)?;
    Ok(repo)
}
pub fn commit_or_not(repo: &Repository) -> Result<()> {
    let statuses = repo.statuses(None)?;

    let commit_or_not = statuses.iter().any(|status| {
        status.status().is_index_new()
            || status.status().is_index_modified()
            || status.status().is_index_deleted()
    });

    if commit_or_not {
        Ok(())
    } else {
        Err(anyhow::anyhow!("No changes to commit"))
    }
}

pub fn get_input_char() -> String {
    print!("Do you want to commit? [y/r/n]");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
pub fn git_add(repo: &Repository) -> Result<()> {
    let mut index = repo.index()?;
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;
    Ok(())
}
pub fn git_commit(repo: &Repository, message: &str) -> Result<()> {
    let mut index = repo.index()?;
    let sig = repo.signature()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    let parent_commit = repo.head()?.resolve()?.peel_to_commit()?;

    // commit
    repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &[&parent_commit])?;
    Ok(())
}

pub fn git_push(_repo: &Repository) -> Result<()> {
    let output = std::process::Command::new("git")
        .arg("push")
        .output()
        .expect("failed to execute process");

    std::io::stdout().write_all(&output.stdout)?;
    std::io::stderr().write_all(&output.stderr)?;
    Ok(())
}

pub fn git_diff_cached(has_status: bool) -> Result<String> {
    let status = if has_status {
        let status = std::process::Command::new("git")
            .arg("status")
            .output()
            .expect("failed to execute process");
        String::from_utf8_lossy(&status.stdout).to_string() + "\n"
    } else {
        "\n".to_string()
    };

    let diff = std::process::Command::new("git")
        .arg("diff")
        .arg("--cached")
        .output()
        .expect("failed to execute process");
    let diff = String::from_utf8_lossy(&diff.stdout);
    let res = status + &diff;
    Ok(res)
}

pub fn git_base_path() -> Result<String> {
    let basepath = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()?;
    let basepath = String::from_utf8_lossy(&basepath.stdout);
    let path_str = basepath.trim().to_string();
    let path = std::path::Path::new(&path_str);

    if path.exists() {
        Ok(path_str)
    } else {
        Err(anyhow::anyhow!("Path not exists"))
    }
}

#[test]
fn test_git_commit() {
    let git_base = git_base_path().unwrap();
    let message = "test commit";
    let repo = get_reop(&git_base).unwrap();
    git_commit(&repo, message).unwrap();
}
