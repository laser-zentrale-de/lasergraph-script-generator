package programming

import (
	"os"

	"github.com/spf13/cobra"
)

// Subcommand description
const (
	progDescriptionShort = "Render the programming scripts"
	progDescriptionLong  = `Render the scripts neccessary for the programming with multiple Lasergraph DSPs.

The scripts must then placed into the master Lasergraph DSP.
Also these scripts must be accessible to the nodes via file share.
`
)

var (
	output  string
	address []string
)

func NewProgrammingCmd() *cobra.Command {

	// progCmd represents the prog command
	cmd := &cobra.Command{
		Use:   "prog",
		Short: progDescriptionShort,
		Long:  progDescriptionLong,
		Run: func(cmd *cobra.Command, args []string) {
			if len(args) < 1 {
				cmd.Help()
				os.Exit(1)
			}
		},
	}

	// Output path
	cmd.Flags().StringVarP(&output, "output", "o", "", "output path for the scripts")
	cmd.Flags().StringSliceVarP(&address, "address", "a", []string{}, "list of IP addresses of the Lasergraph DSP nodes")

	return cmd
}
