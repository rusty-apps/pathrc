# path-rc.sh
#
# Source to extend cd and optionally include scripted fresion of findrc.
#

canonical=$(cd -P -- "$(dirname -- "$0")" && printf '%s\n' "$(pwd -P)/$(basename -- "$0")")
release_path=$(dirname $canonical)/pathrc/target/release

export PATH=$PATH:$release_path
hash

eval "$(pathrc)"

function cd() {
  builtin cd $@
  eval "$(pathrc)"
}
