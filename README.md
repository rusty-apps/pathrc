# path-rc

Shell environment variables, aliases and functions determined by path

This is written to be bourne shell compliemt so should work sith any variant, bash, zsh ...

NOTE: to build the findrc binary you will need rust installed: [Rust Getting Started](https://www.rust-lang.org/learn/get-started).

## Usage

**Clone the repository onto your local system**
```shell
$ git clone https://github.com/tpreecesh/path-rc.git
```
**Enable findrc**

OPTION 1 Build the binary
1) Build the binary:
```shell
cd findrc
cargo build --release
```
3) copy the target/release/findrc binary into your path
```shell
cp target/release/findrc /usr/local/bin
```
OPTION 2 Use the scripted version
1) Uncomment the findrc function in the path-rc.sh script

**Enable path-rc**

1) Source the path-rc.sh from your shell's rc file
```shell
$ source $HOME/repos/path-rc/path-rc.sh
```
2) Create .path-rc files in your directory structure to have environment variables, functions and aliases specific to the directory and children

3) Optianally create a .path-rc file in your home or root directory to unalias and unset aliases, functions and variables that you dont want available throughout.

## EXAMPLE

~/some/path/.path-rc
```shell
alias git='git -c core.sshCommand="ssh -i ~/.ssh/id_github"'
export TF_WORKSPACE=dev-kubernetes-cluster
function hello() {
    echo "hello"
}
```

~/.path-rc
```shell
unalias git
unset TF_WORKSPACE
unset -f hello
```


## Inspiration

ASDF: https://github.com/asdf-vm/asdf

Stack Exchange: https://unix.stackexchange.com/questions/6463/find-searching-in-parent-directories-instead-of-subdirectories
