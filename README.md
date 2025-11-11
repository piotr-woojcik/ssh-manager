# SSH Manager

A command-line tool to simplify managing SSH connections with SFTP and port forwarding.

## Features

- Store and manage SSH connections with easy to remember slugs
- Support for custom SSH ports (default: 22)
- Port forwarding management
- Interactive command prompts with tab completion for file paths
- Zsh and Bash shell completion
- JSON-based configuration storage

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/piotr-woojcik/ssh-manager.git
cd ssh-manager

# Build and install
cargo build --release
sudo cp target/release/ssh-manager /usr/local/bin/ssh-manager

# For development and testing
cargo build --release
# Binary will be at: target/release/ssh-manager
```

### Shell Completion

CLI has commands to generate zsh and bash completion scripts:
```bash
ssh-manager completion-zsh
ssh-manager completion-bash
```

## Usage

### Adding a Connection
```bash
ssh-manager add
# Follow the interactive prompts:
# - Enter slug (e.g., 'prod-server')
# - Enter address (e.g., 'example.com')
# - Enter port [22] (press Enter for default, or e.g., '2222')
# - Enter user (e.g., 'admin')
# - Enter SSH key path (e.g., '~/.ssh/id_rsa') - supports tab completion!
```

### Initializing Connection (Copy SSH Key)
```bash
ssh-manager init-connect [slug]
# Copies your SSH key to the remote server (ssh-copy-id)
```

### Adding Port Forwarding
```bash
ssh-manager forward-port [slug]
# Enter local and remote ports when prompted
```

### Listing Connections
```bash
ssh-manager list
# or
ssh-manager ls
```

### Connecting
```bash
ssh-manager connect [slug]        # SSH connection with port forwarding
ssh-manager connect-sftp [slug]   # SFTP connection
```

### Editing a Connection
```bash
ssh-manager edit [slug]
# Modify slug, address, port, user, or SSH key path
```

### Deleting a Connection
```bash
ssh-manager delete [slug]
```

## Configuration

All data is stored in `~/.config/ssh-manager/`:
- JSON configuration: `~/.config/ssh-manager/config.json`

### Configuration Format

```json
{
  "connections": [
    {
      "slug": "my-server",
      "address": "example.com",
      "port": 2222,
      "user": "admin",
      "ssh_key": "/home/user/.ssh/id_rsa",
      "port_mappings": [
        {
          "local_port": 8080,
          "remote_port": 80
        }
      ]
    }
  ]
}
```

**Note**: Port field is only saved if it's not the default (22).

## Testing

A Docker Compose test environment is provided:

```bash
# Start test SSH servers
docker compose up -d

# Test with localhost:2222 (testuser/testpass) and localhost:2223 (altuser/testpass2)

# Stop test servers
docker compose down
```

## License

[MIT License](LICENSE)
