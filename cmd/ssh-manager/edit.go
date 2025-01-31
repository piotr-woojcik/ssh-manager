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

var editCmd = &cobra.Command{
	Use:   "edit [slug]",
	Short: "Edit an SSH connection",
	Run: func(cmd *cobra.Command, args []string) {
		reader := bufio.NewReader(os.Stdin)

		var slug string
		if len(args) > 0 {
			slug = args[0]
		} else {
			slug = prompt(reader, "Enter connection slug to edit: ")
		}

		conn, err := db.GetConnection(slug)
		if err != nil {
			log.Fatal(err)
		}

		newSlug := prompt(reader, fmt.Sprintf("Enter new slug (current: %s): ", conn.Slug))
		if newSlug == "" {
			newSlug = conn.Slug
		}

		newAddress := prompt(reader, fmt.Sprintf("Enter new address (current: %s): ", conn.Address))
		if newAddress == "" {
			newAddress = conn.Address
		}

		newUser := prompt(reader, fmt.Sprintf("Enter new user (current: %s): ", conn.User))
		if newUser == "" {
			newUser = conn.User
		}

		newSSHKey := prompt(reader, fmt.Sprintf("Enter new SSH key (current: %s): ", conn.SSHKey))
		if newSSHKey == "" {
			newSSHKey = conn.SSHKey
		}

		newIsKeyCmd := prompt(reader, fmt.Sprintf("Is this a command? (y/n) (current: %v): ", conn.IsKeyCmd))
		isKeyCmd := conn.IsKeyCmd
		if newIsKeyCmd != "" {
			isKeyCmd = strings.ToLower(newIsKeyCmd) == "y"
		}

		err = db.UpdateConnection(conn.ID, newSlug, newAddress, newUser, newSSHKey, isKeyCmd)
		if err != nil {
			log.Fatal(err)
		}
		fmt.Printf("Updated connection: %s\n", newSlug)
	},
}
