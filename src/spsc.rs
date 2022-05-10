use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

struct Spsc<const N: usize, T> {
    start: AtomicUsize,
    end: AtomicUsize,
    buffer: [UnsafeCell<MaybeUninit<T>>; N],
}

impl<const N: usize, T> Spsc<N, T> {
    pub fn new() -> Self {
        Self {
            start: AtomicUsize::default(),
            end: AtomicUsize::default(),
            buffer: [UnsafeCell::new(MaybeUninit::uninit()); N],
        }
    }

    pub fn push(&self, value: T) {
        let end = self.end.load(Ordering::Acquire);
        let start = self.start.load(Ordering::Acquire);

        if end != 0 && end == start {}w

        if end < self.buffer.len() {
            let end = unsafe { &self.buffer[end] };
            unsafe { end.get().write(value) };
            self.end.fetch_and(1, Ordering::Release);
        }
    }
}

unsafe impl<const N: usize, T> Sync for Spsc<N, T> {}
unsafe impl<const N: usize, T> Send for Spsc<N, T> {}
