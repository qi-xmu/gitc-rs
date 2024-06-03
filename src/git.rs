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

pub fn git_commit(repo: &Repository, message: &str) -> Result<()> {
    let statuses = repo.statuses(None)?;
    let commit_or_not = statuses.iter().any(|status| {
        status.status().is_index_new()
            || status.status().is_index_modified()
            || status.status().is_index_deleted()
    });

    if commit_or_not {
        let mut index = repo.index()?;
        let sig = repo.signature()?;
        let tree_id = index.write_tree()?;
        let tree = repo.find_tree(tree_id)?;
        let parent_commit = repo.head()?.resolve()?.peel_to_commit()?;
        println!("Parent commit: {}", parent_commit.summary().unwrap_or(""));

        repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &[&parent_commit])?;
        println!("Commit success");
    } else {
        println!("No changes to commit");
    }

    Ok(())
}

pub fn read_git_diff() -> Result<String> {
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

pub fn get_git_base_path() -> Result<String> {
    let output = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output()?;

    let basepath = String::from_utf8_lossy(&output.stdout);
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
    let git_base = get_git_base_path().unwrap();
    let message = "test commit";
    let repo = get_reop(&git_base).unwrap();
    git_commit(&repo, message).unwrap();
}
