package main

import (
	"bufio"
	"fmt"
	"log"
	"strings"

	"github.com/piotr-woojcik/ssh-manager/internal/config"
	"github.com/piotr-woojcik/ssh-manager/internal/db"
	"github.com/spf13/cobra"
)

func prompt(reader *bufio.Reader, text string) string {
	fmt.Print(text)
	input, _ := reader.ReadString('\n')
	return strings.TrimSpace(input)
}

var rootCmd = &cobra.Command{
	Use:   "ssh-manager",
	Short: "SSH connection manager with port forwarding",
	CompletionOptions: cobra.CompletionOptions{
		DisableDefaultCmd:   false,
		DisableNoDescFlag:   false,
		DisableDescriptions: false,
	},
}

func init() {
	if err := config.EnsureConfigDir(); err != nil {
		log.Fatal(err)
	}
	if err := db.InitDB(); err != nil {
		log.Fatal(err)
	}

	rootCmd.AddCommand(
		addCmd,
		deleteCmd,
		lsCmd,
		forwardPortCmd,
		connectCmd,
		connectSftpCmd,
		editCmd,
		completionCmd,
	)

	addShellCompletion()
}

func main() {
	if err := rootCmd.Execute(); err != nil {
		log.Fatal(err)
	}
}
