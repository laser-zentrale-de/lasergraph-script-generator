# Share timescript script

This script is used to share the timescript from a "master" lasergraph to multiple "nodes".

## Prerequisites

* Keep in mind, that if you have different **SetTimeMode** settings for the master and the nodes, you have to change this for the nodes before running the script.

## Files

| Name | Description |
| ---- | ----------- |
| ShareScript.DSCR | This saves the timescript on the master and triggers the LoadScript.DSCR script on all nodes. |
| LoadScript.DSCR  | This loads the previous saved timescript on the node. |

## Lasergraph Commands

### Master

On the master lasergraph the following commands will be executed:

1. root
1. script1
1. edit
1. forcecompile
1. root
1. file
1. cdd [device:\path\to\timescript\share]
1. savescript [name]
1. root
1. sendstring tcp://[node-ip]:[node-port]/startscript [device:\path\to\LoadScript.DSCR]

### Node

After the master sent the string to execute the **LoadScript.DSCR**-script, the following commands will be executed on the node lasergraph:

1. stop
1. root
1. file
1. cdd [device:\path\to\film\share]
1. loadscript [name] /overwrite
1. root
1. script1
1. play
