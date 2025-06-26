# Overview

Changes to the `$PATH` environment variable made in `~/.zshrc` are not spread to other programs.

# Context

There is a separate issue relating to the `latexindent` command not working.  
This GitHub [issue](https://github.com/James-Yu/LaTeX-Workshop/issues/2135) references a problem where the wrong `latexindent` command is getting run.  
The solution fixes this by changing the order of folders in `$PATH`.

# Solution

This shell command can spread `$PATH` to other programs:

```sh
launchctl setenv PATH $PATH
```

. Source [here](https://stackoverflow.com/questions/135688/setting-environment-variables-on-os-x).
This can be added to `~/.zshrc`.
