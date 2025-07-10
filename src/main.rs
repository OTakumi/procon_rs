use clap::Parser;
use colored::*;
use procon_rs::cli::{Cli, Commands};
use procon_rs::commands::new::{NewCommand, NewCommandArgs};

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::New {
            name,
            template,
            path,
        } => {
            println!(
                "{} Creating project '{}'...",
                "✨".bright_yellow(),
                name.bright_cyan()
            );

            let args = NewCommandArgs {
                name: name.clone(),
                template,
                path,
            };

            match NewCommand::execute(args) {
                Ok(()) => {
                    println!(
                        "{} Project '{}' created successfully!",
                        "✅".bright_green(),
                        name.bright_cyan()
                    );
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }

        Commands::Init { .. } => {
            println!("{} Init command not yet implemented", "⚠️".bright_yellow());
            Ok(())
        }

        Commands::Config { key, value } => {
            match value {
                Some(val) => println!(
                    "{} Set {} = {}",
                    "⚙️".bright_blue(),
                    key.bright_cyan(),
                    val.bright_green()
                ),
                None => println!(
                    "{} Get {} (not implemented)",
                    "⚙️".bright_blue(),
                    key.bright_cyan()
                ),
            }
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("{} {}", "❌".bright_red(), e.to_string().bright_red());
        std::process::exit(1);
    }
}
