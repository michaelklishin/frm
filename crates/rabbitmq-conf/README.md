# rabbitmq-conf

A parser and manipulation library for RabbitMQ configuration files (`rabbitmq.conf`),
a partial port of RabbitMQ's (Erlang) [cuttlefish](https://github.com/kyorai/cuttlefish).

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rabbitmq-conf = "0.9"
```

## Features

 * Parses the [cuttlefish](https://github.com/kyorai/cuttlefish) ini-like format used by modern RabbitMQ
 * Validates keys against known RabbitMQ cuttlefish schemas (core `rabbit.schema` and all tier-1 plugins)
 * Preserves comments and whitespace for round-trip editing
 * Supports environment variable interpolation patterns (`$(VAR)`)
 * Handles encrypted values (`encrypted:` prefix)

## Grammar

Keys are identifiers consisting of one or more segments separated by dots:

```
listeners.tcp.default = 5672
auth_oauth2.resource_server_id = my_resource
```

Values containing `#` or spaces must be single-quoted:

```
default_pass = 'a-g3n#r47ed_pa$$w0rD'
```

See [BNF.Grammar.md](https://github.com/michaelklishin/frm/blob/main/crates/rabbitmq-conf/BNF.Grammar.md) for the complete grammar specification.

## License

Double licensed under the MIT and Apache 2.0 (ASL2) licenses.

## Copyright

(c) 2025-2026 Michael S. Klishin and Contributors.
