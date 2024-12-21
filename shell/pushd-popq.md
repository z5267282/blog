Name recent folders with a stack-history rather than using `cd -`.  
Example:

```sh
$ pushd a
# now in a
a
$ pushd b
# now in b
b a
$ pushd c
# now in c
c b a
$ popd
# now in b
a
```

