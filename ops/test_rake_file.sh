#!/bin/bash

# Exit on any command failure
set -e

# Run commands in sequence, stopping on first failure
rake clean                  # Clean the repo of built artifacts
rake audit                  # Audit the Rust code with cargo-audit
rake release_build                  # Build the front-end WASM bundle
rake dev_build                  # Build the front-end WASM bundle
rake build_docs             # Build documentation in the home directory
rake check                  # Use 'cargo check' to verify buildability
rake default                # Default task - print the menu of targets
rake format                 # Format the source code
rake jest_test              # Run the Jest tests
rake lint                   # Lint all source code
rake lint_fix               # Fix linting errors
rake lint_ruby              # Lint the Ruby code
rake lint_rust              # Lint the Rust code
rake pnpm_install           # Install the JavaScript dependencies with 'pnpm'
rake rust_test              # Test the Rust code
rake test_unit              # Run the unit tests
rake tier1                  # Run the Tier1 tests
rake deploy_mina_indexer    # Deploy mina-indexer
rake shutdown_mina_indexer  # Shut down mina-indexer
rake tier2                  # Invoke the Tier2 regression suite

echo "All tasks completed successfully!"
