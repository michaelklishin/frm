# frm shell completion for nushell
# Source this file in config.nu:
#   source /path/to/frm.nu
# Or:
#   use /path/to/frm.nu *

def "nu-complete frm commands" [] {
    [
        { value: "list", description: "List installed RabbitMQ versions" }
        { value: "install", description: "Install a RabbitMQ version" }
        { value: "use", description: "Output shell commands to use a specific version" }
        { value: "default", description: "Set the default RabbitMQ version" }
        { value: "uninstall", description: "Uninstall a RabbitMQ version" }
        { value: "env", description: "Output shell initialization script" }
        { value: "completions", description: "Generate shell completions" }
    ]
}

def "nu-complete frm versions" [] {
    frm list
    | lines
    | where { |line| $line | str starts-with "[" }
    | each { |line| $line | str replace --regex '^\[.\] ' '' }
}

def "nu-complete frm shells" [] {
    ["bash" "zsh" "nu"]
}

export extern "frm" [
    command?: string@"nu-complete frm commands"
    --help(-h)
    --version(-V)
]

export extern "frm list" [
    --help(-h)
]

export extern "frm install" [
    version: string
    --force(-f)
    --help(-h)
]

export extern "frm use" [
    version: string@"nu-complete frm versions"
    --shell(-s): string@"nu-complete frm shells"
    --help(-h)
]

export extern "frm default" [
    version: string@"nu-complete frm versions"
    --help(-h)
]

export extern "frm uninstall" [
    version: string@"nu-complete frm versions"
    --help(-h)
]

export extern "frm env" [
    shell: string@"nu-complete frm shells"
    --help(-h)
]

export extern "frm completions" [
    shell: string@"nu-complete frm shells"
    --help(-h)
]
