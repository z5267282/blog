# Overview

In the Shell, variables are substituted and then globbing occurs.  
This means that any glob characters should appear literally unquoted.

# Example

Suppose there are these files.

```txt
a
aa
b
c
```

Then the glob `a*` should have these files.

```txt
a
aa
```

Running `echo a*` will list those files correctly.  
However, if you put the glob expression in a variable you won't get the same result.

```sh
l='a*'
echo $l
```
