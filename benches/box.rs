#![feature(test)]
extern crate test;
use test::Bencher;


/// 20ns
#[bench]
fn boxed(b: &mut Bencher) {
    b.iter(|| Box::new(42));
}


/// 0ns
#[bench]
fn plain(b: &mut Bencher) {
    b.iter(|| 42)
}


/// 20ns
#[bench]
fn vec(b: &mut Bencher) {
    b.iter(|| vec![42; 3])
}


/// 0ns
#[bench]
fn empty_vec(b: &mut Bencher) {
    b.iter(|| Vec::<u8>::new())
}


/// 0ns
#[bench]
fn vec_with_capacity(b: &mut Bencher) {
    b.iter(|| Vec::<usize>::with_capacity(1))
}


/// 0ns
#[bench]
fn array(b: &mut Bencher) {
    b.iter(|| [42, 42, 42])
}


/// 27ns
#[bench]
fn string(b: &mut Bencher) {
    b.iter(|| String::from("HOGEHOGE"))
}


/// 0ns
#[bench]
fn str(b: &mut Bencher) {
    b.iter(|| "HOGEHOGE")
}
