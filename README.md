# dartgun

the stupid dotfile manager

---

<!--toc:start-->

- [dartgun](#dartgun)
  - [Impetus](#impetus)
  - [Non-goals](#non-goals)
  - [Goals](#goals)
  - [How it works](#how-it-works)
    - [Sample configuration](#sample-configuration)
    - [Why symlinking?](#why-symlinking)
    - [Why no Windows support?](#why-no-windows-support)
  - [Installation Guide](#installation-guide)
  - [Usage guide](#usage-guide)
    - [Linting dartgun.toml](#linting-dartguntoml)
  - [License](#license)
  <!--toc:end-->

---

## Impetus

Managing dot files are annoying. They're some of the most essential parts of a
developer's system, yet most operating systems (distros) don't provide a way to
easily manage and version them. There's the `~/.config` directory, which in
theory holds all of your configuration files. If this were true, you could
simply `git init` inside `~/.config` and have a versioned dotfile repo that you
could backup and deploy to new machines. However, configuration files often end
up all over a system. There's [NixOS](https://nixos.org/), but not everyone can
dedicate 40 hours a week to configuring their OS.

Advanced dotfile helper solutions do exist, but what most users need doesn't
really amount to more than a set of bash scripts to copy their dotfiles around
their system. A more "robust" tool which offers many features for managing your
configuration files also ends up requiring the user to look through a
[massive manpage](https://www.gnu.org/software/stow/manual/stow.txt). Whatever
happened to
[doing one thing, and doing it well](https://suckless.org/philosophy/)?

> Ingenious ideas are simple. Ingenious software is simple. [^1]

[^1]: Disclaimer: this software does not claim to be ingenious.

Dartgun's design statement is to be as simple as possible while providing just
enough features for most non power users. Anyone who reads through the example
configuration files should be able to utilize all of dartgun's features
immediately. It does not offer any advanced features or configuration options,
and is managed by a single `toml` file.

Dartgun essentially takes a central repository of configuration files and places
them into the correct places on your system by symlinking (or hardlinking).
Everything lives in one folder which can be versioned by git. The primary goal
is to provide an easy way to manage your dotfiles in a centralized area and sync
them between different systems. Dartgun allows you to specify which
configuration files are used on which systems by identifying each system with a
string. Instead of using your machine's `hostname`, the system names are
user-defined, which gives great configuration flexibility, for free. For
example, you get specific by giving each system a unique name, or extremely
general and simply have `mac` and `linux` as system identifiers.

A secondary goal is to assist in setting up a new system by helping place
configuration files in the correct places. This is useful for chronic
distro-hoppers or anyone re-installing their OS. However, automatically
installing additional software and dependencies is outside of the scope of the
project. Attempting to automatically set up a system via shell scripting (or
similar methods) is almost always unwieldy and relies on behavior which is
non-deterministic and may break at any time. NixOS is the best option for this,
and actually works, but again, requires the user to first dedicate a significant
amount of time to learning the syntax and idiosyncrasies of NixOS and the Nix
language.

Dartgun is for people who ain't got that kind of time.

## Non-goals

Dartgun is specifically designed to have a minimum amount of features to make it
as easy to adopt as possible. If you're reading this, I'll assume that you are
already looking for a dotfile manager. Therefore, it might be easier to list
things that Dartgun is _not_ designed to do. If you do not need any of these
features, then Dartgun might the right dotfile manager for you.

Dartgun is not for:

- Managing a fleet of computers
- Automatically deploying servers
- Deterministically setting up an entire OS from configuration files (see NixOS
  for that)
- Automatically installing packages or software alongside dotfiles; it only
  manages the files, the user is still responsible for ensuring software is
  available
- Power users who want deep customizability and feature sets to help fully
  automate system configuration

## Goals

- Easily version dotfiles with git by keeping everything in one central
  directory
- Copy dotfiles to a new machine quickly and transparently
- Sync dotfiles between different machines
- Keep configuration effort to a minimum

## How it works

Dartgun allows you to centralize all of your dotfiles in a single directory.

Dartgun is primarily configured through the `dartgun.toml` file, alongside the
`machine.toml` file to specify machine-specific configuration. If you are using
`git` to version your dotfile directory, it is recommended that you add
`machine.toml` to your `.gitignore` since it contains configuration which should
not be shared across machines.

Currently, the only configuration option in `machine.toml` is the following:

```toml
# file: machine.toml

identifiers = ["arch", "hypr", "etc"]
```

The primary mechanism by which dartgun determines whether to install a given set
of dotfiles to a system is through the _identifiers_. These are simply strings
that can represent whatever you want. You configure a set of identifiers for
your system in `machine.toml`, and specify which identifiers a given dotfile
applies to in the `dartgun.toml` file.

This is useful for installing distribution-specific files (like macOS, Arch
Linux, and Fedora specific files), or for applications which you do not always
install on every system.

### Sample configuration

Here is the structure of `dartgun.toml`:

```toml
# file: dartgun.toml

[[dots]]
# the location of the original files, in the dartgun directory. Can be relative or absolute.
location = "./nvim"
#
# The destination which to place the file at. MUST BE AN ABSOLUTE PATH.
destination = "/home/youwen/Projects/Mine/2024/dartgun/dartgun-test/target/nvim"
#
# Whether to use symbolic linking or hard linking. Symlinking is recommended and
# should be used unless there is a reason to use hard links instead.
# Keep in mind hard linking does NOT work on directories.
strategy = "symlink"
#
# The machine identifiers that this dotfile will match. If the identifiers listed
# in a `machine.toml` match ANY of the identifiers in this list, it will be copied on that machine.
identifiers = ["linux", "arch", "neovim"]

[[dots]]
location = "./.zshrc"
destination = "/home/youwen/.zshrc"
strategy = "symlink"
identifiers = ["linux", "macos"]
```

As shown, each separate dotfile directory or file should be prefixed with
`[[dots]]` (technical note: this represents an array of objects in TOML, where
each entry under `[[dots]]` is an object in the array).

### Why symlinking?

Any configuration updates made will always sync back to the dartgun directory so
they can be tracked by git. Likewise, any remote updates pulled in will also be
automatically reflected in the system for free.

### Why no Windows support?

Windows symlinks work a little differently from Unix symlinks. The difference is
not massive and I plan to look into supporting Windows at a later date.

## Installation Guide

`dartgun` can be built and installed from source through Rust's `cargo`.

## Usage guide

Begin by creating a folder to place your dotfiles in. I'll refer to it as the
"dartgun folder" for the rest of these instructions. Mine is located at
`~/.dartgun`, but it can be called whatever you want and located wherever you
want it.

In this directory, create two files called `machine.toml` and `dartgun.toml`.
Then, populate it with all of the dotfiles which you would like dartgun to
manage.

Read [the example configuration files](#sample-configuration) to learn how to
configure your `machine.toml` and `dartgun.toml`.

Note that the identifiers system lets you choose how you want to approach the
machine configuration. You can either have a specific identifier for each
computer you own, or specify identifier by platform (eg. "arch", "mac",
"fedora"), or specify identifiers by application (eg. "neovim", "hyprland",
"zsh"). You can also combine all three, and come up with other identification
schemes to fit your needs. You can get as fine-grained or generic as you'd like.

Then, run `dartgun fire` to apply your dotfiles to the locations. For nested
destination directories, it will attempt create all of the directories in the
path if they do not exist already. You may have to run `dartgun fire` with
superuser privileges if you are trying to copy files to restricted locations.

Because the files are symlinked or hardlinked, you can simply sync your dotfiles
by updating them in the dartgun folder, for example by running `git pull` after
you've set up git versioning.

### Linting dartgun.toml

Optionally, you can set up linting for `dartgun.toml` to ensure that it is
configured properly. A [JSON schema file](./dartgun.schema.json) is provided for
validating the structure of `dartgun.toml`.

The easiest way to use it is to add this line to the top of `dartgun.toml`.

`#:schema https://raw.githubusercontent.com/youwen5/dartgun/main/dartgun.schema.json`

Example:

```toml
# file: dartgun.toml
# add the line below to the top of your file, like so

#:schema https://raw.githubusercontent.com/youwen5/dartgun/main/dartgun.schema.json

[[dots]]
location = "./nvim"
destination = "/home/youwen/Projects/Mine/2024/dartgun/dartgun-test/target/nvim"
strategy = "symlink"
identifiers = ["linux", "arch", "neovim"]
```

If your editor is configured with the proper TOML linting extensions (eg.
[taplo](https://taplo.tamasfe.dev/)), it should lint the file for any errors or
invalid configuration options.

## License

This project is free software under the [GPL v3](./LICENSE).
