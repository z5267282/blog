# Overview

When squashing commits after a pull request into main, git will warn saying that the branch has not yet been merged in.

## Recreation

```sh
# after squashing and merging on GitHub
git checkout main
git pull 
git branch -d feature
```

```txt
warning: deleting branch 'feature' that has been merged to
         'refs/remotes/origin/feature', but not yet merged to HEAD.
Deleted branch feature (was 333333).
```

## Explanation

This is okay.  
The squashed commit will have a new commit number eg from `feature`.  
Say there are these commits.

```txt
111111
222222
333333 <- feature is 333333
```

All these commits will be squashed into `444444` with all previous commit hashes now being lost.  
Hence the command line thinks that `333333` hasn't been merged in yet.
