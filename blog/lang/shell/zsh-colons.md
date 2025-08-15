# Overview

Putting `:` and one of `ahl` changes the behaviour of a path expansion in `zsh`.

```sh
[ -d top ] && rm -rf top
mkdir -p top/next
seq 3 | xargs -I % touch top/next/%

# l behaviour is to delete the first letter

zsh -c 'for f in top/next/*; do echo "$f:la"; done' > zsh-out.txt

cat << EOF > zsh-exp.txt
top/next/1a
top/next/2a
top/next/3a
EOF

diff zsh-???.txt

bash -c 'for f in top/next/*; do echo "$f:la"; done' > bash-out.txt

cat << EOF > bash-exp.txt
top/next/1:la
top/next/2:la
top/next/3:la
EOF

diff bash-???.txt
```
