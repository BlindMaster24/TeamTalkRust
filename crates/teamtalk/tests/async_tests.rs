#![cfg(feature = "async")]

#[cfg(feature = "mock")]
use futures::stream::Stream;
#[cfg(feature = "mock")]
use std::pin::Pin;
#[cfg(feature = "mock")]
use std::sync::Arc;
#[cfg(feature = "mock")]
use std::sync::atomic::{AtomicUsize, Ordering};
#[cfg(feature = "mock")]
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
#[cfg(feature = "mock")]
use teamtalk::Client;
use teamtalk::async_api::AsyncConfig;
#[cfg(feature = "mock")]
use teamtalk::client::backend::MockBackend;

#[test]
fn async_config_defaults() {
    let cfg = AsyncConfig::default();
    assert_eq!(cfg.poll_timeout_ms, 100);
    assert!(cfg.buffer > 0);
}

#[test]
fn async_config_builder() {
    let cfg = AsyncConfig::new().poll_timeout_ms(5).buffer(12);
    assert_eq!(cfg.poll_timeout_ms, 5);
    assert_eq!(cfg.buffer, 12);
}

#[cfg(feature = "mock")]
#[test]
fn async_client_next_event_returns_none_after_stop() {
    let backend = Arc::new(MockBackend::new());
    let client = Client::with_backend(backend).expect("client");
    let mut stream = client.into_async();
    stream.stop();
    let next = futures::executor::block_on(stream.next_event());
    assert!(next.is_none());
}

#[cfg(feature = "mock")]
#[test]
fn async_client_into_client_after_shutdown_returns_client() {
    let backend = Arc::new(MockBackend::new());
    let client = Client::with_backend(backend).expect("client");
    let mut stream = client.into_async();
    stream.shutdown();
    let client = stream.into_client();
    assert!(client.is_some());
}

#[cfg(feature = "mock")]
#[test]
fn async_client_stop_wakes_registered_waker() {
    fn counting_waker(counter: Arc<AtomicUsize>) -> Waker {
        unsafe fn clone(data: *const ()) -> RawWaker {
            let counter = unsafe { Arc::<AtomicUsize>::from_raw(data.cast()) };
            let cloned = Arc::clone(&counter);
            let _ = Arc::into_raw(counter);
            RawWaker::new(Arc::into_raw(cloned).cast(), &VTABLE)
        }
        unsafe fn wake(data: *const ()) {
            let counter = unsafe { Arc::<AtomicUsize>::from_raw(data.cast()) };
            counter.fetch_add(1, Ordering::SeqCst);
        }
        unsafe fn wake_by_ref(data: *const ()) {
            let counter = unsafe { Arc::<AtomicUsize>::from_raw(data.cast()) };
            counter.fetch_add(1, Ordering::SeqCst);
            let _ = Arc::into_raw(counter);
        }
        unsafe fn drop(data: *const ()) {
            let _ = unsafe { Arc::<AtomicUsize>::from_raw(data.cast()) };
        }

        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
        let raw = RawWaker::new(Arc::into_raw(counter).cast(), &VTABLE);
        unsafe { Waker::from_raw(raw) }
    }

    let backend = Arc::new(MockBackend::new());
    let client = Client::with_backend(backend).expect("client");
    let mut stream = client.into_async();
    let wake_count = Arc::new(AtomicUsize::new(0));
    let waker = counting_waker(Arc::clone(&wake_count));
    let mut cx = Context::from_waker(&waker);

    {
        let mut pinned = Pin::new(&mut stream);
        let first = pinned.as_mut().poll_next(&mut cx);
        assert!(matches!(first, Poll::Pending));
        assert_eq!(wake_count.load(Ordering::SeqCst), 0);
    }

    stream.stop();
    assert_eq!(wake_count.load(Ordering::SeqCst), 1);

    let pinned = Pin::new(&mut stream);
    let second = pinned.poll_next(&mut cx);
    assert!(matches!(second, Poll::Ready(None)));
}
