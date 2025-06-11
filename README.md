# Simple Cancelation Token

[![coverage](https://shields.io/endpoint?url=https://raw.githubusercontent.com/jlyonsmith/simple_cancelation_token/main/coverage.json)](https://github.com/jlyonsmith/simple_cancelation_token/blob/main/coverage.json)
[![Crates.io](https://img.shields.io/crates/v/simple_cancelation_token.svg)](https://crates.io/crates/simple_cancelation_token)
[![Docs.rs](https://docs.rs/simple_cancelation_token/badge.svg)](https://docs.rs/simple_cancelation_token)

## Summary

This is a very simple implementation of an inter-thread cancelation token. It is modeled after [tokio_util::sync::CancelationToken](https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancelationToken.html) and uses cloning to pass to other threads vs. having multiple separate structs with different functions. It's for when you don't need all of `tokio` and you just want signal worker thread(s) to stop, for example after hitting Ctrl+C

Plus, it also uses the [American spelling of cancelation](https://www.grammarpalette.com/cancelation-vs-cancelation-which-spelling/) just to be different.
