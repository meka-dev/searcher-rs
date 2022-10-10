# zenith-rs

![Github Actions](https://github.com/meka-dev/zenith-rs/workflows/CI/badge.svg)
[![Crates.io][crates-badge]][crates-url]

[crates-badge]: https://img.shields.io/crates/v/zenith-rs.svg
[crates-url]: https://crates.io/crates/zenith-rs

[Zenith](https://meka.tech/zenith) is the block space market for the interchain
and this crate provides tools to interact with it.

The crate is limited to the demand/searcher side. This means querying auctions
and submitting bids.

To use `zenith-rs` add it to your `[dependencies]`

```toml
[dependencies]

zenith-rs = { version = "0.1",  default-features = false }
```

## Contributing

Please see [`CONTRIBUTING.md`](https://github.com/meka-dev/zenith-rs/blob/main/CONTRIBUTING.md) for details.
