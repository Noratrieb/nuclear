use std::{
    cell::UnsafeCell,
    error::Error,
    fmt::{Display, Formatter, Write},
    mem::MaybeUninit,
    sync::{
        atomic::{AtomicPtr, AtomicUsize, Ordering},
        Arc,
    },
};

const BUF_SIZE: usize = 16;

/// A fancy ring buffer
struct Spsc<T> {
    /// The first element in the buffer
    start: AtomicUsize,
    /// A pointer *at* the last element in the buffer. usize::MAX when it's empty,
    /// in which case the next element will be put directly at `start`
    end: AtomicUsize,
    buffer: [UnsafeCell<MaybeUninit<T>>; BUF_SIZE],
}

impl<T> Spsc<T> {
    fn new() -> Self {
        Self {
            start: AtomicUsize::default(),
            end: AtomicUsize::new(usize::MAX),
            buffer: [UnsafeCell::new(MaybeUninit::uninit()); BUF_SIZE],
        }
    }

    fn try_send(&self, value: T) -> Result<(), errors::QueueFullError> {
        let end = self.end.load(Ordering::Acquire);
        let start = self.start.load(Ordering::Acquire);

        let idx = if end == usize::MAX { start } else { end };

        if end != usize::MAX && end + 1 != start {
            Err(errors::QueueFullError)
        } else {
            unsafe { self.write(value, idx) };
            self.end.store(idx, Ordering::Release);
            Ok(())
        }
    }

    unsafe fn read(&self, index: usize) -> T {
        self.buffer[index].get().read()
    }

    unsafe fn write(&self, value: T, index: usize) {
        self.buffer[index].get().write(value)
    }
}

pub struct Producer<T> {
    queue: Arc<Spsc<T>>,
}

impl<T> Producer<T> {
    pub fn try_send(&self, value: T) -> Result<(), errors::QueueFullError> {
        self.queue.try_send(value)
    }
}

pub struct Consumer<T> {
    queue: Arc<Spsc<T>>,
}

impl<T> Consumer<T> {}

unsafe impl<T> Sync for Spsc<T> {}
unsafe impl<T> Send for Spsc<T> {}

mod errors {
    use std::{
        error::Error,
        fmt::{Display, Formatter},
    };

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
    pub struct QueueFullError;

    impl Display for QueueFullError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str("spsc queue is full")
        }
    }

    impl Error for QueueFullError {}

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
    pub struct QueueEmptyError;

    impl Display for QueueEmptyError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str("spsc queue is empty")
        }
    }

    impl Error for QueueEmptyError {}
}
