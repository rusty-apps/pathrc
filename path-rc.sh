# path-rc.sh
# 
# Source to extend cd and optionally include scripted fresion of findrc.
#

# Scripted version of findrc (may not work on all *NIX versions) 
#function findrc() {
#  echo $(FOUND_RC=''
#  while [ "${FOUND_RC}x" = "x" ]; do
#    CURRENT_DIR=$(pwd -P)
#    if [ -e $CURRENT_DIR/.path-rc ]; then
#      found="$CURRENT_DIR/.path-rc"
#    elif [ "$CURRENT_DIR" = "/" ]; then
#      break
#    else
#      cd -P ..
#    fi
#  done
#  echo $FOUND_RC)
#}

function cd() {
  builtin cd $@
  PATH_RC="$(findrc)"
  test -z $PATH_RC || source $PATH_RC
}