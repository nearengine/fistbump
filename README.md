# fistbump

![Build](https://github.com/chrisshiplet/fistbump/workflows/Build%20&%20Test/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/fistbump.svg)](https://crates.io/crates/fistbump)
[![Languages](https://img.shields.io/badge/languages-Rust-red.svg)]()
[![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/chrisshiplet/fistbump/blob/master/LICENSE)

A tiny command line utility for managing your project's version numbers.

## Installation

Obtain the binary from the Releases page and put it somewhere on your PATH. Make sure to `chmod +x` the binary on Linux and MacOS.

**With Cargo:** Run `cargo install fistbump`

### macOS

Supported by macOS 10.7+. Packaged with [Homebrew](https://brew.sh/).

**With Brew:**

```
brew tap nearengine/tap
brew install nearengine/tap/fistbump
```

## Usage

### Configuration

`fistbump` is used with a configuration file that lives in your project directory. To specify a configuration, create a file in your project's root directory called `.fistbumprc.json`.

#### Example

```json
{
    "current_version": "0.1.0",
    "files": [
        {
            "path": "Cargo.toml",
            "search": "version = \"{current_version}\"",
            "replace": "version = \"{new_version}\""
        }
    ],
    "search": "{current_version}",
    "replace": "{new_version}"
}
```

#### Defaults

The `search` and `replace` configs are always optional. They default to `{current_version}` and `{new_version}`, respectively. You can change these to match on more specific strings, but don't forget to include the `{current_version}` or `{new_version}` in your string. They may only be plain strings, regex is not supported at this time.

The only required fields are `current_version` and `files` (which must each contain a `path`).

### Arguments

When you create a new release on your project, run `fistbump <version>` where `<version>` is the new version string. For example: `fistbump 1.0.0`.

### Environment Variables

Set `FISTBUMP_LOG_LEVEL` to view detailed logs while `fistbump` is running. The default value is `error`. Valid values are `debug`, `info`, `error`. Example to view all output: `FISTBUMP_LOG_LEVEL=debug fistbump 1.0.0`.

## Develop

Run `FISTBUMP_LOG_LEVEL=debug cargo run -- 1.0.0` to view all debugging output.

## Publish

-   `fistbump <version>`
-   `cargo update`
-   `git commit -am "<version>"`
-   `git tag -a <version> -m "<release notes>"`
-   `git push --all`
-   `cargo publish --dry-run`
-   `cargo publish`

## Contribute

If `fistbump` makes your life easier, consider contributing to further development by donating to [bitcoin:18Xg5pLi4ueMCpoNHt5of44AirPhzjXXQ2](bitcoin:18Xg5pLi4ueMCpoNHt5of44AirPhzjXXQ2)

If you would like to contribute code to the project, feel free to submit pull requests for existing issues or file an issue for an enhancement or bug request. Pull requests will be merged at the sole discretion of the maintainer.
