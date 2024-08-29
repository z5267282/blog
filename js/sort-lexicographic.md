The in-build `Array.sort` method sorts by lexicographic order so:

```js
const x = [1, 2, 10, 20];
x.sort();
assert(x[0] === 1);
assert(x[1] === 10);
assert(x[2] === 2);
assert(x[3] === 20);
```

. A sorting function must be provided

```js
const x = [1, 2, 10, 20];
x.sort((a, b) => a - b);
```

.

