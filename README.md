## bbqio

[`embedded_io_async`] wrappers for [`bbqueue`].

Adapts bbqueue's stream producer/consumer ends to the standard
[`embedded_io_async::Read`] and [`embedded_io_async::Write`] traits,
so they can be used anywhere those traits are expected.
Works in `no_std` / `no_alloc` environments.

| Type | Wraps | Implements |
|------|-------|------------|
| `CWrap<Q>` | `StreamConsumer<Q>` | `embedded_io_async::Read` |
| `PWrap<Q>` | `StreamProducer<Q>` | `embedded_io_async::Write` |

[`embedded_io_async`]: https://crates.io/crates/embedded-io-async
[`bbqueue`]: https://crates.io/crates/bbqueue
[`embedded_io_async::Read`]: https://docs.rs/embedded-io-async/latest/embedded_io_async/trait.Read.html
[`embedded_io_async::Write`]: https://docs.rs/embedded-io-async/latest/embedded_io_async/trait.Write.html
