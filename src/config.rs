use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortMapping {
    pub local_port: u16,
    pub remote_port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub slug: String,
    pub address: String,
    #[serde(default = "default_port", skip_serializing_if = "is_default_port")]
    pub port: u16,
    pub user: String,
    pub ssh_key: String,
    #[serde(default)]
    pub port_mappings: Vec<PortMapping>,
}

fn default_port() -> u16 {
    22
}

fn is_default_port(port: &u16) -> bool {
    *port == 22
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub connections: Vec<Connection>,
}

impl Config {
    pub fn load() -> io::Result<Self> {
        let config_path = get_config_path()?;

        if !config_path.exists() {
            return Ok(Config::default());
        }

        let contents = fs::read_to_string(&config_path)?;
        let config: Config = serde_json::from_str(&contents)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(config)
    }

    pub fn save(&self) -> io::Result<()> {
        let config_path = get_config_path()?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let contents = serde_json::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        fs::write(&config_path, contents)?;
        Ok(())
    }

    pub fn find_connection(&self, slug: &str) -> Option<&Connection> {
        self.connections.iter().find(|c| c.slug == slug)
    }

    pub fn find_connection_mut(&mut self, slug: &str) -> Option<&mut Connection> {
        self.connections.iter_mut().find(|c| c.slug == slug)
    }

    pub fn add_connection(&mut self, connection: Connection) -> io::Result<()> {
        if self.find_connection(&connection.slug).is_some() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("Connection with slug '{}' already exists", connection.slug),
            ));
        }
        self.connections.push(connection);
        Ok(())
    }

    pub fn remove_connection(&mut self, slug: &str) -> io::Result<()> {
        let index = self
            .connections
            .iter()
            .position(|c| c.slug == slug)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("Connection '{}' not found", slug),
                )
            })?;

        self.connections.remove(index);
        Ok(())
    }

    pub fn update_connection(&mut self, old_slug: &str, connection: Connection) -> io::Result<()> {
        if old_slug != connection.slug && self.find_connection(&connection.slug).is_some() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!("Connection with slug '{}' already exists", connection.slug),
            ));
        }

        let conn = self.find_connection_mut(old_slug).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("Connection '{}' not found", old_slug),
            )
        })?;

        *conn = connection;
        Ok(())
    }
}

fn get_config_path() -> io::Result<PathBuf> {
    let config_dir = dirs::config_dir().ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "Could not find config directory")
    })?;

    Ok(config_dir.join("ssh-manager").join("config.json"))
}
