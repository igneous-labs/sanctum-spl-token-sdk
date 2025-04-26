# sanctum-spl-token-sdk

SDK for the [spl-token program](https://docs.rs/spl-token/latest/spl_token/).

Currently only implements a subset of what we need for our programs.

## Structure

- `sanctum-spl-token-core` a no-std, minimal dependencies crate defining common base types and procedures portable to different environments (onchain and offchain). All the other crates below build on top of it.
- `sanctum-spl-token-jiminy` CPI bindings for use onchain with [jiminy](https://github.com/igneous-labs/jiminy)

## Development

This section contains dev info for people who wish to work on the library.
