# frm Change Log

## v0.11.0 (in development)

### Enhancements

 * Extracted `rabbitmq-versioning` crate for RabbitMQ version parsing, comparison, and URL generation


## v0.10.0 (Jan 25, 2026)

### Enhancements

 * `use` and `default` commands now accept `latest` as a special version value; the value resolves to the most recent installed GA release
 * `releases uninstall`, `alphas uninstall`, and `alphas reinstall` also support `latest` for `--version` values

### Dependency Upgrades

 * `indicatif` upgraded to `0.18.3`
 * `reqwest` upgraded to `0.13.x`
 * `toml` upgraded to `0.9.11`


## v0.9.3 (Jan 25, 2026)

### Enhancements

 * Initial public release
