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

At the moment, supported backends are the following. Pull Requests for
additional backends are welcome!

| Backend               | Group Name  | Notes                                 |
| --------------------- | ----------- | ------------------------------------- |
| `pacman`/`paru`/`yay` | `[arch]`    | see the `arch_package_manager` config |
| `apt`                 | `[apt]`     |                                       |
| `dnf`                 | `[dnf]`     |                                       |
| `flatpak`             | `[flatpak]` |                                       |
| `pipx`                | `[pipx]`    |                                       |
| `cargo`               | `[cargo]`   |                                       |
| `rustup`              | `[rustup]`  |                                       |
| `xbps`                | `[xbps]`    |                                       |

## Config

```toml
# By default the `pacdef` `config.toml` file is expected in the
# `XDG_CONFIG_HOME/pacdef` directory (`~/.config/pacdef/config.toml`)
# unless using the `--config-dir` cli option.

# To decide which group files are relevant for the current machine
# `pacdef` uses the machine's hostname in the `hostname_groups` table in
# the config file to get a list of group file names.

# Since pacman, yay and paru all operate on the same package database
# they are mutally exclusive and so you must pick which one you want
# pacdef to use.
# Examples include: "pacman", "paru", "yay"
# Default: "pacman"
arch_package_manager = "paru"

# Extra arguments passed to pacman when removing an arch package.
# Default: []
arch_rm_args = ["-ns"]

# Whether to install flatpak packages systemwide or for the current user.
# Default: true
flatpak_systemwide = true

# Backends to disable from all pacdef behavior. See the README.md for
# the list of backend names
# Default: []
disabled_backends = ["apt"]

# Which group files apply for which hostnames
# Default: None
[hostname_groups]
pc = ["example_group"]
laptop = ["example_group"]
server = ["example_group"]
```

## Group Files

```toml
# Declared Packages in toml format can come in two formats, short-form
# and long-form:
#
# short-form syntax is simply a string of the name of the package.
#
# long-form syntax is a table which contains several fields which can
# optionally be set to specify install options on a per-package basis.
# The "package" field in the table specifies the name of the package.
#
# For example, the following two packages are equivalent:
# arch = [
# 	"pacdef",
# 	{ package = "pacdef" }
# ]

arch = [
	"pacdef",
	# optional_deps: additional packages to install with this package, short-form syntax only
	{ package = "pacdef",  optional_deps = ["git"] }
]
cargo = [
	"pacdef",
	# see cargo docs for info on the options
	{ package = "pacdef", version = "0.1.0", git = "https://github.com/ripytide/pacdef", all_features = true, no_default_features = false, features = [ "feature1", ] },
]
pipx = [
	"pacdef",
	{ package = "packdef" }
]
apt = [
	"pacdef",
	{ package = "packdef" }
]
xbps = [
	"pacdef",
	{ package = "packdef" }
]
flatpak = [
	"pacdef",
	{ package = "packdef" }
]
dnf = [
	"pacdef",
	# see dnf docs for more info on these options
	{ package = "pacdef", repo = "/etc/yum.repos.d/fedora_extras.repo" },
]
rustup = [
	"stable",
	# components: extra non-default components to install with this toolchain
	{ package = "stable", components = ["rust-analyzer"] }
]
```

## Commands

Run `pacdef -h` to see an overview of the commands available with
`pacdef`.

## Miscellaneous

### Automation

Pacdef is supported by
[`topgrade`](https://github.com/topgrade-rs/topgrade).

## Naming

`pacdef` combines the words "package" and "define".

## Backend Pitfalls

### Pipx

Some packages like
[`mdformat-myst`](https://github.com/executablebooks/mdformat-myst) do
not provide an executable themselves but rather act as a plugin to
their dependency, which is mdformat in this case. Please install such
packages explicitly by running: `pipx install <package-name>
--include-deps`.
