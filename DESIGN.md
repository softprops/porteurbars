# local storage

Porteurbars stores templates downloaded from github (in zip file format) locally
under `~/.porterbars/templates/` using the `get` command. A template tag has the following
specfication

```
user/repo
```

 `apply` will optionally download an install a template if its not resolvable
 locally

users are able to specify a branch (the default branch is master) to apply.

If the provided tag starts with a `/` it will be assumed they are referencing a
fully qualified path on the local filesystem.

# template directory structure

1) a `default.env` file exists in the root directory

2) a `template` directory exists containing a handlebars source files
