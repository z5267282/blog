# Overview

The ability for an empty glob to return no results can be turned on.

# Using setopt

Use this Shell command.

```sh
setopt -s nullglob
```

# The N Character

The `(N)` character can be used to mimic this behaviour.  
Suppose there are these files.

```txt
a
b
c
```

Then `echo fish*(N)` produces nothing as per \[1\].

# Not Using Glob

We can loop and manually break the loop if the literal glob gets returned from \[2\].

```sh
for txt in *.txt
do
  [ -e "$txt" ] || break
  echo "loading data from $txt"
done
```

# Sources

1. [Stack Exchange](https://unix.stackexchange.com/questions/26805/how-to-silently-get-an-empty-string-from-a-glob-pattern-with-no-matches).
2. [Super User](https://superuser.com/questions/519374/how-to-handle-bash-matching-when-there-are-no-matches)
