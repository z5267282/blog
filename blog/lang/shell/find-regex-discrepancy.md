# A Discrepancy Between the find command regex flag and grep

I was trying to select these two files which were yet to be committed in my git repo.

```
css-replicate-layered-and-mobile/styles.css
css-replicate-simple-flex/style.css
```

The basename for both match the approximate regex `styles?.css` (i.e. `style.css` with an optional `s` after the `e`).

If I run `find` to find all `.css` files and then manually `grep` the output I am able to locate the two files.

```
> find . -regex '.*.css' | grep -E '.*styles?.css'
./css-replicate-layered-and-mobile/styles.css
./css-replicate-simple-flex/style.css
```

However, the native `find -regex` flag which should match the same files as the `grep` does not locate these files.

```
> find . -regex '.*styles?.css'
```

I am not sure why not.
