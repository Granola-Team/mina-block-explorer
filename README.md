# Mina Block Explorer

[![Build status](https://badge.buildkite.com/1f8c338cb4ede4e41a4d84de89479fb2eddf9a6f64b72dcf36.svg)](https://buildkite.com/granola/mina-block-explorer)

The Mina Block Explorer is a user-friendly website for exploring the
Mina Blockchain. It offers a straightforward interface to view details
about blocks, transactions, addresses, and validators, simplifying
interaction with the network.

## Getting Started

This project utilizes Nix to ensure consistent and reproducible
development environment. This choice streamlines our development
process, allowing us to focus on delivering value with minimal concern
for varying system configurations or dependency conflicts.

### Installing Nix

1. Install Nix: Visit [Nix download
   page](https://nixos.org/download.html) and follow the installation
   instructions.

2. Enable Nix Flakes: Refer to the [Nix Flakes
   guide](https://nixos.wiki/wiki/Flakes) for enabling Flakes in Nix.

No additional dependencies are needed beyond this setup.

## Quick Start Guide

1. Setup Environment: Run `nix develop` in your terminal to prepare your
   development environment.

2. Start the Server: Execute `just serve` to start the server. Execute `just tailwind-watch` in another terminal.

These two steps will get your environment ready and your server running quickly.

## License (See LICENSE file for full license)

Copyright 2023 Mina Foundation, Inc.

Free use of this software is granted under the terms of the Mozilla
Public License 2.0.

## Contributing

This project uses [C4(Collective Code Construction
Contract)](https://rfc.zeromq.org/spec/42/) process for contributions.
