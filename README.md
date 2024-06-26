# dartgun

the stupid dotfile manager.

## Impetus

Managing dot files are annoying. They're some of the most essential parts of a
developer's system, yet most operating systems (distros) don't provide a way to
easily manage them. There's the `~/.config` directory, which in theory holds all
of your configuration files. If this were true, you could simply `git init`
inside `~/.config` and have a versioned dotfile repo that you could backup and
deploy to new machines. However, configuration files often end up all over a
system. There's [NixOS](https://nixos.org/), but not everyone can dedicate 40
hours a week to configuring their OS.

Advanced dotfile helper solutions do exist, but what most users need doesn't
really amount to more than a set of bash scripts to copy their dotfiles around
their system. A more "robust" tool which offers many features for managing your
configuration files also ends up requiring the user to look through a
[massive manpage](https://www.gnu.org/software/stow/manual/stow.txt). Whatever
happened to
[doing one thing, and doing it well](https://suckless.org/philosophy/)?

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

Dartgun is configured through the `dartgun.toml` file. You should set your
machine name in this file like so:

```toml
machine = "youwens-mac"
```

Also, it's common to have programs that are not installed on every computer.
Therefore, each dotfile will specify which application it is for, and whether or
not it should be applied by default. See the dotfile configuration below for how
to configure this.

You can specify which applications are in which machines with the `apps.toml`
file.

```toml
[youwens-mac]
available = ["neovim", "hyprland", "zsh"]
```

### Why symlinking?

Any configuration updates made will always sync back to the dartgun directory so
they can be tracked by git. Likewise, any remote updates pulled in will also be
automatically reflected in the system for free.

### Why no Windows support?

Windows symlinks work a little differently from Unix symlinks. The difference is
not massive and I plan to look into supporting Windows at a later date.

## Usage guide

Begin by creating a folder to place your dotfiles in. I'll refer to it as the
"dartgun folder" for the rest of these instructions. Mine is located at
`~/.dartgun`, but it can be called whatever you want and located wherever you
want it.

Place your dotfiles in the dartgun folder. You can organize them however you
want. The primary configuration is done in the `dartgun.json` file, located in
root of the dartgun folder. In here, you specify where each file or folder in
the directory goes.

For example, I can tell the `.dartgun/nvim` folder to go to `~/.config/nvim`
with

```json
{
  darts = [
    {
      location: "./nvim",
      destination: "~/.config/nvim",
      strategy: "hardlink",
      machines: [youwens-mac, youwens-archlinux],
      for: "neovim"
    }
  ]
}
```

```
location: the relative path, from the dartgun folder, where either the directory or file to be copied is located
destination: the destination path to which the file should be copied to. must be an absolute path
strategy: which strategy to use to copy the file. accepts 'hardlink', 'symlink', or 'copy'
machines: which machine names this file will be copied on
for: the application which the dotfile is for
```

Note that you can choose how you want to approach the machine configuration. You
can either have a specific machine key for each computer you own, or specify
machines by platform (eg. "arch", "mac", "fedora"). You can get as fine-grained
or generic as you'd like.

Then, run `dartgun blast` to apply your dotfiles to the locations. For nested
destination directories, it will create all of the directories in the chain if
they do not exist already. You may have to run `sudo dartgun blast` if you are
trying to copy files to restricted locations.

For symlinked and hardlinked directories, you can simply sync your dotfiles by
updating them in the dartgun folder, for example by running `git pull` after
you've set up git versioning.

If you use the `copy` strategy, then you need to re-run `dartgun blast` each
time you update the files in your dartgun folder. generally, we do not recommend
the copy method unless you have to for some reason, since it may lead to desyncs
between the dartgun folder and your actual system, while this is impossible with
hardlinking or symlinking.

## License

This project is licensed under [GPLv3](./LICENSE).
