# Share timescript script

This script is used to share the timescript from a "master" lasergraph to multiple "nodes".

## Prerequisites

* Keep in mind, that if you have different **SetTimeMode** settings for the master and the nodes, you have to change this for the nodes before running the script.

## Files

| Name | Description |
| ---- | ----------- |
| share-script.dscr | This saves the timescript on the master and triggers the load-script.dscr script on all nodes. |
| load-script.dscr  | This loads the previous saved timescript on the node. |
