mod commands;
mod config;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ssh-manager")]
#[command(about = "SSH connection manager with port forwarding", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new SSH connection
    Add,

    /// Remove an SSH connection (also `ssh-manager rm`)
    #[command(alias = "rm")]
    Remove {
        /// Connection slug to remove
        slug: Option<String>,
    },

    /// List all SSH connections (also `ssh-manager ls`)
    #[command(alias = "ls")]
    List,

    /// Edit an SSH connection
    Edit {
        /// Connection slug to edit
        slug: Option<String>,
    },

    /// Connect to an SSH server
    Connect {
        /// Connection slug
        slug: Option<String>,
    },

    /// Connect to an SFTP server
    #[command(name = "connect-sftp")]
    ConnectSftp {
        /// Connection slug
        slug: Option<String>,
    },

    /// Copy SSH public key to remote server (ssh-copy-id)
    #[command(name = "init-connect")]
    InitConnect {
        /// Connection slug
        slug: Option<String>,
    },

    /// Add port forwarding to a connection
    #[command(name = "forward-port")]
    ForwardPort {
        /// Connection slug
        slug: Option<String>,
    },

    /// Generate zsh completion script
    #[command(name = "completion-zsh")]
    CompletionZsh,

    /// Generate bash completion script
    #[command(name = "completion-bash")]
    CompletionBash,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Add => commands::add(),
        Commands::Remove { slug } => commands::remove(slug),
        Commands::List => commands::list(),
        Commands::Edit { slug } => commands::edit(slug),
        Commands::Connect { slug } => commands::connect(slug),
        Commands::ConnectSftp { slug } => commands::connect_sftp(slug),
        Commands::InitConnect { slug } => commands::init_connect(slug),
        Commands::ForwardPort { slug } => commands::forward_port(slug),
        Commands::CompletionZsh => commands::completion_zsh(),
        Commands::CompletionBash => commands::completion_bash(),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
