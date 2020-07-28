<!-- cargo-sync-readme start -->

[![license badge][]][license link] [![rust badge]][rust link]

# twilight-command-parser

`twilight-command-parser` is a command parser for the [`twilight`] ecosystem.

Included is a mutable configuration that allows you to specify the command
names and prefixes. The parser parses out commands matching an available
command and prefix and provides the command arguments to you.

# Installation

`twilight-command-parser` requires at least Rust 1.36.0.

Add the following to your Cargo.toml:

```toml
[dependencies]
twilight-command-parser = "0.1"
```

### Examples

A simple parser for a bot with one prefix (`"!"`) and two commands: `"echo"`
and `"ping"`:

```rust,no_run
use twilight_command_parser::{Command, CommandParserConfig, Parser};

let mut config = CommandParserConfig::new();

config.add_command("echo", false);
config.add_command("ping", false);

// Add the prefix `"!"`.
// (Use `CommandParserConfig::add_prefixes` to add multiple prefixes)
config.add_prefix("!");

let parser = Parser::new(config);

// Now pass a command to the parser
match parser.parse("!echo a message") {
    Some(Command { name: "echo", arguments, .. }) => {
        let content = arguments.as_str();

        println!("Got an echo request to send `{}`", content);
    },
    Some(Command { name: "ping", .. }) => {
        println!("Got a ping request");
    },
    // Ignore all other commands.
    Some(_) => {},
    None => println!("Message didn't match a prefix and command"),
}
```

[license badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=flat-square
[license link]: https://opensource.org/licenses/ISC
[rust badge]: https://img.shields.io/badge/rust-1.36+-93450a.svg?style=flat-square
[rust link]: https://blog.rust-lang.org/2019/07/04/Rust-1.36.0.html
[`twilight`]: https://twilight.valley.cafe

<!-- cargo-sync-readme end -->
