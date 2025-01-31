package main

import (
	"bufio"
	"fmt"
	"github.com/piotr-woojcik/ssh-manager/internal/db"
	"github.com/spf13/cobra"
	"log"
	"os"
	"strconv"
)

var forwardPortCmd = &cobra.Command{
	Use:   "forward-port [slug]",
	Short: "Add port forwarding to a connection",
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

		localPortStr := prompt(reader, "Enter local port: ")
		remotePortStr := prompt(reader, "Enter remote port: ")

		localPort, _ := strconv.Atoi(localPortStr)
		remotePort, _ := strconv.Atoi(remotePortStr)

		err = db.AddPortMapping(conn.ID, localPort, remotePort)
		if err != nil {
			log.Fatal(err)
		}
		fmt.Printf("Added port forwarding for %s: %d -> %d\n", slug, localPort, remotePort)
	},
}
