# path-rc

### Shell environment variables, aliases and functions determined by path.

The findrc binary is written in rust so is memory safe and blazingly fast!

The accompanying shell script is written to be bourne shell compliemt so should work sith any variant, bash, zsh ...

Currently at present there are no pre-built binaries so you need to build yourself.

NOTE: to build the findrc binary you will need rust installed: [Rust Getting Started](https://www.rust-lang.org/learn/get-started).

### Usage

**Clone the repository onto your local system**

```shell
$ git clone https://github.com/tpreecesh/path-rc.git
```

**Enable findrc**

1. Build the binary:

```shell
cd path-rc/findrc
cargo build --release
```

3. copy the target/release/findrc binary somewhere into your path

```shell
cp target/release/findrc /usr/local/bin
```

**Enable path-rc**

1. Source the path-rc.sh from your shell's rc file

```shell
$ source $HOME/repos/path-rc/path-rc.sh
```

2. Create .path-rc files in your directory structure to have environment variables, functions and aliases specific to the directory and children. Findrc searches for the .path-rc files by cascading up the directory structure and then the discovered files are sourced in a downward sequence to the current directory.

3. Optianally create a .path-rc file in your home or root directory to unset aliases, functions and variables that go out of scope and to set global defaults.

## EXAMPLE

~/some/path/.path-rc

```shell
# Directory scoped aliases, env variables and functions.
alias git='git -c core.sshCommand="ssh -i ~/.ssh/id_github"'
AWS_PROFILE="me-at-work"
export TF_WORKSPACE=dev-kubernetes-cluster
function hello() {
    echo "hello"
}
```

~/.path-rc

```shell
# Remove out of scope aliases, env variables and functions
alias | grep -q git && unalias git
unset TF_WORKSPACE
unset -f hello

# Add global defaults
AWS_PROFILE="me-at-home"
```

## Inspiration

ASDF: https://github.com/asdf-vm/asdf

Stack Exchange: https://unix.stackexchange.com/questions/6463/find-searching-in-parent-directories-instead-of-subdirectories
