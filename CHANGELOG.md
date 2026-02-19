# frm Changelog

## v0.20.0 (Feb 18, 2026)

### Bug Fixes

 * `releases install` really truly supports `--version [VERSION]`: the positional `[VERSION]` argument is now optional


## v0.19.0 (Feb 17, 2026)

### Enhancements

 * `frm releases install` now supports `--version [VERSION]` as well as the position `[VERSION]` argument

### Bug Fixes

 * `frm releases install` now correctly first fetches the list of releases from GitHub, then
   evalutes the `latest` version

### Dependency Updates

 * `clap` upgraded to `4.5.58`
 * `flate2` upgraded to `1.1.9`
 * `futures-util` upgraded to `0.3.32`
 * `indicatif` upgraded to `0.18.4`
 * `predicates` upgraded to `3.1.4`
 * `proptest` upgraded to `1.10.0`
 * `reqwest` upgraded to `0.13.2`
 * `tempfile` upgraded to `3.25.0`
 * `toml` upgraded to `1.0.2`


## v0.18.0 (Feb 6, 2026)

### Enhancements

 * `frm` nwo uses a [`conf.d`](https://www.rabbitmq.com/docs/configure#config-confd-directory) instead of a single file

 * `fg node` and `bg start` now log to both a file and the standard output by default.

   The default logging configuration now lives in `conf.d/90-logging.conf` instead of the default `rabbitmq.conf` template.


## v0.17.0 (Feb 1, 2026)

### Enhancements

 * Releases now include `.tar.gz` archives (Linux/macOS) and `.zip` archives (Windows) with README and LICENSE files
 * Releases now include a Homebrew formula for `brew install`
 * Releases now include an AUR PKGBUILD for Arch Linux
 * Releases now include a Winget manifest for Windows Package Manager
 * Releases now include a consolidated SHA256SUMS file


## v0.16.0 (Feb 1, 2026)

### Enhancements

 * Releases now include SBOM artifacts (SPDX and CycloneDX formats)
 * Releases now include DEB and RPM packages for Linux GNU targets
 * Releases now include DMG installers for macOS and MSI installers for Windows
 * Release artifacts are now signed with Sigstore


## v0.15.0 (Feb 1, 2026)

### Enhancements

 * New command group, `bg`, with `start` and `stop` subcommands for running RabbitMQ nodes in the background using `sbin/rabbitmq-server -detached`, `rabbitmqctl shutdown`
 * `releases check-signature` is a new command that verifies GPG signatures of installed releases

### Internal Changes

Some internal changes worth mentioning:

 * Significant refactoring to extract several shared modules under `frm::common::*`
 * Significant release automation improvements


## v0.14.0 (Jan 31, 2026)

### Enhancements

 * Release automation improvements


## v0.13.0 (Jan 31, 2026)

### Enhancements

 * New command `status` shows active, default, and installed versions
 * New commands `releases use`, `alphas use`, and `tanzu use` replace the top-level `use` command
 * Commands that accept a positional `[VERSION]` argument now also support `--version`/`-V` as an alternative:
   `releases use`, `releases install`, `releases reinstall`, `releases uninstall`, `alphas use`, `alphas install`, `alphas reinstall`, `alphas uninstall`, `tanzu use`, `default`

### Breaking Changes

 * `alphas install --latest` was removed; use `alphas install --version latest` (or `alphas install latest`) instead


## v0.12.0 (Jan 31, 2026)

### Enhancements

 * `releases install latest` now fetches the latest GA release from GitHub when no versions are installed locally
 * `frm help` now displays the tool version
 * New commands, `releases cp-etc-file` and `alphas cp-etc-file`, for copying configuration files to a version's `etc/rabbitmq` directory


## v0.11.0 (Jan 26, 2026)

### Enhancements

 * Group `env` and `completions` were moved under a new group, `shell`, so use them as `frm shell env` and `frm shell completions`
 * `shell completions` now supports Nu shell (pass `nushell` or `nu` as the argument value)
 * New hidden subcommands `releases completions` and `alphas completions` generate shell completion scripts for available versions
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
