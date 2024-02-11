package cmd

import (
	"os"

	"github.com/laser-zentrale-de/lasergraph-script-generator/cmd/programming"
	"github.com/spf13/cobra"
)

const (
	descriptionShort = "Script generator for the Lasergraph DSP"
	descriptionLong  = `Script generator for the Laseranimation Sollinger Lasergraph DSP.

Use the subcommands to generate the desired scripts.
`
)

// rootCmd represents the base command when called without any subcommands
var rootCmd = &cobra.Command{
	Use:     "lasergraph-script-generator",
	Short:   descriptionShort,
	Long:    descriptionLong,
	Args:    cobra.NoArgs,
	Version: "0.1.0",
	Run: func(cmd *cobra.Command, args []string) {
		cmd.Help()
	},
}

func Execute() {
	err := rootCmd.Execute()
	if err != nil {
		os.Exit(1)
	}
}

func init() {
	// Subcommands
	rootCmd.AddCommand(programming.NewProgrammingCmd())
}
