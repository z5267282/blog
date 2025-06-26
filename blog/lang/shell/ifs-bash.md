In `bash`, how the shell processes arguments expanded via

```sh
command $var
```

where `$var` is unquoted depends on the `IFS` variable.
More information [here](https://unix.stackexchange.com/questions/26661/what-is-word-splitting-why-is-it-important-in-shell-programming/26672#26672).
Normally, this

```sh
bash
A='a b c'
python3 -c 'import sys; print(",".join(sys.argv))' $A
```

will run `command` with three arguments `a b c`.  
This is because there is normally whitespace in `IFS`.

Here is an example of something that will only run correctly on `zsh` compared to `bash`

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

.

In `zsh`

```
'foo:bar:foobar' is the splitted word
```

compared to `bash`

```
'foo' is the splitted word
'bar' is the splitted word
'foobar' is the splitted word
```

.
