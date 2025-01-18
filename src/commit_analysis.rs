use chrono::{DateTime, Utc};
use console::{style, Term};
use dialoguer::FuzzySelect;
use serde_json::json;
use std::collections::HashMap;
use std::sync::mpsc;
use termimad::*;

#[derive(Debug)]
pub struct CommitAnalysis {
    claude_api_key: String,
    commits: Vec<CommitInfo>,
}

#[derive(Debug, Clone)]
struct CommitInfo {
    id: String,
    timestamp: i64,
    message: String,
    author: String,
    code_changes: Vec<FileChange>,
}

#[derive(Debug, Clone)]
struct FileChange {
    file_path: String,
    additions: Vec<String>,
    deletions: Vec<String>,
    language: String,
}

impl CommitAnalysis {
    pub fn new(api_key: String) -> Self {
        CommitAnalysis {
            claude_api_key: api_key,
            commits: Vec::new(),
        }
    }

    pub fn add_commit(
        &mut self,
        id: String,
        timestamp: i64,
        message: String,
        author: String,
        diff: &git2::Diff,
    ) -> Result<(), git2::Error> {
        let mut changes = Vec::new();
        let mut current_path = String::new();
        let (tx, rx) = mpsc::channel();
        let tx_clone = tx.clone();

        // Process file changes
        diff.foreach(
            &mut |delta, _| {
                if let Some(path) = delta.new_file().path() {
                    if !current_path.is_empty() {
                        let received: Vec<(bool, String)> = rx.try_iter().collect();
                        let (mut additions, mut deletions) = (Vec::new(), Vec::new());

                        for (is_addition, content) in received {
                            if is_addition {
                                additions.push(content);
                            } else {
                                deletions.push(content);
                            }
                        }

                        changes.push(FileChange {
                            file_path: std::mem::take(&mut current_path),
                            additions,
                            deletions,
                            language: detect_language(path.to_string_lossy().as_ref()),
                        });
                    }
                    current_path = path.to_string_lossy().to_string();
                }
                true
            },
            None,
            None,
            Some(&mut |_, _, line| {
                let content = String::from_utf8_lossy(line.content()).into_owned();
                let _ = tx_clone.send((line.origin() == '+', content));
                true
            }),
        )?;

        // Process remaining changes for the last file
        if !current_path.is_empty() {
            let received: Vec<(bool, String)> = rx.try_iter().collect();
            let (mut additions, mut deletions) = (Vec::new(), Vec::new());

            for (is_addition, content) in received {
                if is_addition {
                    additions.push(content);
                } else {
                    deletions.push(content);
                }
            }

            changes.push(FileChange {
                file_path: current_path.clone(),
                additions,
                deletions,
                language: detect_language(&current_path),
            });
        }

        self.commits.push(CommitInfo {
            id,
            timestamp,
            message,
            author,
            code_changes: changes,
        });

        Ok(())
    }

    pub fn display_commits(&self) {
        let term = Term::stdout();
        let _ = term.clear_screen();

        // Modern header with version
        println!(
            "\n{}\n{}\n",
            style(" 🔍 Commit History ").on_cyan().bold().black(),
            style("Find and analyze your git commits").dim(),
        );

        // Clean list of commits
        for (i, commit) in self.commits.iter().enumerate() {
            let date = DateTime::<Utc>::from_timestamp(commit.timestamp, 0)
                .unwrap()
                .format("%Y-%m-%d %H:%M");

            let message_first_line = commit.message.lines().next().unwrap_or("").trim();
            let message_preview = if message_first_line.len() > 50 {
                format!("{}...", &message_first_line[..47])
            } else {
                message_first_line.to_string()
            };

            let stats = commit.code_changes.iter().fold((0, 0), |acc, change| {
                (
                    acc.0 + change.additions.len(),
                    acc.1 + change.deletions.len(),
                )
            });

            println!(
                "{} {}\n   {} {} {} {}\n   {} {}\n",
                style(format!("#{}", i + 1)).bold().green(),
                style(&message_preview).white().bold(),
                style("👤").dim(),
                style(&commit.author).blue(),
                style("•").dim(),
                style(date).yellow(),
                style("📊").dim(),
                style(format!(
                    "{} files  {}  {}",
                    commit.code_changes.len(),
                    style(format!("+{}", stats.0)).green(),
                    style(format!("-{}", stats.1)).red(),
                ))
            );
        }

        println!(
            "\n{}\n{}\n",
            style("Type to search, ↑↓ to navigate").dim(),
            style("Press Enter to analyze commit").dim()
        );
    }

