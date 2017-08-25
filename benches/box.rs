#![feature(test)]
extern crate test;
use test::Bencher;


#[bench]
fn boxed(b: &mut Bencher) {
    b.iter(|| Box::new(42));
}


/// It takes 0ns
#[bench]
fn plain(b: &mut Bencher) {
    b.iter(|| 42)
}


#[bench]
fn vec(b: &mut Bencher) {
    b.iter(|| vec![42, 42, 42])
}


/// It takes 0ns
#[bench]
fn empty_vec(b: &mut Bencher) {
    b.iter(|| Vec::<u8>::new())
}


/// It takes 0ns
#[bench]
fn array(b: &mut Bencher) {
    b.iter(|| [42, 42, 42])
}
