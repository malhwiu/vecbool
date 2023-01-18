use test::{self, Bencher};
use vecbool::VecBool;

// === Functions to be benchmarked ===
fn iter(vec: &VecBool) -> usize {
    vec.iter().filter(|b| *b).count()
}

fn push(values: &[bool]) -> VecBool {
    let mut vecbool = VecBool::new();

    for v in values {
        vecbool.push(*v);
    }

    vecbool
}

fn get(vecbool: &VecBool, indexes: &[usize]) {
    for index in indexes {
        vecbool.get(*index).expect("should contain element");
    }
}

// Benchmark code
#[bench]
fn iter_10_elements(b: &mut Bencher) {
    setup_iter(b, 10);
}

#[bench]
fn iter_1000_elements(b: &mut Bencher) {
    setup_iter(b, 1000);
}

#[bench]
fn iter_1_000_000_elements(b: &mut Bencher) {
    setup_iter(b, 1_000_000);
}

#[bench]
fn push_10_elements(b: &mut Bencher) {
    setup_push(b, 10);
}

#[bench]
fn push_1000_elements(b: &mut Bencher) {
    setup_push(b, 1000);
}

#[bench]
fn push_1_000_000_elements(b: &mut Bencher) {
    setup_push(b, 1_000_000);
}

#[bench]
fn get_10_elements(b: &mut Bencher) {
    setup_get(b, 10);
}

#[bench]
fn get_1000_elements(b: &mut Bencher) {
    setup_get(b, 1000);
}

#[bench]
fn get_1_000_000_elements(b: &mut Bencher) {
    setup_get(b, 1_000_000);
}

fn setup_iter(b: &mut Bencher, size: usize) {
    let values = crate::bench_values(size);
    let vecbool = push(&values);

    b.iter(|| test::black_box(iter(&vecbool)))
}

fn setup_push(b: &mut Bencher, size: usize) {
    let values = crate::bench_values(size);

    b.iter(|| test::black_box(push(&values)))
}

fn setup_get(b: &mut Bencher, size: usize) {
    let values = crate::bench_values(size);
    let vecbool = push(&values);
    let indexes = crate::bench_random_access(size);

    b.iter(|| test::black_box(get(&vecbool, &indexes)))
}
