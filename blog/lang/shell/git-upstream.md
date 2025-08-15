# Overview

This problem shows up when making a fresh branch with `git checkout -b`.

```txt
fatal: The current branch main has no upstream branch.
To push the current branch and set the remote as upstream, use

    git push --set-upstream origin main

To have this happen automatically for branches without a tracking
upstream, see 'push.autoSetupRemote' in 'git help config'.
```

# Solution

We can run this command to always configure our local branch to link to a branch with the same name on the remote repository.

```sh
git config --global --add --bool push.autoSetupRemote true
```

# Sources

1. [Stack Overflow](https://stackoverflow.com/questions/29422101/automatically-track-remote-branch-with-git)
