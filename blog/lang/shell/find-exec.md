# Overview

This blog covers how to use the `find` shell command.  
It is very useful for running a shell command on multiple files matching a certain filter.

# Options

## 1. Aggregate Files

```sh
find -exec command {} +
```

Run `command` once and `{}` is passed as its argument list.  
This can be roughly translated in Shell like so.

```sh
command {1} {2} ... {n}
```

## 2. Individually Run

```sh
find -exec command \;`
```

Run `command` on all arguments given to `{}`.
This can be roughly translated in Shell like so.

```sh
command {1}
command {2}
command {...}
command {n}
```

The `;` character is a delimeter to the `-exec` flag.  
We can't type raw `;` because the shell will interpret it first.  
Hence we need to escape it - `\;`.

# Sources

1. [baeldung](https://www.baeldung.com/linux/find-exec-command)
