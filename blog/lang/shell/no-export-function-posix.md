# Overview

POSIX compliance is sometimes difficult to work with in `dash`.

# Example With Xargs

Using `xargs` it is handy to use Shell functions with the `-exec` flag.  
This is better than having to write an individual file for shell functions.

However, this is not possible as quoted from \[1\].

> In sh, it is not possible to export a function

A full explanation can be found in \[2\].

> No. The POSIX specification for export lacks the -f present in bash that allows one to export a function.
> A (very verbose) workaround is to save your function to a file and source it in the child script.

# Sources

1. [Stack Overflow](https://stackoverflow.com/questions/1885871/exporting-a-function-in-shell)
2. [Stack Overflow](https://stackoverflow.com/questions/29239806/how-to-export-a-function-in-bourne-shell)
