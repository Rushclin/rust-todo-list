mod commands;
mod storage;
mod todo;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "todo",
    about = "📝 A command-line task manager",
    version = "1.0",
    author = "Rushclin Takam 🚀"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Add {
        title: String,
    },

    List {
        #[arg(short, long)]
        all: bool,
    },

    Done {
        id: u32,
    },

    Remove {
        id: u32,
    },

    Clear,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Add { title } => commands::add(title),
        Command::List { all } => commands::list(all),
        Command::Done { id } => commands::complete(id),
        Command::Remove { id } => commands::remove(id),
        Command::Clear => commands::clear_done(),
    }
}
