# path-rc

Shell environment variables, aliases and functions determined by path

This is written to be bourne shell compliemt so should work sith any variant, bash, zsh ...

NOTE: to build the findrc binary you will need rust installed: [Rust Getting Started](https://www.rust-lang.org/learn/get-started).

## Usage

1) Clone the repository onto your local system
```shell
$ git clone https://github.com/tpreecesh/path-rc.git
```
2) Build the binary:
```shell
cd findrc
cargo build --release
```
3) copy the target/release/findrc binary into your path
```shell
cp target/release/findrc /usr/local/bin
```
2) Source the path-rc.sh from your shell's rc file
```shell
$ source $HOME/repos/path-rc/path-rc.sh
```
3) Create .path-rc files in your directory structure to have environment variables, functions and aliases specific to the directory and children

## EXAMPLE

.path-rc

alias git='git -c core.sshCommand="ssh -i ~/.ssh/id_github"'<br/>
export TF_WORKSPACE=dev-kubernetes-cluster

## Inspiration

ASDF: https://github.com/asdf-vm/asdf

Stack Exchange: https://unix.stackexchange.com/questions/6463/find-searching-in-parent-directories-instead-of-subdirectories
