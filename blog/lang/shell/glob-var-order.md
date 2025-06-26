# Overview

1. Variable substitution occurs
2. Then globbing

# Proof

If there are files

```txt
a aa b c
```

then the glob

```sh
a*
```

should have the files

```txt
a aa
```

. If you run

```sh
echo a*
```

you will get the right answer

```
a aa
```

. But if you put the glob in a variable it won't work.

# Script

```sh
touch a aa b c
echo a*
l='a*'
echo $l
```

# Double Quotes

Note that if you try

```sh
echo "$l"
```

the star will get interpretted literally.
