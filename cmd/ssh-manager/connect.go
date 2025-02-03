package main

import (
	"bufio"
	"fmt"
	"github.com/piotr-woojcik/ssh-manager/internal/db"
	"github.com/spf13/cobra"
	"log"
	"os"
	"os/exec"
	"strings"
)

var connectCmd = &cobra.Command{
	Use:   "connect [slug]",
	Short: "Connect to an SSH server",
	Run: func(cmd *cobra.Command, args []string) {
		reader := bufio.NewReader(os.Stdin)

		var slug string
		if len(args) > 0 {
			slug = args[0]
		} else {
			slug = prompt(reader, "Enter connection slug: ")
		}

		conn, err := db.GetConnection(slug)
		if err != nil {
			log.Fatal(err)
		}

		mappings, err := db.GetPortMappings(conn.ID)
		if err != nil {
			log.Fatal(err)
		}

		var portArgs []string
		for _, pm := range mappings {
			portArgs = append(portArgs, fmt.Sprintf("-L %d:localhost:%d", pm.LocalPort, pm.RemotePort))
		}

		var sshKeyArg string
		if conn.IsKeyCmd {
			keyOutput, err := exec.Command("sh", "-c", conn.SSHKey).Output()
			if err != nil {
				log.Fatal(err)
			}
			sshKeyArg = strings.TrimSpace(string(keyOutput))
		} else {
			sshKeyArg = strings.TrimSpace(conn.SSHKey)
		}

		sshArgs := []string{"-i", sshKeyArg}
		sshArgs = append(sshArgs, portArgs...)
		sshArgs = append(sshArgs, fmt.Sprintf("%s@%s", conn.User, conn.Address))

		sshCmd := exec.Command("ssh", sshArgs...)
		sshCmd.Stdin = os.Stdin
		sshCmd.Stdout = os.Stdout
		sshCmd.Stderr = os.Stderr
		sshCmd.Run()
	},
}