    pub fn select_commit(&self) -> Option<usize> {
        let commit_options: Vec<String> = self
            .commits
            .iter()
            .map(|c| {
                let date = DateTime::<Utc>::from_timestamp(c.timestamp, 0)
                    .unwrap()
                    .format("%Y-%m-%d %H:%M");

                let stats = c.code_changes.iter().fold((0, 0), |acc, change| {
                    (
                        acc.0 + change.additions.len(),
                        acc.1 + change.deletions.len(),
                    )
                });

                format!(
                    "{} │ {} │ {} │ +{} -{} │ {}",
                    style(date).yellow(),
                    style(&c.author).blue(),
                    style(format!("{} files", c.code_changes.len())).cyan(),
                    style(stats.0).green(),
                    style(stats.1).red(),
                    style(c.message.lines().next().unwrap_or("").trim()).white()
                )
            })
            .collect();

        if commit_options.is_empty() {
            return None;
        }

        FuzzySelect::new()
            .with_prompt("Select a commit to analyze")
            .default(0)
            .items(&commit_options)
            .interact()
            .ok()
    }

    pub async fn analyze_selected_commit(
        &self,
        index: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let commit = &self.commits[index];
        let term = Term::stdout();
        let _ = term.clear_screen();

        // Elegant header with commit info
        println!(
            "\n{}\n\n{}\n\n{}\n{}\n",
            style(" 🔍 Commit Analysis ").on_cyan().bold().black(),
            style(&format!("  {}", &commit.message))
                .white()
                .bold()
                .on_black()
                .underlined(),
            style("Commit Details").bold().cyan(),
            style("─".repeat(50)).dim(),
        );

        // Author and date in a clean line
        println!(
            "{}\n\n",
            format!(
                "{}  {}  {}  {}",
                style("👤").dim(),
                style(&commit.author).blue().bold(),
                style("•").dim(),
                style(
                    DateTime::<Utc>::from_timestamp(commit.timestamp, 0)
                        .unwrap()
                        .format("%Y-%m-%d %H:%M")
                )
                .yellow()
            ),
        );

        // Stats with better spacing
        let (total_additions, total_deletions) =
            commit.code_changes.iter().fold((0, 0), |acc, change| {
                (
                    acc.0 + change.additions.len(),
                    acc.1 + change.deletions.len(),
                )
            });

        println!(
            "{}  {}  {}\n",
            format!(
                "📁 {}",
                style(format!("{} files", commit.code_changes.len())).cyan()
            ),
            format!("✨ {}", style(format!("+{}", total_additions)).green()),
            format!("📝 {}", style(format!("-{}", total_deletions)).red()),
        );

        // Language breakdown with percentage
        let mut language_stats: HashMap<String, (usize, usize)> = HashMap::new();
        for change in &commit.code_changes {
            let entry = language_stats
                .entry(change.language.clone())
                .or_insert((0, 0));
            entry.0 += change.additions.len();
            entry.1 += change.deletions.len();
        }

        if !language_stats.is_empty() {
            println!("\n{}\n", style("📊 Language Breakdown").bold());

            for (lang, (adds, dels)) in language_stats {
                let total = adds + dels;
                let add_ratio = adds as f32 / total as f32;
                let bars = 30;
                let add_bars = (add_ratio * bars as f32) as usize;
                let del_bars = bars - add_bars;
                let percentage = (add_ratio * 100.0) as usize;

                println!(
                    "{}  {:<12} {}{}  {}",
                    style("│").dim(),
                    style(lang).bold(),
                    style("▇".repeat(add_bars)).green().dim(),
                    style("▇".repeat(del_bars)).red().dim(),
                    style(format!("{}% additions", percentage)).dim(),
                );
            }
            println!();
        }

        // AI Analysis section
        println!(
            "\n{}\n{}\n",
            style("🤖 AI Analysis").bold().magenta(),
            style("─".repeat(50)).dim()
        );

        match self.get_ai_code_review(commit).await {
            Ok(analysis) => {
                let mut skin = MadSkin::default();
                skin.set_headers_fg(crossterm::style::Color::Magenta);
                skin.bold.set_fg(crossterm::style::Color::Yellow);
                skin.italic.set_fg(crossterm::style::Color::Blue);
                skin.code_block.set_fg(crossterm::style::Color::Green);
                skin.inline_code.set_fg(crossterm::style::Color::Cyan);
                skin.print_text(&analysis);
            }
            Err(_) => {
                println!(
                    "\n{}\n{}\n",
                    style("⚠️  AI analysis unavailable").yellow(),
                    style("Check your API key and connection").dim(),
                );
            }
        }

        println!("\n{}", style("─".repeat(50)).dim());
        Ok(())
    }

