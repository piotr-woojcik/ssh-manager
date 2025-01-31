package main

import (
	"fmt"
	"github.com/piotr-woojcik/ssh-manager/internal/db"
	"github.com/spf13/cobra"
	"log"
	"strings"
)

var lsCmd = &cobra.Command{
	Use:   "ls",
	Short: "List all SSH connections",
	Run: func(cmd *cobra.Command, args []string) {
		connections, err := db.ListConnections()
		if err != nil {
			log.Fatal(err)
		}

		for _, conn := range connections {
			mappings, err := db.GetPortMappings(conn.ID)
			if err != nil {
				log.Fatal(err)
			}

			var ports []string
			for _, pm := range mappings {
				ports = append(ports, fmt.Sprintf("%d:%d", pm.LocalPort, pm.RemotePort))
			}
			portsStr := "none"
			if len(ports) > 0 {
				portsStr = strings.Join(ports, ", ")
			}

			fmt.Printf("%s: %s@%s (Key: %s) Ports: %s\n",
				conn.Slug, conn.User, conn.Address, conn.SSHKey, portsStr)
		}
	},
}
