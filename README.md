# porteurbars [![Build Status](https://travis-ci.org/softprops/porteurbars.svg?branch=master)](https://travis-ci.org/softprops/porteurbars)

> portable github hosted project templates

> less assembly required

Porteurbars is a fast and simple command line interface for templatizing
your development workflow. Templatize away your boilerplate and share your templates
on github. Focus on your idea, not your idea's setup.

## goals

* no runtime dependencies
* use existing and familiar tools: handlebars, github
* use environment for configuration
* fast
* focused feature set
* fun

## installation

### github releases

```bash
curl -L "https://github.com/softprops/porteurbars/releases/download/v0.0.1/porteurbars-$(uname -s)-$(uname -m).tar.gz" \
  | tar -xv \
  > porteurbars
```

## assumptions

Porteurbars templates follow two simple conventions.  

1) Porteurbars follows the [12-factor philosophy](http://12factor.net/config) for how to configure your templates.
Porteurbars assumes a file at the root of a directory called `default.env` exists, containing
key value pairs that represent your templates environment variables. When applying a template,
this file will be read and the user will be prompted for each key that isn't defined in their
current environment. This works well on most systems and allows for promptless template execution
as you can specify and environment before running the program

```bash
$ FOO=bar BAR=baz porteurbars user/repo target
```

2) Porteurbars assumes a directory exists called `template` in your template's
root directory, next to your `default.env` file. This directory will contain arbitrary
handlebars template files representing the your templatized project. Porteurbars will walk
through this directory evaluating templates and copying results to your target directory.
If Porteurbars detects the presence of a local file will differences for a given file, you will be
prompted for whether or not you wish to keep those local changes.

## template hosting

Just upload your templates to github. That's it.

## Usage

### writing templates

Porteurbars defines a convention for writing templates with only two rules

1) create file at the root of a directory called `default.env` which stores
line-oriented key value pairs

```bash
$ touch default.env
$ echo "FOO=bar" > default.env
```


2) create a directory called `template` under which you define a set of handlebars templates

Porteurbars supports the notion of rendering templates from file content as well as file paths
so you can also templatize the location of your template files. See [softprops/mit](https://github.com/softprops/mit) for an example.

```bash
$ mkdir  template
$ echo "Hello {{FOO}}" > template/hello
```

Publishing a Porteurbars template is has simple has storing this work in a git repo.
To share these templates with others you can simply push this repo to github.

### applying templates

Install the porteurbars binary and ensure it's on your execution path.


porteurbars requires one and optionally a second argument.

The first argument is a reference to a template. The simplest case is using a
github user/repo. By default porteurbars will render this template in the current
working directory


```bash
$ porteurbars user/repo
```

If this is undesirable, you can provide a path to render into

```bash
$ porteurbars user/repo target_path
```

porteurbars will clone this template repo and read the defined template
variables from the default.env file. If any of these variables are not defined
in your env, porteurbars will prompt you for a value falling back on a default
if you do not provide one.

Finally porteurbars will apply that data to the handlebars templates and write
all files to the target path.

## alternatives

### [giter8](https://github.com/foundweekends/giter8)

This project is heavily influenced by giter8. porteurbars aims to solve some of
the issues I've experienced with it. giter8 is a jvm-based cli. To install it you
need to install another tool called conscript and before that you need to install
a jvm. porteurbars comes with a single standalone static binary. giter8 uses a
templating language many are not familiar with but can get acclimated to.
porteurbars uses handlebars templates in order to be familiar to a larger audience.
giter8 templates are just git repositories. porteurbars templates are as well.

## [yeoman](http://yeoman.io/)

Yeomon is a similar tool that is more focused on providing a scaffolding for template
authors to write node.js modules that serve as generators. porteurbars focuses
on having template authors just create handlebars templates. Yeomon requires
you to install the node runtime and also setup and account on npm to share your
work. porteurbars only requires git repositories. For convenience to facilitate
sharing it provides convenience for referencing github user repositories. Yeomon's
focus and/or marketing targets front end web development. porteurbars generalizes
the problem of templating any workflow.

Doug Tangren (softprops) 2016
