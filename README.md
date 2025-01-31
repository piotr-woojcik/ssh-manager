# SSH Manager

A command-line tool for managing SSH connections and port forwarding written in Go.

## Features

- Store and manage SSH connections with easy-to-remember slugs
- Support for SSH key paths and commands (like `pass`)
- Port forwarding management
- SFTP support
- Interactive command prompt
- Zsh completion

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/piotr-woojcik/ssh-manager.git
cd ssh-manager

# Build and install
go install ./cmd/ssh-manager
```

### Shell Completion (zsh)

Add to your ~/.zshrc if not already present:
```bash
autoload -U compinit; compinit
```

Generate completion script:
```bash
ssh-manager zsh-completion > "${fpath[1]}/_ssh-manager"
```

## Usage

### Adding a Connection
```bash
ssh-manager add
# Follow the interactive prompts:
# - Enter slug (e.g., 'prod-server')
# - Enter address (e.g., 'example.com')
# - Enter user (e.g., 'admin')
# - Enter SSH key path or command (e.g., '~/.ssh/id_rsa' or 'pass show ssh/key')
# - Is this a command? (y/n)
```

### Adding Port Forwarding
```bash
ssh-manager forward-port [slug]
# Enter local and remote ports when prompted
```

### Listing Connections
```bash
ssh-manager ls
```

### Connecting
```bash
ssh-manager connect [slug]    # SSH connection
ssh-manager connect-sftp [slug]  # SFTP connection
```

### Editing a Connection
```bash
ssh-manager edit [slug]
```

### Deleting a Connection
```bash
ssh-manager delete [slug]
```

## Configuration

All data is stored in `~/.ssh-manager/`:
- SQLite database: `~/.ssh-manager/ssh-manager.db`

## License

[MIT License](LICENSE)