# stash - move file(s) to trashcan

This program is an implementation of the [XDG Trash Specification](https://specifications.freedesktop.org/trash-spec/trashspec-latest.html).

## Usage

Trash one or more files or directories:

```console
$ stash foo/ bar
```

List files currently in all trashcans:

```console
$ stash --list
bar
foo/
foo/bar
```

Restore a file to its original path:

```console
$ stash --restore bar && ls
bar
```
