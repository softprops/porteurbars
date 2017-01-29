# porteurbars [![Build Status](https://travis-ci.org/softprops/porteurbars.svg?branch=master)](https://travis-ci.org/softprops/porteurbars)

> portable github hosted project templates

Porteurbars is a fast and simple command line interface for templatizing
your development workflow. Templatize your boilerplate, share your templates
on github. Focus on your idea, not your ideas setup.

## goals

* no runtime dependencies
* use existing and familiar tools: handlebars, github
* use environment for configuration
* fast
* focused feature set
* fun

## assumptions

Porteurbars templates follow two simple conventions.  

1) Porteurbars follows the [12-factor philosophy](http://12factor.net/config) for how to configure your templates.
Porteurbars assumes a file at the root of a directory called `default.env`, containing
key value pairs that represent your templates environment variables. When applying a template,
this file will be read and the user will be prompted for each key that isn't defined in their
current environment. This works well on most systems and allows for promptless template execution
as you can specify and environment before running the program

```bash
FOO=bar BAR=baz porteurbars user/repo target
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
echo "FOO=bar" > default.env
```


2) create a directory called `template` under which you define a set of handlebars templates

Porteurbars supports the notion of rendering templates from file content as well as file paths
so you can also templatize the location of your template files.

```bash
$ mkdir  template
echo "Hello {{FOO}}" > template/hello
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

Doug Tangren (softprops) 2016
