package db

import (
	"database/sql"
	"embed"

	_ "github.com/mattn/go-sqlite3"
	"github.com/piotr-woojcik/ssh-manager/internal/config"
)

// Embed SQL schema
//
//go:embed schema.sql
var schemaFS embed.FS

var DB *sql.DB

// Connection represents an SSH connection entry
type Connection struct {
	ID       int
	Slug     string
	Address  string
	User     string
	SSHKey   string
	IsKeyCmd bool
}

// PortMapping represents a port forwarding configuration
type PortMapping struct {
	ID           int
	ConnectionID int
	LocalPort    int
	RemotePort   int
}

// InitDB initializes the SQLite database
func InitDB() error {
	dbPath, err := config.GetDBPath()
	if err != nil {
		return err
	}

	// Open SQLite database
	DB, err = sql.Open("sqlite3", dbPath)
	if err != nil {
		return err
	}

	// Read and execute schema
	schema, err := schemaFS.ReadFile("schema.sql")
	if err != nil {
		return err
	}

	_, err = DB.Exec(string(schema))
	return err
}

// GetConnection retrieves a connection by its slug
func GetConnection(slug string) (*Connection, error) {
	conn := &Connection{}
	err := DB.QueryRow(`
		SELECT id, slug, address, user, ssh_key, is_key_cmd
		FROM connections WHERE slug = ?
	`, slug).Scan(&conn.ID, &conn.Slug, &conn.Address, &conn.User, &conn.SSHKey, &conn.IsKeyCmd)

	if err != nil {
		return nil, err
	}
	return conn, nil
}

// GetPortMappings retrieves all port mappings for a connection
func GetPortMappings(connectionID int) ([]PortMapping, error) {
	rows, err := DB.Query(`
		SELECT id, connection_id, local_port, remote_port
		FROM port_mappings WHERE connection_id = ?
	`, connectionID)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var mappings []PortMapping
	for rows.Next() {
		var pm PortMapping
		err := rows.Scan(&pm.ID, &pm.ConnectionID, &pm.LocalPort, &pm.RemotePort)
		if err != nil {
			return nil, err
		}
		mappings = append(mappings, pm)
	}
	return mappings, nil
}

// AddConnection adds a new SSH connection
func AddConnection(slug, address, user, sshKey string, isKeyCmd bool) error {
	_, err := DB.Exec(`
		INSERT INTO connections (slug, address, user, ssh_key, is_key_cmd)
		VALUES (?, ?, ?, ?, ?)
	`, slug, address, user, sshKey, isKeyCmd)
	return err
}

// UpdateConnection updates an existing SSH connection
func UpdateConnection(id int, slug, address, user, sshKey string, isKeyCmd bool) error {
	_, err := DB.Exec(`
		UPDATE connections 
		SET slug = ?, address = ?, user = ?, ssh_key = ?, is_key_cmd = ?
		WHERE id = ?
	`, slug, address, user, sshKey, isKeyCmd, id)
	return err
}

// DeleteConnection deletes a connection and its port mappings
func DeleteConnection(slug string) error {
	_, err := DB.Exec("DELETE FROM connections WHERE slug = ?", slug)
	return err
}

// AddPortMapping adds a new port mapping to a connection
func AddPortMapping(connectionID, localPort, remotePort int) error {
	_, err := DB.Exec(`
		INSERT INTO port_mappings (connection_id, local_port, remote_port)
		VALUES (?, ?, ?)
	`, connectionID, localPort, remotePort)
	return err
}

// ListConnections returns all SSH connections
func ListConnections() ([]Connection, error) {
	rows, err := DB.Query(`
		SELECT id, slug, address, user, ssh_key, is_key_cmd
		FROM connections
	`)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var connections []Connection
	for rows.Next() {
		var conn Connection
		err := rows.Scan(&conn.ID, &conn.Slug, &conn.Address, &conn.User, &conn.SSHKey, &conn.IsKeyCmd)
		if err != nil {
			return nil, err
		}
		connections = append(connections, conn)
	}
	return connections, nil
}

// GetSlugs returns all connection slugs
func GetSlugs() ([]string, error) {
	rows, err := DB.Query("SELECT slug FROM connections")
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var slugs []string
	for rows.Next() {
		var slug string
		if err := rows.Scan(&slug); err != nil {
			return nil, err
		}
		slugs = append(slugs, slug)
	}
	return slugs, nil
}
