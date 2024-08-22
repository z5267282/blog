# Options

1. `find -exec command {} +` - work on aggregated file list
2. `find -exec [ command ] \;` - work on individual files

## Rough Shell Translation

1. `-exec command {} +`:

```sh
files='f1 f2 ... fn'
command f1 f2 ... fn
```

2. `-exec [ command ] \;` is like:

```sh
files='f1 f2 ... fn'
command f1
command f2
...
command fn
```

## Delimiter

`{}` is the currently iterated file\[s\].

## 1 - `{} +`

1. `find` will aggregate all files (ie. say in a variable `fs`)
2. The `{}` will be unpacked like so: `command f1 f2 f3 ...`

So these two should be the same

```sh
echo 1 > james
echo 2 > jane
echo 3 > john
find . -name 'ja*' -exec cat {} +
```

```sh
echo 1 > james
echo 2 > jane
echo 3 > john
cat james jane
```

. They both will output

```
1
2
```

. So `command` needs to be able to take in multiple command line arguments.

## 2 - `[ command ] \;`

The `;` character is a delimeter to the `-exec` flag.  
We can't type raw `;` because the shell will interpret it first.  
Hence we need to escape it - `\;`.  
Since we are working with individual files you can use `{}` to mean the current file name.

```sh
touch james jane john
find . -name 'ja*' -exec echo "{} says hello" \;
```

will output

```
james says hello
jane says hello
```

.

# Sources

[baeldung](https://www.baeldung.com/linux/find-exec-command)
