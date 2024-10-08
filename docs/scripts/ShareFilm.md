# Share film script

This script is used to share a film from a "master" lasergraph to multiple "nodes".

## Prerequisites

* The node lasergraph must have all pictures loaded in order to load a new film, produced on the master.
* The film will be forcecompiled within the script-workflow but you should compile it for anyway to ensure everything works as expected.

## Files

| Name | Description |
| ---- | ----------- |
| ShareFilm.DSCR | This saves the film on the master and triggers the LoadFilm.DSCR script on all nodes. |
| LoadFilm.DSCR  | This loads the previous saved film on the node. |

## Lasergraph Commands

### Master

On the master lasergraph the following commands will be executed:

1. root
1. film1
1. edit
1. forcecompile
1. root
1. file
1. cdd [device:\path\to\film\share]
1. savefilm film1 [name]
1. root
1. sendstring tcp://[node-ip]:[node-port]/startscript [device:\path\to\LoadFilm.DSCR]

### Node

After the master sent the string to execute the **LoadFilm.DSCR**-script, the following commands will be executed on the node lasergraph:

1. stop
1. root
1. file
1. cdd [device:\path\to\film\share]
1. loadfilm [name] film1 /overwrite /quiet
1. loadfilm [name] film2 /overwrite /quiet
1. root
1. script1
1. play
