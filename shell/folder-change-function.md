Changing a folder in a function will persist outside it.  

```sh
#!/bin/dash

mkdir a
cd a
mkdir b

change() {
    cd b
}

change
# and not a
[ "$(basename $(pwd))" = b ]
exit $?
```

