//! `embedded_io_async` wrappers for [`bbqueue`].
//!
//! Adapts bbqueue's stream producer/consumer ends to the standard async I/O
//! traits, in version-specific modules:
//!
//! - [`v0_6`]: wrappers using `embedded-io-async` 0.6
//! - [`v0_7`]: wrappers using `embedded-io-async` 0.7 (default)

#![cfg_attr(not(test), no_std)]

use bbqueue::{
    prod_cons::stream::{StreamConsumer, StreamProducer},
    traits::{bbqhdl::BbqHandle, notifier::AsyncNotifier},
};

/// Wrappers using `embedded-io-async` v0.6.
#[cfg(feature = "io_v0-6")]
pub mod v0_6 {
    use super::*;
    use embedded_io_async::{ErrorType, Read};
    use embedded_io_async_v0_6 as embedded_io_async;

    /// Wraps a bbqueue [`StreamConsumer`] to implement [`Read`].
    pub struct CWrap<Q: BbqHandle> {
        consumer: StreamConsumer<Q>,
    }

    impl<Q: BbqHandle> CWrap<Q> {
        pub fn new(consumer: StreamConsumer<Q>) -> Self {
            Self { consumer }
        }
    }

    impl<Q: BbqHandle> ErrorType for CWrap<Q> {
        type Error = embedded_io_async::ErrorKind;
    }

    impl<Q: BbqHandle> Read for CWrap<Q>
    where
        Q::Notifier: AsyncNotifier,
    {
        async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
            let g = self.consumer.wait_read().await;
            let size = core::cmp::min(g.len(), buf.len());
            buf[..size].copy_from_slice(&g[..size]);
            g.release(size);
            Ok(size)
        }
    }

    /// Wraps a bbqueue [`StreamProducer`] to implement [`embedded_io_async::Write`].
    pub struct PWrap<Q: BbqHandle> {
        producer: StreamProducer<Q>,
    }

    impl<Q: BbqHandle> PWrap<Q> {
        pub fn new(producer: StreamProducer<Q>) -> Self {
            Self { producer }
        }
    }

    impl<Q: BbqHandle> ErrorType for PWrap<Q> {
        type Error = embedded_io_async::ErrorKind;
    }

    impl<Q: BbqHandle> embedded_io_async::Write for PWrap<Q>
    where
        Q::Notifier: AsyncNotifier,
    {
        async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
            let mut g = self.producer.wait_grant_max_remaining(buf.len()).await;
            let size = core::cmp::min(g.len(), buf.len());
            g[..size].copy_from_slice(&buf[..size]);
            g.commit(size);
            Ok(size)
        }

        async fn flush(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
    }
}

/// Wrappers using `embedded-io-async` v0.7.
#[cfg(feature = "io_v0-7")]
pub mod v0_7 {
    use super::*;
    use embedded_io_async::{ErrorType, Read};
    use embedded_io_async_v0_7 as embedded_io_async;

    /// Wraps a bbqueue [`StreamConsumer`] to implement [`Read`].
    pub struct CWrap<Q: BbqHandle> {
        consumer: StreamConsumer<Q>,
    }

    impl<Q: BbqHandle> CWrap<Q> {
        pub fn new(consumer: StreamConsumer<Q>) -> Self {
            Self { consumer }
        }
    }

    impl<Q: BbqHandle> ErrorType for CWrap<Q> {
        type Error = embedded_io_async::ErrorKind;
    }

    impl<Q: BbqHandle> Read for CWrap<Q>
    where
        Q::Notifier: AsyncNotifier,
    {
        async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
            let g = self.consumer.wait_read().await;
            let size = core::cmp::min(g.len(), buf.len());
            buf[..size].copy_from_slice(&g[..size]);
            g.release(size);
            Ok(size)
        }
    }

    /// Wraps a bbqueue [`StreamProducer`] to implement [`embedded_io_async::Write`].
    pub struct PWrap<Q: BbqHandle> {
        producer: StreamProducer<Q>,
    }

    impl<Q: BbqHandle> PWrap<Q> {
        pub fn new(producer: StreamProducer<Q>) -> Self {
            Self { producer }
        }
    }

    impl<Q: BbqHandle> ErrorType for PWrap<Q> {
        type Error = embedded_io_async::ErrorKind;
    }

    impl<Q: BbqHandle> embedded_io_async::Write for PWrap<Q>
    where
        Q::Notifier: AsyncNotifier,
    {
        async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
            let mut g = self.producer.wait_grant_max_remaining(buf.len()).await;
            let size = core::cmp::min(g.len(), buf.len());
            g[..size].copy_from_slice(&buf[..size]);
            g.commit(size);
            Ok(size)
        }

        async fn flush(&mut self) -> Result<(), Self::Error> {
            Ok(())
        }
    }
}
