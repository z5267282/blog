The ability for an empty glob to return no results can be turned on.

# Using `setopt`

The shell command is

```sh
setopt -s nullglob
```

.

# The `(N)` character

The `(N)` character can be used to mimic this behaviour.
Eg. if there are these files:

```txt
a
b
c
```

then `echo fish*(N)` produces `` as per [here](https://unix.stackexchange.com/questions/26805/how-to-silently-get-an-empty-string-from-a-glob-pattern-with-no-matches).

# Not using glob at all

Manually break the loop if the literal glob gets returned

```sh
for txt in *.txt
do
  [ -e "$txt" ] || break
  echo "loading data from $txt"
done
```

from [here](https://superuser.com/questions/519374/how-to-handle-bash-matching-when-there-are-no-matches).
