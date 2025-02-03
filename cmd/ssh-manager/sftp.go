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

var connectSftpCmd = &cobra.Command{
	Use:   "connect-sftp [slug]",
	Short: "Connect to an SFTP server",
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

		sftpArgs := []string{"-i", sshKeyArg}
		sftpArgs = append(sftpArgs, fmt.Sprintf("%s@%s", conn.User, conn.Address))

		sftpCmd := exec.Command("sftp", sftpArgs...)
		sftpCmd.Stdin = os.Stdin
		sftpCmd.Stdout = os.Stdout
		sftpCmd.Stderr = os.Stderr
		sftpCmd.Run()
	},
}
