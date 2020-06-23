# `twilight-http` development

## Gateway tests

Gateway runtime tests will connect to Discord's gateway for testing. Tests
should be run one at a time with `-j1`. Tests will also run slowly and be
intentionally delayed, due to ratelimits. This should be used with a
delvelopment bot in only 1 or 2 guilds at most.

To run gateway runtime tests, run:

```shell
$ env DISCORD_TOKEN="your token here" cargo test -j1 -- --ignored
$ # if you need to print output for testing, run:
$ env DISCORD_TOKEN="your token here" cargo test -j1 -- --ignored --nocapture
```
