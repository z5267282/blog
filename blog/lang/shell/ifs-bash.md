# Overview

In Bash, the shell processes arguments from unquoted variables based on the `IFS` variable.

```sh
command $var
```

# Example

```sh
bash
A='a b c'
python3 -c 'import sys; print(",".join(sys.argv))' $A
```

This will run `python3` with three arguments `a b c`.  
This is because there is normally whitespace in `IFS`.

# Z-Shell vs Bash

Here is an example of something that will only run correctly on `zsh` compared to `bash`.

```sh
#!/bin/zsh

string=foo:bar:foobar
old_ifs="$IFS"
IFS=":"
for i in $string
do
  echo "'$i' is the splitted word"
done
```

The following is the Z-Shell output.

```txt
foo:bar:foobar
```

Comparatively, this is the Bash output.

```txt
'foo' is the splitted word
'bar' is the splitted word
'foobar' is the splitted word
```

# Sources

1. [Stack Exchange](https://unix.stackexchange.com/questions/26661/what-is-word-splitting-why-is-it-important-in-shell-programming/26672#26672)
