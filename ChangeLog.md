# frm Change Log

## v0.12.0 (in development)

No changes yet.


## v0.11.0 (Jan 26, 2026)

### Enhancements

 * Group `env` and `completions` were moved under a new group, `shell`, so use them as `frm shell env` and `frm shell completions`
 * `shell completions` now supports Nu shell (pass `nushell` or `nu` as the argument value)
 * New hidden subcommands `releases completions` and `alphas completions` generate shell completion scripts
 * Initial CLI documentation in `README.md`

 ### Dependency Updates

  * Extracted `rabbitmq-versioning` crate for RabbitMQ version parsing, comparison, and URL generation


## v0.10.0 (Jan 25, 2026)

### Enhancements

 * `use` and `default` commands now accept `latest` as a special version value; the value resolves to the most recent installed GA release
 * `releases uninstall`, `alphas uninstall`, and `alphas reinstall` also support `latest` for `--version` values

### Dependency Updates

 * `indicatif` upgraded to `0.18.3`
 * `reqwest` upgraded to `0.13.x`
 * `toml` upgraded to `0.9.11`


## v0.9.3 (Jan 25, 2026)

### Enhancements

 * Initial public release
