# PATHRC:

## Command line utility to provide per path configurations for aliases, environment variables and functions.

## Currently supported shells:
* `Bash`
* `Zsh`
* `Fish`(Experimental)

*Other untested bourne shell deviates including the `Bourne` shell may also work if forced to run as `Bash`.* 

## Installation

There is currently no automated method to install but hopefully this will be implemented soon. 

The currently recommended installation process is:

1) Manually download a release from https://github.com/rusty-apps/pathrc/releases
2) Validate the release against it checksums
3) Move the binary into /usr/local/bin
4) chmod +x /usr/local/bin/pathrc 

## TLDR;

Add 
```
eval "$(pathrc init)"
```

to your shell init file:

By default, pathrc rc files should be named according to the shell:

* Under `Bash` pathrc searches for files named `.pathrc.bash`
* Under`Zsh` pathrc searches for files named `.pathrc.zsh`
* Under`Fish` pathrc searches for files named `.pathrc.fish`

During initialisation shells with a reasonable level of compatibility i.e. `Bash` and `Zsh` can use the same files by 
enabling shared config, see usage below. When shared config is enabled both `Bash` and `Zsh` will use `.pathrc.sh`.

The number of directories travelled up can be configured by the PATHRC_DEPTH environment variable, see environment 
variables below. 

## Commands:

### Init
There are three commands the most important of which is `init` which initialises the shell to use pathrc. By default, 
without any arguments `init` will try to determine the shell from the SHELL environment variable. In some cases there 
may be a need to explicitly specify the shell, i.e. if the SHELL environment is missing or in an unrecognised format.
Being able to override the shell might also be useful for running under different but compatible shells i.e. specifying 
bash with the `Bourne` shell. NOTE this is completely untested!

### Collate
This is the engine that collates all the pathrc configuration files which get evaluated into the shell when changing 
directory.

### List
This is a useful utility for identifying the pathrc configuration files that will be processed at the current directory.


## Usage  

`pathrc` itself has the usual flags for help `-h` and version `-V`.

`list` and `collate` have no further options.

`init` takes an optional `shell` argument with `env` being the default which tried to fetch the current shell from the 
environment. In addition `init` can also take an optional flag `-s` to enable shared configuration and should be 
configured in the shell's init file as 
```
eval "$(pathrc init -s)"
```
or to force a shell i.e. bash
```
eval "$(pathrc init bash -s)"
```

## Environment variables

### PATHRC_DEPTH
This optional environment variable overrides the maximum directory
descent when searching for configuration files.

The default depth is set 1000.

### PATHRC_SHARED_CONFIG

This optional environment variable can be set to TRUE or FALSE
and changes the of how the bourne shell derivatives `Bash` and `Zsh` config files are named.
The default is TRUE and causes `pathrc` to read .pathrc.sh config files for both shells.
If set to FALSE then the config files are not shared, instead `pathrc` reads .pathrc.bash and .pathrc.zsh accordingly.

PATHRC_SHELL

### PATHRC_SHELL
This environment variable is dynamically set when the shell init file or the file that configures pathrc is invoked. 

The value of the variable is set to the name of the shell so not to rely on the SHELL environment variable that maybe 
missing or different in some systems.
