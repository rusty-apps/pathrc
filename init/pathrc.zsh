autoload -U add-zsh-hook
pathrc_after_cd() {
       eval "$(pathrc collate)"
}
add-zsh-hook chpwd pathrc_after_cd