    async fn get_ai_code_review(
        &self,
        commit: &CommitInfo,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let code_changes = commit
            .code_changes
            .iter()
            .map(|change| {
                format!(
                    "## File: {} ({})\n\n```{}\n// Removed:\n{}\n\n// Added:\n{}\n```\n",
                    change.file_path,
                    change.language,
                    change.language.to_lowercase(),
                    change.deletions.join("\n"),
                    change.additions.join("\n")
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        let prompt = format!(
            "Analyze the following code changes from commit: '{}'\n\n\
            Please provide a code review in markdown format. Focus on:\n\n\
            1. Summary of Changes\n\
               - Brief overview of what changed\n\
               - Impact and purpose\n\n\
            2. Code Analysis (for each file):\n\
               - Key improvements or concerns\n\
               - Potential bugs\n\
               - Performance implications\n\n\
            3. Recommendations:\n\
               - Specific code improvements (with examples in markdown code blocks)\n\
               - Best practices for the used languages\n\
               - Testing and documentation needs\n\n\
            Format your response in markdown with:\n\
            - Clear headings (##, ###)\n\
            - Code blocks with language tags (```language)\n\
            - Bullet points for lists\n\
            - Bold and italic for emphasis\n\n\
            Here are the changes:\n\n{}",
            commit.message, code_changes
        );

        let client = reqwest::Client::new();
        let response = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.claude_api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&json!({
                "model": "claude-3-sonnet-20240229",
                "max_tokens": 1000,
                "messages": [{
                    "role": "user",
                    "content": prompt
                }]
            }))
            .send()
            .await?;

        let response_data: serde_json::Value = response.json().await?;
        let ai_insight = response_data["content"][0]["text"]
            .as_str()
            .unwrap_or("Unable to get AI insights")
            .to_string();

        Ok(ai_insight)
    }
}

fn detect_language(file_path: &str) -> String {
    match file_path.split('.').last() {
        Some("js") => "JavaScript",
        Some("ts") => "TypeScript",
        Some("py") => "Python",
        Some("rs") => "Rust",
        Some("go") => "Go",
        Some("java") => "Java",
        Some("cpp" | "cc" | "cxx") => "C++",
        Some("c") => "C",
        Some("html") => "HTML",
        Some("css") => "CSS",
        Some("php") => "PHP",
        Some("rb") => "Ruby",
        Some("swift") => "Swift",
        Some("kt") => "Kotlin",
        _ => "Unknown",
    }
    .to_string()
}
