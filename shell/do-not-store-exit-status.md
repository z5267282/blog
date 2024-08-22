Do not store the exit status `$?` in a variable.  
You will get this error:

```
read-only variable: status
```

. The alternative would be to use a `trap` command.  
According to this [so post](https://stackoverflow.com/questions/36921658/save-command-output-on-variable-and-check-exit-status):

```
I recommend against the use of $? as much as possible, as it is fragile and easy to overlook when refactoring
```

.
