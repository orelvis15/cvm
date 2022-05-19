## CVM - Cardano Version Manager

Version Management System for Cardano Pools.

This application is implemented based on the scripts generated by the community, with the aim of facilitating a little more the management of changes between versions of cardano node.

## Support

|   OS    |    BASH     |     ZSH     |    FISH     |
|:-------:|:-----------:|:-----------:|:-----------:|
| Ubuntu  |     Yes     | Coming Soon | Coming Soon |
| Debian  |     Yes     | Coming Soon | Coming Soon |
| centos  | Coming Soon | Coming Soon | Coming Soon |
| RedHat  | Coming Soon | Coming Soon | Coming Soon |
| Fedora  | Coming Soon | Coming Soon | Coming Soon |
| Rasbian | Coming Soon | Coming Soon | Coming Soon |

## Install CVM

```
curl https://raw.githubusercontent.com/orelvis15/cvm/master/install.sh -sSf | bash
```

And then run this

```
source "$HOME"/.cvm/env
```
## Prepare Pool

Prepare the server to be able to run cardano-node.

* Install all necessary dependencies.
* Create the community-proposed folder structure in [prepare.sh](https://github.com/cardano-community/guild-operators/blob/alpha/scripts/cnode-helper-scripts/prereqs.sh#L427) and other extras.
* Download all the configuration files depending on the network we select (mainnet by default)
* Download all the [scripts](https://github.com/cardano-community/guild-operators/tree/alpha/scripts/cnode-helper-scripts) that the community offers us

### Prepare pool for mainnet

`cvm init`
or
`cvm init -n mainnet`

### Prepare pool for testnet

`cvm init -n testnet`

## Install cardano node

Build the version of cardano node passed by parameters, by default the latest version is built.

### build the x.x.x version

`
cvm install -v x.x.x
`

### build last version

`
cvm install
`

## List install versions

List all version installed.

`
cvm list
`
or
`
cvm -l
`

## Use versions

Start using the version passed by parameters.

`
cvm use -v x.x.x
`
or
`
cvm use --version x.x.x
`