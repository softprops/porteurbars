# porteurbars [![Build Status](https://travis-ci.org/softprops/porteurbars.svg?branch=master)](https://travis-ci.org/softprops/porteurbars)

> portable git hosted project templates

Porteurbars is a self-service command line interface for bootstrapping
your workflow by generating projects from github hosted handlebars templates.

## goals

* no runtime dependencies
* use existing familiar tools: handlebars, git(hub)
* use environment for configuration
* fast
* focused feature set
* fun

## assumptions

Porteurbars templates follow a two simple conventions.  

1) Porteurbars follows the [12-factor philosophy](http://12factor.net/config) for how to configure your templates.
Porteurbars assumes a file at the root of a directory called `default.env`, containing
key value pairs that represent your templates environment variables. When applying a template,
this file will be read and the user will be prompted for each key that isn't defined in their
current environment. This works well on most systems and allows for promptless template execution
as you can specify and environment before running the program

```bash
FOO=bar BAR=baz pb apply user/repo target
```

2) Porteurbars assumes a directory exists called `template` in your template's
root directory, next to your `default.env` file. This directory will contain arbitrary
handlebars template files representing the your templatized project. Porteurbars will walk
through this directory evaluating templates and copying results to your target directory.
If Porteurbars detects the existence of a local file will differences for a given file, you will be
prompted for whether or not you wish to keep those local changes.

## template hosting

Just upload your templates in github. That's it.

## Usage

TODO


Doug Tangren (softprops) 2016
