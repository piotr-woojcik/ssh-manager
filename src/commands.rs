use crate::config::{Config, Connection, PortMapping};
use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Editor, Helper};
use std::io::{self, Write};
use std::process::Command;

struct FilePathHelper {
    completer: FilenameCompleter,
}

impl FilePathHelper {
    fn new() -> Self {
        Self {
            completer: FilenameCompleter::new(),
        }
    }
}

impl Completer for FilePathHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>), ReadlineError> {
        self.completer.complete(line, pos, ctx)
    }
}

impl Hinter for FilePathHelper {
    type Hint = String;
}

impl Highlighter for FilePathHelper {}

impl Validator for FilePathHelper {}

impl Helper for FilePathHelper {}

fn prompt(text: &str) -> io::Result<String> {
    print!("{}", text);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn prompt_or_arg(arg: Option<String>, text: &str) -> io::Result<String> {
    if let Some(value) = arg {
        Ok(value)
    } else {
        prompt(text)
    }
}

fn prompt_with_completion(text: &str, default: Option<&str>) -> io::Result<String> {
    let mut rl = Editor::new().map_err(io::Error::other)?;
    rl.set_helper(Some(FilePathHelper::new()));

    let prompt_text = if let Some(def) = default {
        format!("{} [{}]: ", text, def)
    } else {
        format!("{}: ", text)
    };

    match rl.readline(&prompt_text) {
        Ok(line) => {
            let trimmed = line.trim();
            if trimmed.is_empty() && default.is_some() {
                if let Some(def) = default {
                    Ok(def.to_string())
                } else {
                    Ok(trimmed.to_string())
                }
            } else {
                Ok(trimmed.to_string())
            }
        }
        Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => Err(io::Error::new(
            io::ErrorKind::Interrupted,
            "Input cancelled",
        )),
        Err(e) => Err(io::Error::other(e)),
    }
}

pub fn add() -> io::Result<()> {
    let slug = prompt("Enter slug: ")?;
    let address = prompt("Enter address: ")?;

    let port_input = prompt("Enter port [22]: ")?;
    let port: u16 = if port_input.is_empty() {
        22
    } else {
        port_input
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid port number"))?
    };

    let user = prompt("Enter user: ")?;

    let ssh_key = loop {
        let key_path = prompt_with_completion("Enter SSH key path (e.g., ~/.ssh/id_rsa)", None)?;

        let expanded_path = if let Some(stripped) = key_path.strip_prefix("~/") {
            if let Some(home) = dirs::home_dir() {
                home.join(stripped).to_string_lossy().to_string()
            } else {
                key_path.clone()
            }
        } else {
            key_path.clone()
        };

        let path = std::path::Path::new(&expanded_path);
        if !path.exists() {
            println!("Error: SSH key file not found: {}", expanded_path);
            println!("Please enter a valid SSH key path.");
        } else if path.is_dir() {
            println!("Error: Path is a directory, not a file: {}", expanded_path);
            println!("Please enter a valid SSH key file path.");
        } else {
            break expanded_path;
        }
    };

    let connection = Connection {
        slug: slug.clone(),
        address,
        port,
        user,
        ssh_key,
        port_mappings: Vec::new(),
    };

    let mut config = Config::load()?;
    config.add_connection(connection)?;
    config.save()?;

    println!("Added connection: {}", slug);
    Ok(())
}

pub fn remove(slug: Option<String>) -> io::Result<()> {
    let slug = prompt_or_arg(slug, "Enter connection slug to remove: ")?;

    let mut config = Config::load()?;
    config.remove_connection(&slug)?;
    config.save()?;

    println!("Removed connection: {}", slug);
    Ok(())
}

pub fn list() -> io::Result<()> {
    let config = Config::load()?;

    for conn in &config.connections {
        let ports_str = if conn.port_mappings.is_empty() {
            "none".to_string()
        } else {
            conn.port_mappings
                .iter()
                .map(|pm| format!("{}:{}", pm.local_port, pm.remote_port))
                .collect::<Vec<_>>()
                .join(", ")
        };

        println!(
            "{}: {}@{}:{} (key: {}), Port Forwards: {}",
            conn.slug, conn.user, conn.address, conn.port, conn.ssh_key, ports_str
        );
    }

    Ok(())
}

pub fn edit(slug: Option<String>) -> io::Result<()> {
    let slug = prompt_or_arg(slug, "Enter connection slug to edit: ")?;

    let mut config = Config::load()?;
    let conn = config
        .find_connection(&slug)
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("Connection '{}' not found", slug),
            )
        })?
        .clone();

    let new_slug_input = prompt(&format!("Enter new slug (current: {}): ", conn.slug))?;
    let new_slug = if new_slug_input.is_empty() {
        conn.slug.clone()
    } else {
        new_slug_input
    };

    let new_address_input = prompt(&format!("Enter new address (current: {}): ", conn.address))?;
    let new_address = if new_address_input.is_empty() {
        conn.address.clone()
    } else {
        new_address_input
    };

    let new_port_input = prompt(&format!("Enter new port (current: {}): ", conn.port))?;
    let new_port = if new_port_input.is_empty() {
        conn.port
    } else {
        new_port_input
            .parse()
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid port number"))?
    };

    let new_user_input = prompt(&format!("Enter new user (current: {}): ", conn.user))?;
    let new_user = if new_user_input.is_empty() {
        conn.user.clone()
    } else {
        new_user_input
    };

    let new_ssh_key = loop {
        let key_path = prompt_with_completion("Enter new SSH key path", Some(&conn.ssh_key))?;

        let expanded_path = if let Some(stripped) = key_path.strip_prefix("~/") {
            if let Some(home) = dirs::home_dir() {
                home.join(stripped).to_string_lossy().to_string()
            } else {
                key_path.clone()
            }
        } else {
            key_path.clone()
        };

        let path = std::path::Path::new(&expanded_path);
        if !path.exists() {
            println!("Error: SSH key file not found: {}", expanded_path);
            println!("Please enter a valid SSH key path.");
        } else if path.is_dir() {
            println!("Error: Path is a directory, not a file: {}", expanded_path);
            println!("Please enter a valid SSH key file path.");
        } else {
            break expanded_path;
        }
    };

    let updated_conn = Connection {
        slug: new_slug.clone(),
        address: new_address,
        port: new_port,
        user: new_user,
        ssh_key: new_ssh_key,
        port_mappings: conn.port_mappings,
    };

    config.update_connection(&slug, updated_conn)?;
    config.save()?;

    println!("Updated connection: {}", new_slug);
    Ok(())
}

