use commit_analysis::CommitAnalysis;
use console::style;
use git2::Repository;
use std::env;

mod commit_analysis;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!(
            "{}\n{}\n{}",
            style("Error: Missing required arguments").red(),
            style("Usage: ur-commit-mentor <repository_path> <claude_api_key>").yellow(),
            style("Example: ur-commit-mentor ./my-repo CLAUDE_API_KEY").dim()
        );
        std::process::exit(1);
    }

    let current_dir = env::current_dir()?;
    let repo_path = current_dir.join(&args[1]);
    let repo_path = repo_path.canonicalize().map_err(|e| {
        eprintln!(
            "{}: {}\nPath: {}\nTried resolving: {}",
            style("Error resolving repository path").red(),
            e,
            args[1],
            repo_path.display()
        );
        e
    })?;

    let claude_api_key = &args[2];

    println!("{}", style("🔍 Analyzing Git Repository...").bold().cyan());

    let mut analysis = CommitAnalysis::new(claude_api_key.to_string());

    let repo = match Repository::open(&repo_path) {
        Ok(repo) => repo,
        Err(e) => {
            eprintln!(
                "{}: {}\nPath: {}",
                style("Error opening repository").red(),
                e,
                repo_path.display()
            );
            return Err(e.into());
        }
    };

    let mut revwalk = repo.revwalk()?;
    revwalk.push_head()?;
    revwalk.set_sorting(git2::Sort::TIME)?;

    println!("{}", style("📅 Loading commit history...").dim());

    for commit_id in revwalk {
        let commit = repo.find_commit(commit_id?)?;
        let parent = commit.parent(0).ok();
        let tree = commit.tree()?;
        let parent_tree = parent.as_ref().and_then(|c| c.tree().ok());

        let diff = repo.diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), None)?;

        analysis.add_commit(
            commit.id().to_string(),
            commit.time().seconds(),
            commit.message().unwrap_or("").to_string(),
            commit.author().name().unwrap_or("Unknown").to_string(),
            &diff,
        )?;
    }

    // Display and select commits
    analysis.display_commits();

    if let Some(selected_index) = analysis.select_commit() {
        analysis.analyze_selected_commit(selected_index).await?;
    } else {
        println!("{}", style("No commit selected.").yellow());
    }

    Ok(())
}
