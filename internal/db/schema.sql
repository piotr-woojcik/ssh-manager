CREATE TABLE IF NOT EXISTS connections (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    slug TEXT UNIQUE NOT NULL,
    address TEXT NOT NULL,
    user TEXT NOT NULL,
    ssh_key TEXT NOT NULL,
    is_key_cmd BOOLEAN NOT NULL
);

CREATE TABLE IF NOT EXISTS port_mappings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    connection_id INTEGER,
    local_port INTEGER NOT NULL,
    remote_port INTEGER NOT NULL,
    FOREIGN KEY(connection_id) REFERENCES connections(id) ON DELETE CASCADE
);