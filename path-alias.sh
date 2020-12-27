function cd() {
  rpath=$HOME
  if [ -z $@ ]; then
    builtin cd "$rpath"
  else
    rpath=$(realpath -s $@)
    builtin cd "$rpath"
    shift 1
  fi
  while [[ $rpath != / ]];
  do
    apath=$rpath"/.aliases"
    if [ -e $apath ]; then
      source $apath
      return
    else
      rpath="$(realpath -s "$rpath"/..)"
    fi
  done
  return 0
}

