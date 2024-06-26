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

Advanced solutions dotfile helpers, but most users don't really need much more
than a set of bash scripts which copies their dotfiles around their system.

Dartgun essentially does this in a more systematic manner, and seeks to be just
as still simple to set up and manage. Everything lives in one folder which can
be versioned by git. Dartgun will put your dotfiles in the correct places by
symlinking from the central dotfile directory to the specified locations.

The primary goal is to provide an easy way to manage your dotfiles in a
centralized area and sync them between different systems. A secondary goal is to
help automatically set up a new system with the configuration files in the
correct places. However, automatically installing additional software and
dependencies is outside of the scope of the project.

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
