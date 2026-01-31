# frm, for Frakking RabbitMQ Manager

This is `frm`, a RabbitMQ version switcher based on the [generic binary builds](https://www.rabbitmq.com/docs/install-generic-unix).

It is inspired by multiple (runtime) version management tools, from `rvm` to `kerl` to `fnm` to `rustup`.

## Intended Use Cases

`frm` was developed to be used in development and (certain) integration testing scenarios,
namely when testing client libraries against a range of RabbitMQ server versions.

## Project Maturity

`frm` is a very young project, breaking changes are likely.


## Usage

### Getting Help

```shell
frm help
```

which will output a list of command groups:

```
Frakking RabbitMQ version Manager

Usage: frm [COMMAND]

Commands:
  status       Show frm status: active version, default, installed versions
  releases     Install or manage RabbitMQ releases (GA, RCs, betas); for alphas, see the 'alphas' command group
  alphas       Install, manage, rotate alpha RabbitMQ releases
  tanzu        Install and manage Tanzu RabbitMQ from local tarballs
  conf         Manage RabbitMQ configuration files
  default      Set the default RabbitMQ version
  cli          Run a RabbitMQ CLI tool
  fg           Run RabbitMQ nodes in foreground
  inspect      Inspect a RabbitMQ configuration file
  shell        Shell-related operations
  help         Print this message or the help of the given subcommand(s)
```

To explore commands in a specific group, use

```shell
frm {group name} help
```

### Shell Setup

Add to your shell profile:

```shell
# bash: add to ~/.bashrc
eval "$(frm shell env bash)"

# zsh: add to ~/.zshrc
eval "$(frm shell env zsh)"

# nushell: save to file and source in config.nu
frm shell env nu | save -f ~/.local/frm/env.nu
```

After setup, use `frm-use <version>` to switch versions.

### Install a Release

```shell
frm releases install 4.2.3
```

```shell
frm releases install 4.2.0-rc.1
```

```shell
# Force reinstallation
frm releases install --force 4.2.3
```

### Install an Alpha Release

```shell
frm alphas install 4.2.0-alpha.20250120
```

### List Installed Releases

```shell
frm releases list
```

```shell
frm alphas list
```

### Uninstall a Release

```shell
frm releases uninstall 4.2.3
```

```shell
frm alphas uninstall 4.2.0-alpha.20250120
```

### Clean Up Alpha Releases

```shell
# Remove all alpha releases
frm alphas prune
```

```shell
frm alphas clean --older-than "2 weeks ago"
```

### Use a Specific Version

```shell
# bash/zsh
eval "$(frm releases use 4.2.3)"

# Use the latest installed GA version
eval "$(frm releases use latest)"
```

### Set Default Version

```shell
frm default 4.2.3

# Use the latest installed GA version
frm default latest
```

### Run RabbitMQ CLI Tools

```shell
frm cli rabbitmqctl -V 4.2.3 -- status
```

### Start RabbitMQ in Foreground

```shell
frm fg node -V 4.2.3
```

### Inspect Configuration Files

```shell
frm inspect rabbitmq.conf -V 4.2.3
```

### Manage rabbitmq.conf

```shell
frm conf set-key listeners.tcp.default 5673 -V 4.2.3
```

### Install Tanzu RabbitMQ

```shell
frm tanzu install --local-tanzu-rabbitmq-tarball-path /path/to/tanzu-rabbitmq.tar.xz -V 4.2.3
```

### Generate Shell Completions

```shell
frm shell completions bash > ~/.local/share/bash-completion/completions/frm
```

```shell
frm shell completions zsh > ~/.zfunc/_frm
```

```shell
frm shell completions fish > ~/.config/fish/completions/frm.fish
```

```shell
frm shell completions nushell | save -f ~/.cache/frm/completions.nu
```


## License

Double licensed under the MIT and Apache 2.0 (ASL2) licenses.

## Copyright

(c) 2025-2026 Michael S. Klishin and Contributors.
