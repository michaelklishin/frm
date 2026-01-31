#!/usr/bin/env nu

let version = $env.VERSION
let header = $'## v($version)'

let lines = open CHANGELOG.md | lines
let start_idx = $lines | enumerate | where { |it| $it.item | str starts-with $header } | get 0.index

let remaining = $lines | skip ($start_idx + 1)
let end_offset = $remaining | enumerate | where { |it| $it.item | str starts-with '## v' } | get -i 0.index | default ($remaining | length)

let section = $lines | skip $start_idx | take ($end_offset + 1)

$section | str join "\n" | save --force release_notes.md