pub fn connect(slug: Option<String>) -> io::Result<()> {
    let slug = prompt_or_arg(slug, "Enter connection slug: ")?;

    let config = Config::load()?;
    let conn = config.find_connection(&slug).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("Connection '{}' not found", slug),
        )
    })?;

    let mut ssh_args = vec![
        "-i".to_string(),
        conn.ssh_key.clone(),
        "-p".to_string(),
        conn.port.to_string(),
    ];

    for pm in &conn.port_mappings {
        ssh_args.push("-L".to_string());
        ssh_args.push(format!("{}:localhost:{}", pm.local_port, pm.remote_port));
    }

    ssh_args.push(format!("{}@{}", conn.user, conn.address));

    let status = Command::new("ssh").args(&ssh_args).status()?;

    if !status.success() {
        return Err(io::Error::other("SSH command failed"));
    }

    Ok(())
}

pub fn connect_sftp(slug: Option<String>) -> io::Result<()> {
    let slug = prompt_or_arg(slug, "Enter connection slug: ")?;

    let config = Config::load()?;
    let conn = config.find_connection(&slug).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("Connection '{}' not found", slug),
        )
    })?;

    let sftp_args = vec![
        "-i".to_string(),
        conn.ssh_key.clone(),
        "-P".to_string(),
        conn.port.to_string(),
        format!("{}@{}", conn.user, conn.address),
    ];

    let status = Command::new("sftp").args(&sftp_args).status()?;

    if !status.success() {
        return Err(io::Error::other("SFTP command failed"));
    }

    Ok(())
}

pub fn init_connect(slug: Option<String>) -> io::Result<()> {
    let slug = prompt_or_arg(slug, "Enter connection slug: ")?;

    let config = Config::load()?;
    let conn = config.find_connection(&slug).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("Connection '{}' not found", slug),
        )
    })?;

    let ssh_copy_id_args = vec![
        "-i".to_string(),
        conn.ssh_key.clone(),
        "-p".to_string(),
        conn.port.to_string(),
        format!("{}@{}", conn.user, conn.address),
    ];

    let status = Command::new("ssh-copy-id")
        .args(&ssh_copy_id_args)
        .status()?;

    if !status.success() {
        return Err(io::Error::other("ssh-copy-id command failed"));
    }

    println!(
        "SSH key successfully copied to {}@{}",
        conn.user, conn.address
    );
    Ok(())
}

pub fn forward_port(slug: Option<String>) -> io::Result<()> {
    let slug = prompt_or_arg(slug, "Enter connection slug: ")?;

    let mut config = Config::load()?;

    if config.find_connection(&slug).is_none() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Connection '{}' not found", slug),
        ));
    }

    let local_port_str = prompt("Enter local port: ")?;
    let remote_port_str = prompt("Enter remote port: ")?;

    let local_port: u16 = local_port_str
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid local port"))?;

    let remote_port: u16 = remote_port_str
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Invalid remote port"))?;

    let port_mapping = PortMapping {
        local_port,
        remote_port,
    };

    let conn = config.find_connection_mut(&slug).unwrap();
    conn.port_mappings.push(port_mapping);

    config.save()?;

    println!(
        "Added port forwarding for {}: {} -> {}",
        slug, local_port, remote_port
    );
    Ok(())
}

pub fn completion_zsh() -> io::Result<()> {
    use clap::CommandFactory;
    use clap_complete::{generate, shells::Zsh};

    let mut cmd = crate::Cli::command();
    generate(Zsh, &mut cmd, "ssh-manager", &mut io::stdout());
    Ok(())
}

pub fn completion_bash() -> io::Result<()> {
    use clap::CommandFactory;
    use clap_complete::{generate, shells::Bash};

    let mut cmd = crate::Cli::command();
    generate(Bash, &mut cmd, "ssh-manager", &mut io::stdout());
    Ok(())
}
