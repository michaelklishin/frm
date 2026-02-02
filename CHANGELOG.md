# frm Changelog

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
