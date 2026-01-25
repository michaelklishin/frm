# rabbitmq.conf BNF Grammar

The [`cuttlefish`](https://github.com/kyorai/cuttlefish) configuration format used by modern RabbitMQ.

## Grammar

```bnf
<config>         ::= <line>*

<line>           ::= <setting> | <comment> | <empty>

<setting>        ::= <ws> <key> <ws> "=" <ws> <value> <inline-comment>? <eol>

<key>            ::= <key-segment> ("." <key-segment>)*

<key-segment>    ::= <alpha> <key-char>*
                   | <digit>+

<key-char>       ::= <alpha> | <digit> | "_"

<value>          ::= <quoted-value> | <unquoted-value>

<quoted-value>   ::= "'" <quoted-char>* "'"

<quoted-char>    ::= any character except "'"

<unquoted-value> ::= (<any-char> - "#" - <eol>)*

<comment>        ::= <ws> "#" <any-char>* <eol>

<inline-comment> ::= <ws> "#" <any-char>*

<empty>          ::= <ws> <eol>

<ws>             ::= (" " | "\t")*

<eol>            ::= "\n" | "\r\n" | <eof>

<alpha>          ::= [a-zA-Z]

<digit>          ::= [0-9]

<any-char>       ::= any character except <eol>
```

## A Very Brief Description in English

 * Keys are identifiers that consist of one or more segments (path elements) separated with dots: `listeners.tcp.default`, `auth_oauth2.resource_server_id`
 * Values containing `#` or spaces must be single-quoted: `'a-g3n#r47ed_pa$$w0rD'`
 * Inline comments start with `#` after the value
 * The format does not support escape sequences inside quotes
 * Environment variables can be interpolated: `$(VAR)` (note: the patters are preserved literally without interpolation by this crate's parser)
 * Encrypted values use the `encrypted:` prefix (preserved literally by the parser)
