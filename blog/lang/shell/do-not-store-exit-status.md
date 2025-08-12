# Overview

Do not store the exit status `$?` in a variable.  
You will get this error:

```
read-only variable: status
```

# Solution

The alternative would be to use a `trap` command.

This sentiment is shared online \[1\].

> I recommend against the use of $? as much as possible, as it is fragile and easy to overlook when refactoring

# Sources

1. [Stack Overflow](https://stackoverflow.com/questions/36921658/save-command-output-on-variable-and-check-exit-status)
