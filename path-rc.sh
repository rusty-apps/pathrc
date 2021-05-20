function cd() {
  builtin cd $@
  PATH_RC="$(findrc)"
  test -z $PATH_RC || source $PATH_RC
}