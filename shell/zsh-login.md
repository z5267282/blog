# Overview

I am trying to decide if `~/.zshrc` is the best place to put updates to `$PATH`.  
Right now, putting modifications in `~/.zshrc` is resulting in duplicate folders.

# Order of scripts

There are special scripts prefixed `z` in `/etc` and `~` which control the setup of the shell.  
The order is:

1. `.zshenv`
2. `.zprofile`
3. `.zshrc`
4. `.zlogin`
5. `.zlogout`

.

There can be up to two copies of these files:

1. The first run is in `/etc`. This is the default configuration
2. The second one is in `~`. This overwrites the corresponding `/etc` version

.

Source [1](https://apple.stackexchange.com/questions/388622/zsh-zprofile-zshrc-zlogin-what-goes-where).

# Solution 1

Source 1 recommends putting `$PATH` setup in `.zshenv`.

# Solution 2

Source [2](https://apple.stackexchange.com/questions/432226/homebrew-path-set-in-zshenv-is-overridden) recommends putting them in `zprofile`.  
The issue is that `/etc/zprofile` runs a script `path_helper`.  
This script will **append** changes to `$PATH` made from `~/.zshenv`.  
This is bad because we want our own changes to `$PATH` to be **prepended**.
