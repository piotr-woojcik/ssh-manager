package main

import (
	"bufio"
	"fmt"
	"github.com/piotr-woojcik/ssh-manager/internal/db"
	"github.com/spf13/cobra"
	"log"
	"os"
)

var deleteCmd = &cobra.Command{
	Use:   "delete [slug]",
	Short: "Delete an SSH connection",
	Run: func(cmd *cobra.Command, args []string) {
		reader := bufio.NewReader(os.Stdin)

		var slug string
		if len(args) > 0 {
			slug = args[0]
		} else {
			slug = prompt(reader, "Enter connection slug to delete: ")
		}

		err := db.DeleteConnection(slug)
		if err != nil {
			log.Fatal(err)
		}
		fmt.Printf("Deleted connection: %s\n", slug)
	},
}
