# Mina Block Explorer

[![Build status](https://badge.buildkite.com/1f8c338cb4ede4e41a4d84de89479fb2eddf9a6f64b72dcf36.svg?branch=main)](https://buildkite.com/granola/mina-block-explorer)

The Mina Block Explorer is a user-friendly website for exploring the Mina
Blockchain. It offers a straightforward interface to view details about blocks,
transactions, addresses, and validators.

## Development Environment

This project uses [Nix](https://nix.dev) (via [Flox](https://flox.dev)) to ensure a consistent and reproducible development
environment. This choice streamlines the development process, allowing focus on value delivery with minimal concern for varying system configurations or
dependency conflicts.

### Prerequisites

[Install Flox](https://flox.dev/docs/install-flox/) (use `Nix-Generic` method)

Flox will install Nix for you (including `nix develop` and friends).
__If you have previously installed Nix, you may need to uninstall it first:__

- [The Determinate Nix Uninstaller](https://github.com/DeterminateSystems/nix-installer#uninstalling)
- [Standard Nix uninstall process](https://nix.dev/manual/nix/2.24/installation/uninstall)

## Quick Start

1. You must first enter the Flox environment by using `flox activate`.

Running any [`rake`](Rakefile) task will automatically enter the Flox environment.

2. Execute `rake dev` to start the server

## Back-End Integration

Environment variables are bundled into the WASM file at compile time (see
[`build.rs`](build.rs)). With the intended environment variables specified at compile time,
the production WASM file may be deployed without any further configuration
required.

Note that the `mina-indexer` is a git submodule and is integrated at a specific version
for tier2 tests. Tier2 tests are coded to work against the first 10000 blocks in the
Mina Blockchain. Running `rake tier2` the first time will incur a one-time penalty as the
indexer will be setup on localhost against which tier2 tests are run.

## License

Copyright 2022-2024 Granola Systems Inc.

This software is [licensed](LICENSE) under the Apache License, Version 2.0.

## Contributing

This project uses [C4 (Collective Code Construction Contract)](https://rfc.zeromq.org/spec/42/) process for external contributions.
Additionally, maintainers and select members of [Granola-Team](https://github.com/Granola-Team/) have direct access
and are permitted to push directly to [`main`](https://github.com/Granola-Team/mina-block-explorer/tree/main).
