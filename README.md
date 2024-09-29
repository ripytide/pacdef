# pacdef

multi-backend declarative package manager

## Installation

### Arch Linux

`pacdef` is available in the Arch User Repository in the
[`pacdef`](https://aur.archlinux.org/packages/pacdef) and
[`pacdef-bin`](https://aur.archlinux.org/packages/pacdef-bin)
packages.

### Cargo

To install `pacdef` using cargo:

```bash
cargo install pacdef
# or using cargo-binstall
cargo binstall pacdef
```

### Binary

Check out the releases page for downloading `pacdef` binaries for
various architectures: [latest
release](https://github.com/steven-omaha/pacdef/releases).

## Use-case

`pacdef` allows the user to have consistent packages among multiple
Linux machines and different backends by managing packages in group
files. The idea is that (1) any package in the group files ("managed
packages") will be installed explicitly, and (2) explicitly installed
packages _not_ found in any of the group files ("unmanaged packages")
will be removed. The group files are maintained outside of `pacdef` by
any VCS, like git.

If you work with multiple Linux machines and have asked yourself "_Why
do I have the program that I use every day on my other machine not
installed here?_", then `pacdef` is the tool for you.

## Multi-Backend

`pacdef` is a sort of meta package manager in that it does not
directly possess the functionality to install packages on your system,
instead it provides a single standardized interface for a whole bunch
of other non-meta package managers. See the [Supported
Backends](#supported-backends) section for a list of the currently
supported backend package managers.

## Declarative

`pacdef` is a declarative package manager, that means that you declare
in `.toml` group files the packages you would like installed on your
system and then run one of the `pacdef` commands which read these
group files and then operate on your system to do some function such
as install packages in your group files that are not present on your
system yet (`pacdef sync`), or remove packages present on your system
but not in your group files (`pacdef clean`).

## Supported Backends

At the moment, supported backends are the following. Pull requests for
additional backends are welcome!

Package Manager | Group Name     | Notes                                                                                    |
--------------- | ----------- | ---------------------------------------------------------------------------------------- |
`pacman`        | `[arch]`    | only one of `pacman`/`paru`/`yay` can
be used at once since they operate on the same local package database,
configure which one via the `arch_package_manager` config option|
`paru`        | `[arch]`    | includes pacman-wrapping AUR helpers (configurable)                                      |
`yay`        | `[arch]`    | includes pacman-wrapping AUR helpers (configurable)                                      |
`apt`           | `[debian]`  | minimum supported apt-version 2.0.2 ([see upstream](https://gitlab.com/volian/rust-apt)) |
`dnf`           | `[fedora]`  |                                                                                          |
`flatpak`       | `[flatpak]` | can manage either system-wide or per-user installation (configurable)                    |
`pip`           | `[python]`  |                                                                                          |
`cargo`         | `[rust]`    |                                                                                          |
`rustup`        | `[rustup]`  | See the comments [below](#rustup) about the syntax of the packages in the group file.    |
`xbps`          | `[void]`    |                                                                                          |

Backends that have a `feature flag` require setting the respective
flag for the build process. The appropriate system libraries and their
header files must be present on the machine and be detectable by
`pkg-config`. For backends that state "built-in", they are always
supported during compile time. Any backend can be disabled during
runtime (see below, "[Configuration](#configuration)").

For example, to build `pacdef` with support for Debian Linux, you can
run one of the two commands.

- (recommended) `cargo install -F debian pacdef`, this downloads and
  builds it from [https://crates.io](https://crates.io)
- in a clone of this repository, `cargo install --path . -F debian`

### Example

This tree shows my pacdef repository (not the `pacdef` config dir).

```ini
.
├── generic
│   ├── audio
│   ├── base
│   ├── desktop
│   ├── private
│   ├── rust
│   ├── wayland
│   ├── wireless
│   ├── work
│   └── xorg
├── hosts
│   ├── hostname_a
│   ├── hostname_b
│   └── hostname_c
└── pacdef.toml
```

- The `base` group holds all packages I need unconditionally, and
  includes things like zfs,
  [paru](https://github.com/Morganamilo/paru) and
  [neovim](https://github.com/neovim/neovim).
- In `xorg` and `wayland` I have stored the respective graphic servers
  and DEs.
- `wireless` contains tools like `iwd` and `bluez-utils` for machines
  with wireless interfaces.
- Under `hosts` I have one file for each machine I use. The filenames
  match the corresponding hostname. The packages are specific to one
  machine only, like device drivers, or any programs I use exclusively
  on that machine.

Usage on different machines:

- home server: `base private hostname_a`
- private PC: `audio base desktop private rust wayland hostname_b`
- work PC: `base desktop rust work xorg hostname_c`

### Example

Let's assume you have the following group files.

`base`:

```ini
[arch]
paru
zsh

[rust]
pacdef
topgrade
```

`development`:

```ini
[arch]
rustup
rust-analyzer

[rust]
cargo-tree
flamegraph
```

Pacdef will make sure you have the following packages installed for
each package manager:

- Arch (`pacman`, AUR helpers): paru, zsh, rustup, rust-analyzer
- Rust (`cargo`): pacdef, topgrade, cargo-tree, flamegraph

Note that the name of the section corresponds to the ecosystem it
relates to, rather than the package manager it uses.

## Commands

| Subcommand                        | Description                                                           |
| --------------------------------- | --------------------------------------------------------------------- |
| `group import [<path>...]`        | create a symlink to the specified group file(s) in your groups folder |
| `group export [args] <group> ...` | export (move) a non-symlink group and re-import it as symlink         |
| `group list`                      | list names of all groups                                              |
| `group new [-e] [<group>...]`     | create new groups, use `-e` to edit them immediately after creation   |
| `group remove [<group>...]`       | remove a previously imported group                                    |
| `group show [<group>...]`         | show contents of a group                                              |
| `package clean [--no_confirm]`    | remove all unmanaged packages                                         |
| `package review`                  | for each unmanaged package interactively decide what to do            |
| `package search <regex>`          | search for managed packages that match the search string              |
| `package sync [--no_confirm]`     | install managed packages                                              |
| `package unmanaged`               | show all unmanaged packages                                           |
| `version`                         | show version information, supported backends                          |

### Aliases

Most subcommands have aliases. For example, instead of `pacdef package
sync` you can write `pacdef p sy`, and `pacdef group show` would
become `pacdef g s`.

Use `--help` or the zsh completion to find the right aliases.

## Configuration

On first execution, it will create an empty config file under
`$XDG_CONFIG_HOME/pacdef/pacdef.toml`. The following key-value pairs
can be set. The listed values are the defaults.

```toml
aur_helper = "paru"  # AUR helper to use on Arch Linux (paru, yay, ...)
arch_rm_args = []  # additional args to pass to AUR helper when removing packages (optional)
disabled_backends = []  # backends that pacdef should not manage, e.g. ["python"], this can reduce runtime if the package manager is notoriously slow (like pip)

warn_not_symlinks = true  # warn if a group file is not a symlink
flatpak_systemwide = true  # whether flatpak packages should be installed system-wide or per user
pip_binary = "pip"  # choose whether to use pipx instead of pip for python package management (see below, 'pitfalls while using pipx')
```

## Group file syntax

Group files loosely follow the syntax for `ini`-files.

1. Sections begin by their name in brackets.
2. One package per line.
3. Anything after a `#` is ignored.
4. Empty lines are ignored.
5. If a package exists in multiple repositories, the repo can be
   specified as prefix followed by a forward slash. The package
   manager must understand this notation.

Example:

```ini
[arch]
alacritty
firefox  # this comment is ignored
libreoffice-fresh
mycustomrepo/zsh-theme-powerlevel10k

[rust]
cargo-update
topgrade
```

### Rustup

Rustup packages are managed quite differently. For referring to the
syntax, have a look [below](#group-file-syntax). In contrast to other
package managers, rustup handles package naming very differently.
These packages are either of the form `toolchain/<VERSION>` or
`component/<VERSION>/<component>`, where <VERSION> can be stable,
nightly, or any explicit rust version. The `<component>` field has to
be substituted with the name of the component you want installed.

Example:

```ini
[rustup]
component/stable/rust-analyzer
toolchain/stable
component/stable/cargo
component/stable/rust-src
component/stable/rustc
toolchain/1.70.0
component/1.70.0/cargo
component/1.70.0/clippy
component/1.70.0/rust-docs
component/1.70.0/rust-src
component/1.70.0/rust-std
component/1.70.0/rustc
component/1.70.0/rustfmt
```

## Misc.

### Automation

Pacdef is supported by
[topgrade](https://github.com/topgrade-rs/topgrade).

### Naming

`pacdef` combines the words "package" and "define".

### minimum supported rust version (MSRV)

MSRV is 1.74 due to dependencies that require this specific version.
Development is conducted against the latest stable version.

### Pitfalls while using pipx

Some packages like
[mdformat-myst](https://github.com/executablebooks/mdformat-myst) do
not provide an executable themselves but rather act as a plugin to
their dependency, which is mdformat in this case. Please install such
packages explicitly by running `pipx install <package-name>
--include-deps`.
