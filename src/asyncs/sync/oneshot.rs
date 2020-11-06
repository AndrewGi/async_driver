use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct RecvError(pub ());

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TryRecvError {
    Empty,
    Closed,
}
#[cfg(feature = "tokio_asyncs")]
pub mod oneshot_impl {
    use super::{RecvError, TryRecvError};
    use core::future::Future;
    use core::pin::Pin;
    use core::task::{Context, Poll};

    pub struct ReceiverImpl<T>(tokio::sync::oneshot::Receiver<T>);
    impl<T> ReceiverImpl<T> {
        pub fn try_recv(&mut self) -> Result<T, TryRecvError> {
            self.0.try_recv().map_err(|e| match e {
                tokio::sync::oneshot::error::TryRecvError::Empty => TryRecvError::Empty,
                tokio::sync::oneshot::error::TryRecvError::Closed => TryRecvError::Closed,
            })
        }
    }
    impl<T> Future for ReceiverImpl<T> {
        type Output = Result<T, RecvError>;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            match Pin::new(&mut self.0).poll(cx) {
                Poll::Ready(r) => Poll::Ready(match r {
                    Ok(t) => Ok(t),
                    Err(_) => Err(RecvError(())),
                }),
                Poll::Pending => Poll::Pending,
            }
        }
    }
    pub struct SenderImpl<T>(tokio::sync::oneshot::Sender<T>);
    impl<T> SenderImpl<T> {
        pub fn send(self, t: T) -> Result<(), T> {
            self.0.send(t)
        }
    }
    pub fn channel<T>() -> (SenderImpl<T>, ReceiverImpl<T>) {
        let (tx, rx) = tokio::sync::oneshot::channel();
        (SenderImpl(tx), ReceiverImpl(rx))
    }
}
pub struct Receiver<T>(oneshot_impl::ReceiverImpl<T>);
impl<T> Receiver<T> {
    pub fn try_recv(&mut self) -> Result<T, TryRecvError> {
        self.0.try_recv()
    }
}
impl<T> Future for Receiver<T> {
    type Output = Result<T, RecvError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.0).poll(cx)
    }
}
pub struct Sender<T>(oneshot_impl::SenderImpl<T>);
impl<T> Sender<T> {
    pub fn send(self, t: T) -> Result<(), T> {
        self.0.send(t)
    }
}
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let (tx, rx) = oneshot_impl::channel();
    (Sender(tx), Receiver(rx))
}
