# frm shell completion for nushell
# Source this file in config.nu:
#   source /path/to/frm.nu
# Or:
#   use /path/to/frm.nu *

def "nu-complete frm commands" [] {
    [
        { value: "status", description: "Show frm status" }
        { value: "releases", description: "Manage RabbitMQ releases" }
        { value: "alphas", description: "Manage alpha releases" }
        { value: "tanzu", description: "Manage Tanzu RabbitMQ" }
        { value: "default", description: "Set the default RabbitMQ version" }
        { value: "shell", description: "Shell environment and completions" }
    ]
}

def "nu-complete frm releases versions" [] {
    frm releases completions --shell nu
    | lines
    | where { |line| $line | str length > 0 }
}

def "nu-complete frm alphas versions" [] {
    frm alphas completions --shell nu
    | lines
    | where { |line| $line | str length > 0 }
}

def "nu-complete frm shells" [] {
    ["bash" "zsh" "nu"]
}

export extern "frm" [
    command?: string@"nu-complete frm commands"
    --help(-h)
    --version(-V)
]

export extern "frm status" [
    --help(-h)
]

export extern "frm releases list" [
    --help(-h)
]

export extern "frm releases install" [
    version: string
    --force(-f)
    --help(-h)
]

export extern "frm releases use" [
    version: string@"nu-complete frm releases versions"
    --shell(-s): string@"nu-complete frm shells"
    --help(-h)
]

export extern "frm releases uninstall" [
    version: string@"nu-complete frm releases versions"
    --help(-h)
]

export extern "frm alphas list" [
    --help(-h)
]

export extern "frm alphas install" [
    version?: string
    --latest
    --force(-f)
    --help(-h)
]

export extern "frm alphas use" [
    version: string@"nu-complete frm alphas versions"
    --shell(-s): string@"nu-complete frm shells"
    --help(-h)
]

export extern "frm alphas uninstall" [
    version: string@"nu-complete frm alphas versions"
    --help(-h)
]

export extern "frm tanzu install" [
    --local-tanzu-rabbitmq-tarball-path: string
    --version(-V): string
    --force(-f)
    --help(-h)
]

export extern "frm tanzu use" [
    version: string@"nu-complete frm releases versions"
    --shell(-s): string@"nu-complete frm shells"
    --help(-h)
]

export extern "frm default" [
    version: string@"nu-complete frm releases versions"
    --help(-h)
]

export extern "frm shell env" [
    shell: string@"nu-complete frm shells"
    --help(-h)
]

export extern "frm shell completions" [
    shell: string@"nu-complete frm shells"
    --help(-h)
]
