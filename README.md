# stash - move file(s) to trashcan

#### Usage

Stash one or more files or directories to trashcan:

```console
	$ stash foo/ bar
```

List files currently in the trashcan:

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
