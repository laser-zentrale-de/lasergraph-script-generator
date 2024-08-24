# Lasergraph Script Generator

Script generator for the LaserAnimation Sollinger Lasergraph DSP.

## Usage

This commandline tool uses subcommands to generate scripts and write them to the local filesystem.

### Root command

This is the help of the root command.

```bash
Usage: lasergraph-script-generator <COMMAND>

Commands:
  programming  Generate the programming scripts
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Programming command

This is the help of the programming command.<br>
It generates the scripts needed for programming on master DSP and deploy the changes to 
the node DSPs.

```bash
Usage: lasergraph-script-generator programming [OPTIONS] --share-path <SHARE_PATH> --load-path <LOAD_PATH> --dest-path <DEST_PATH> --master <MASTER>

Options:
  -s, --share-path <SHARE_PATH>
          Set the path to the share folder

  -l, --load-path <LOAD_PATH>
          Set the path to the load folder

  -d, --dest-path <DEST_PATH>
          Set the local destination path of the scripts

  -p, --port <PORT>
          TCP port of the Lasergraph DSP nodes

          [default: 8210]

  -n, --nodes <NODES>
          IP addresses of all Lasergraph DSP nodes

  -m, --master <MASTER>
          IP address of the Lasergraph DSP master

  -h, --help
          Print help (see a summary with '-h')
```
