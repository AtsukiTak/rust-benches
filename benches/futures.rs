#![feature(test)]
extern crate test;
use test::Bencher;

extern crate futures;

use futures::{Stream, sync, unsync};


#[bench]
fn sync_sender(b: &mut Bencher) {
    let (sender, receiver) = sync::mpsc::unbounded();
    b.iter(move || sender.send(42));
    receiver.wait();
}


#[bench]
fn unsync_sender(b: &mut Bencher) {
    let (sender, receiver) = unsync::mpsc::unbounded();
    b.iter(move || sender.send(42));
    receiver.wait();
}


#[bench]
fn sync_oneshot(b: &mut Bencher) {
    b.iter(|| {
        let (sender, receiver) = sync::oneshot::channel();
        sender.send(42);
    });
}


#[bench]
fn unsync_oneshot(b: &mut Bencher) {
    b.iter(|| {
        let (sender, receiver) = unsync::oneshot::channel();
        sender.send(42);
    });
}
