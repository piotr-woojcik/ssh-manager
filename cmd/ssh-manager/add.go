package main

import (
	"bufio"
	"fmt"
	"github.com/piotr-woojcik/ssh-manager/internal/db"
	"github.com/spf13/cobra"
	"log"
	"os"
	"strings"
)

var addCmd = &cobra.Command{
	Use:   "add",
	Short: "Add a new SSH connection",
	Run: func(cmd *cobra.Command, args []string) {
		reader := bufio.NewReader(os.Stdin)

		slug := prompt(reader, "Enter slug: ")
		address := prompt(reader, "Enter address: ")
		user := prompt(reader, "Enter user: ")
		sshKey := prompt(reader, "Enter SSH key path or command: ")
		isKeyCmd := prompt(reader, "Is this a command? (y/n): ")

		err := db.AddConnection(slug, address, user, sshKey, strings.ToLower(isKeyCmd) == "y")
		if err != nil {
			log.Fatal(err)
		}
		fmt.Printf("Added connection: %s\n", slug)
	},
}
