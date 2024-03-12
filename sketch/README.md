# minisketch-rs

`minisketch-rs` is a wrapper around [minisketch](https://github.com/sipa/minisketch),
a C library by [Pieter Wuille](https://github.com/sipa) for efficient set reconciliation.

> minisketch is proposed as a part of an [Erlay](https://arxiv.org/abs/1905.10518) technique for bandwidth-efficient TX propagation in Bitcoin.

This library exposes type-safe Rust bindings to all `minisketch` functions by providing `Minisketch` structure.

## Examples

See the [examples](examples).
