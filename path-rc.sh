# path-rc.sh
# 
# Source to extend cd and optionally include scripted fresion of findrc.
#

function cd() {
  builtin cd $@
  IFS=$'\n'; set -f; for i in $(findrc); do source "$i"; done; set +f; unset IFS
}
