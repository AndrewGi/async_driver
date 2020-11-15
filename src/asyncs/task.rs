use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
#[cfg(feature = "tokio_asyncs")]
pub mod task_impl {
    use super::{Context, Future, Pin, Poll};

    pub struct JoinHandleImpl<T>(tokio::task::JoinHandle<T>);
    impl<T> Future for JoinHandleImpl<T> {
        type Output = T;

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            match unsafe { self.map_unchecked_mut(|s| &mut s.0) }.poll(cx) {
                Poll::Ready(r) => Poll::Ready(r.expect("task join failed")),
                Poll::Pending => Poll::Pending,
            }
        }
    }
    pub fn spawn<T: Send + 'static, F: Future<Output = T> + Send + 'static>(
        future: F,
    ) -> JoinHandleImpl<T> {
        JoinHandleImpl(tokio::task::spawn(future))
    }
    pub fn spawn_local<T: 'static, F: Future<Output = T> + 'static>(
        future: F,
    ) -> JoinHandleImpl<T> {
        JoinHandleImpl(tokio::task::spawn_local(future))
    }

    pub fn block_in_place<R, F: FnOnce() -> R>(f: F) -> R {
        tokio::task::block_in_place(f)
    }
}
pub fn spawn<T: Send + 'static, F: Future<Output = T> + Send + 'static>(
    future: F,
) -> JoinHandle<T> {
    JoinHandle(task_impl::spawn(future))
}
pub fn spawn_local<T: 'static, F: Future<Output = T> + 'static>(future: F) -> JoinHandle<T> {
    JoinHandle(task_impl::spawn_local(future))
}
pub fn block_in_place<R, F: FnOnce() -> R>(f: F) -> R {
    task_impl::block_in_place(f)
}
pub struct JoinHandle<T>(task_impl::JoinHandleImpl<T>);
impl<T> Future for JoinHandle<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        unsafe { self.map_unchecked_mut(|s| &mut s.0) }.poll(cx)
    }
}
/// Yield the task back to the executor. Just returns `Poll::Pending` once and calls
/// `.waker_by_ref()` to put the task back onto the queue. Workaround for blocking futures
pub async fn yield_now() {
    struct YieldNow {
        yielded: bool,
    }

    impl Future for YieldNow {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
            if self.yielded {
                return Poll::Ready(());
            }

            self.yielded = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }

    YieldNow { yielded: false }.await
}
