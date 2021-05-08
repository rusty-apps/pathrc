# path-rc

Shell environment variables, aliases and functions determined by path

This is written to be bourne shell compliemt so should work sith any variant, bash, zsh ...

NOTE: This script overrides the cd builtin function and expects the HOME environment variable to be set.

## USAGE
1) Clone the repository onto your local system
```
$ git clone https://github.com/tpreecesh/path-rc.git
```
2) Source the path-rc.sh from your shell's rc file

```bash
$ source $HOME/repos/path-rc/path-rc.sh
```
3) Create .path-rc files in your directory structure to have environment variables, functions and aliases specific to the directory and children

## EXAMPLE

.path-rc

alias git='git -c core.sshCommand="ssh -i ~/.ssh/id_github"<br/>
export TF_WORKSPACE=dev-kubernetes-cluster

## Inspiration

ASDF: https://github.com/asdf-vm/asdf

Stack Exchange: https://unix.stackexchange.com/questions/6463/find-searching-in-parent-directories-instead-of-subdirectories
