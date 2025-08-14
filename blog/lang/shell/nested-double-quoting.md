# Overview

There is no need to wrap variables expanded in subshells with double quotes.  
As soon as you write one `""`, white spaces are preserved in variable expansions.  
If you did then this should print out `0`:

```sh
[ -d t ] && rm -r t
cd t
touch 'three   two  one end.txt'
full='t/three   two  one end.txt'
cd ..
[ "$(basename $full)" = "$(basename "$full")" ]
echo $?
```
