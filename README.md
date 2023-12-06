# Mina Block Explorer

[![Build status](https://badge.buildkite.com/1f8c338cb4ede4e41a4d84de89479fb2eddf9a6f64b72dcf36.svg)](https://buildkite.com/granola/mina-block-explorer)

The Mina Block Explorer is a website that allows users to explore and
view details about blocks, transactions, addresses, validators and
other relevant network information. It provides a user-friendly user
interface to interact with the Mina blockchain. This document
describes how to build such an application.

## Quickstart

* Install Rust 
```
curl https://sh.rustup.rs -sSf | sh
source "$HOME/.cargo/env"
```

* Serve app 
```
cd app 
cargo build
trunk serve --open
```

* Playwright Tests
```
cd app
npx playwright test
```

## Nix flakes

This project utilizes Nix Flakes for development and building. Install
Nix [here](https://nixos.org/download.html) and enable Flakes using
the guide [here](https://nixos.wiki/wiki/Flakes). No additional
dependencies are needed.

### Setting Up the Development Environment

Create your development environment using `nix develop`. This prepares
your system with the necessary dependencies, compilers, and
development tools, eliminating the need for an independent Rust
installation. For VSCode, enhance your experience with the `Nix
Environment Selector` extension by linking it to
`shell.nix`. Alternatively, configure your IDE environment with
`direnv`.

## License (See LICENSE file for full license)

Copyright 2023 Mina Foundation, Inc.

Free use of this software is granted under the terms of the Mozilla
Public License 2.0.

## Contributing

This project uses [C4(Collective Code Construction
Contract)](https://rfc.zeromq.org/spec/42/) process for contributions.
