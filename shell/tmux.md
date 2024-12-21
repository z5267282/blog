# Notes

1. Enter `tmux` to start
2. Cannot enter `Command + k` to clear screen
3. Any command letter that is a shift-pressed key, must have shift pressed to work

# Modifier

Press the modifier key and then a command letter.  
In Zac's `.tmux.conf` this was `Control + a`.

- It is apparently the most ergonomic combination.

By default it is `Control + b`.  
You have to release the modifier and then press the command letter as per this [guide](https://superuser.com/questions/266725/tmux-ctrlb-not-working).  
This is a list of [default command letters](https://man.openbsd.org/tmux#DEFAULT_KEY_BINDINGS).

# Windows

They are more like tabs in a browser.

| Letter | Description           |
| ------ | --------------------- |
| c      | make new window       |
| &      | kill current window   |
| 1..9   | go to window 1..9     |
| ,      | rename window         |
| p      | go to previous window |
| n      | go to next window     |
| &      | close window          |

# Pane

A window can be split into panes.  
Panes are closed by `Control + d` or the command letter `x`.  
Can switch using arrow keys as the command letter or `o`.

| Letter | Description              |
| ------ | ------------------------ |
| %      | split vertically         |
| "      | split horizontally       |
| z      | toggle pane as full size |
| x      | close pane               |

# Session

All open windows are saved in a session.
Exiting is also referred to as **detatching**.

| Letter | Description         |
| ------ | ------------------- |
| d      | exit out of session |

Sessions are 0-indexed.

## Shell Commands

Run these via `tmux` then provide the argument.

| Argument                | Description                                                            |
| ----------------------- | ---------------------------------------------------------------------- |
| `attach -t[n]`          | go back into a previous session replacing `[n]` with the index or name |
| `rename-session [name]` | rename the session to `[name]`                                         |
| `ls`                    | see all running sessions                                               |

Running `tmux` will start tmux with a new session.
[ TODO ] how to run previosu one?

# Configuration

The configuration file is stored in `~/.tmux.conf`.  
When it is changed, `tmux` will automatically update in response.  
However, if there are any running sessions, they must all be [exited first](https://unix.stackexchange.com/questions/66606/tmux-not-sourcing-my-tmux-conf#answer-66607).

# YouTube Guides

| Title                                                                              |
| ---------------------------------------------------------------------------------- |
| [Inside my iPad Pro SSH Setup - TMUX](https://www.youtube.com/watch?v=B-1wGwvUwm8) |
