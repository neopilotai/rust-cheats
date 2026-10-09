use anyhow::{Context, Result};
use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use clap_complete::{generate, Shell};
use cheats_core::SearchRequest;
use std::{io, path::PathBuf};

#[derive(Debug, Parser)]
#[command(name = "rust-cheats", version, about = "Offline-first programming cheat-sheet engine")]
struct Cli {
    #[arg(long, global = true, env = "RUST_CHEATS_DATA_DIR", default_value = "data")]
    data_dir: PathBuf,
    #[arg(long, global = true, help = "Suppress nonessential output")]
    quiet: bool,
    #[arg(long, global = true, help = "Privacy mode: quiet output and no persistent query history")]
    private: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Search {
        query: String,
        #[arg(long)]
        language: Option<String>,
        #[arg(long, default_value_t = 10)]
        limit: usize,
    },
    Languages,
    Show { id: String },
    Complete { prefix: String, #[arg(long)] language: Option<String> },
    Ask { query: String },
    Completions { shell: ShellArg },
    Data {
        #[command(subcommand)]
        command: DataCommands,
    },
    Lsp,
    Tui,
}

#[derive(Debug, Subcommand)]
enum DataCommands {
    Validate,
    RebuildIndex,
}

#[derive(Debug, Clone, ValueEnum)]
enum ShellArg { Bash, Zsh, Fish, Powershell, Elvish }

fn main() -> Result<()> {
    let cli = Cli::parse();
    let quiet = cli.quiet || cli.private;
    match cli.command {
        Commands::Completions { shell } => {
            let mut cmd = Cli::command();
            let shell = match shell {
                ShellArg::Bash => Shell::Bash, ShellArg::Zsh => Shell::Zsh,
                ShellArg::Fish => Shell::Fish, ShellArg::Powershell => Shell::PowerShell,
                ShellArg::Elvish => Shell::Elvish,
            };
            generate(shell, &mut cmd, "rust-cheats", &mut io::stdout());
        }
        Commands::Lsp => {
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(cheats_lsp::run(cli.data_dir))?;
        }
        Commands::Tui => {
            println!("TUI starter: interactive interface is scheduled for Phase 2.");
            println!("Try `rust-cheats search <query>` for now.");
        }
        command => {
            let sheets = cheats_data::load_dir(&cli.data_dir)
                .with_context(|| format!("failed to load sheets from {}", cli.data_dir.display()))?;
            match command {
                Commands::Search { query, language, limit } => {
                    let results = cheats_search::search(&sheets, &SearchRequest { query, language, limit });
                    if results.is_empty() {
                        if !quiet { println!("No matching cheat sheets found."); }
                    } else {
                        for result in results {
                            println!("{}\t[{}] {}\n  {}", result.id, result.language, result.title, result.description);
                        }
                    }
                }
                Commands::Languages => {
                    for lang in cheats_language::builtins() {
                        println!("{:<12} {} ({})", lang.id, lang.display_name, lang.extensions.join(", "));
                    }
                }
                Commands::Show { id } => {
                    let sheet = sheets.iter().find(|s| s.id == id)
                        .with_context(|| format!("unknown sheet id: {id}"))?;
                    println!("# {}\n{}\nLanguage: {}\nCategory: {}\nTags: {}\n",
                        sheet.title, sheet.description, sheet.language, sheet.category, sheet.tags.join(", "));
                    for example in &sheet.examples {
                        println!("## {}\n```{}\n{}\n```\n{}",
                            example.title, example.language, example.code,
                            example.description.as_deref().unwrap_or(""));
                    }
                }
                Commands::Complete { prefix, language } => {
                    for item in cheats_complete::complete(&sheets, &prefix, language.as_deref(), 20) {
                        println!("{}\t{}", item.label, item.detail.unwrap_or_default());
                    }
                }
                Commands::Ask { query } => {
                    let parsed = cheats_intent::parse(&query);
                    let results = cheats_search::search(&sheets, &SearchRequest {
                        query: parsed.query, language: parsed.language, limit: 5,
                    });
                    println!("Intent: {:?}", parsed.intent);
                    for result in results {
                        println!("{}\t[{}] {}", result.id, result.language, result.title);
                    }
                }
                Commands::Data { command: DataCommands::Validate } => {
                    println!("Validated {} cheat sheets.", sheets.len());
                }
                Commands::Data { command: DataCommands::RebuildIndex } => {
                    println!("Loaded {} cheat sheets; search uses the deterministic in-memory index.", sheets.len());
                }
                Commands::Completions { .. } | Commands::Lsp | Commands::Tui => unreachable!(),
            }
        }
    }
    Ok(())
}
