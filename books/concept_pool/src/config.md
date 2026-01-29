# microcad config

Microcad will requires a configuration.
The configuration should be a simple key value and is preferable stored and loaded as TOML.
The configuration source is hierarchical, sorted by preference:

* a global configuration located in `~/.config/microcad/microcad.toml`.
* a local configuration located in the current directory:
  * `.microcad.toml`
  * `.microcad/microcad.toml`
* A custom config passed via command line:
  * `microcad -c some_config_file.toml`
* A config settings passed via command line:
  * `microcad -c some.config.option=42`

## microcad CLI `config` command

A CLI command `config` that prints the current config (as TOML?) might be useful:

```sh
microcad config # Print current config as TOML
```
