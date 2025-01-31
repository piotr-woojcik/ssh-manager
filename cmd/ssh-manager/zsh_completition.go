package main

import (
	"github.com/piotr-woojcik/ssh-manager/internal/db"
	"github.com/spf13/cobra"
	"log"
	"os"
)

var completionCmd = &cobra.Command{
	Use:   "zsh-completion",
	Short: "Generate zsh completion script",
	Long: `To load zsh completions:

  # If shell completion is not already enabled in your environment,
  # you will need to enable it. Add to your ~/.zshrc:
  autoload -U compinit; compinit

  # Generate and load completion script:
  ssh-manager zsh-completion > "${fpath[1]}/_ssh-manager"`,
	Run: func(cmd *cobra.Command, args []string) {
		err := cmd.Root().GenZshCompletion(os.Stdout)
		if err != nil {
			log.Fatal(err)
		}
	},
}

func addShellCompletion() {
	for _, cmd := range []*cobra.Command{editCmd, deleteCmd, connectCmd, connectSftpCmd, forwardPortCmd} {
		cmd.ValidArgsFunction = func(cmd *cobra.Command, args []string, toComplete string) ([]string, cobra.ShellCompDirective) {
			slugs, err := db.GetSlugs()
			if err != nil {
				return nil, cobra.ShellCompDirectiveError
			}
			return slugs, cobra.ShellCompDirectiveNoFileComp
		}
	}
}
