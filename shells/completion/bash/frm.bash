# frm shell completion for bash
# Source this file or add to ~/.bashrc:
#   source /path/to/frm.bash
# Or generate dynamically:
#   eval "$(frm completions bash)"

_frm() {
    local cur prev words cword
    _init_completion || return

    local commands="list install use default uninstall env completions"

    case $prev in
        frm)
            COMPREPLY=($(compgen -W "$commands" -- "$cur"))
            return
            ;;
        install|use|default|uninstall)
            # Version completion - list installed versions
            local versions
            versions=$(frm list 2>/dev/null | sed 's/^\[.\] //')
            COMPREPLY=($(compgen -W "$versions" -- "$cur"))
            return
            ;;
        env|completions)
            COMPREPLY=($(compgen -W "bash zsh nu" -- "$cur"))
            return
            ;;
        --shell|-s)
            COMPREPLY=($(compgen -W "bash zsh nu" -- "$cur"))
            return
            ;;
    esac

    case $cur in
        -*)
            case ${words[1]} in
                install)
                    COMPREPLY=($(compgen -W "--force -f --help" -- "$cur"))
                    ;;
                use)
                    COMPREPLY=($(compgen -W "--shell -s --help" -- "$cur"))
                    ;;
                *)
                    COMPREPLY=($(compgen -W "--help" -- "$cur"))
                    ;;
            esac
            ;;
    esac
}

complete -F _frm frm
