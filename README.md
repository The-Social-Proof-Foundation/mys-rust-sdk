# MySocial Rust SDK

The MySocial Rust SDK for integrating with the [MySocial blockchain](https://docs.mysocial.network/).

> [!NOTE]
> This is project is under development and many features may still be under
> development or missing.

## Overview

This repository contains a collection of libraries for integrating with the MySocial blockchain.

A few of the project's high-level goals are as follows:

* **Be modular** - user's should only need to pay the cost (in terms of dependencies/compilation time) for the features that they use.
* **Be light** - strive to have a minimal dependency footprint.
* **Support developers** - provide all needed types, abstractions and APIs to enable developers to build robust applications on MySocial.
* **Support wasm** - where possible, libraries should be usable in wasm environments.

## Crates

In an effort to be modular, functionality is split between a number of crates.

* [`mys-sdk-types`](crates/mys-sdk-types)
    [![mys-sdk-types on crates.io](https://img.shields.io/crates/v/mys-sdk-types)](https://crates.io/crates/mys-sdk-types)
    [![Documentation (latest release)](https://img.shields.io/badge/docs-latest-brightgreen)](https://docs.rs/mys-sdk-types)
    [![Documentation (master)](https://img.shields.io/badge/docs-master-59f)](https://mystenlabs.github.io/mys-rust-sdk/mys_sdk_types/)
* [`mys-crypto`](crates/mys-crypto)
    [![mys-crypto on crates.io](https://img.shields.io/crates/v/mys-crypto)](https://crates.io/crates/mys-crypto)
    [![Documentation (latest release)](https://img.shields.io/badge/docs-latest-brightgreen)](https://docs.rs/mys-crypto)
    [![Documentation (master)](https://img.shields.io/badge/docs-master-59f)](https://mystenlabs.github.io/mys-rust-sdk/mys_crypto/)
* [`mys-graphql-client`](crates/mys-crypto)
    [![mys-graphql-client on crates.io](https://img.shields.io/crates/v/mys-graphql-client)](https://crates.io/crates/mys-graphql-client)
    [![Documentation (latest release)](https://img.shields.io/badge/docs-latest-brightgreen)](https://docs.rs/mys-graphql-client)
    [![Documentation (master)](https://img.shields.io/badge/docs-master-59f)](https://mystenlabs.github.io/mys-rust-sdk/mys-graphql-client/)

## License

This project is available under the terms of the [Apache 2.0 license](LICENSE).
