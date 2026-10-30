use divan;

fn main() {
  divan::main();
}

const SIZE: usize = 10;
const fn sizes() -> [u64; SIZE] {
  let mut arr: [u64; SIZE] = [0; SIZE];
  let mut i = 0;
  while i < SIZE {
    arr[i] = 2u64.pow((i) as u32);
    i += 1;
  }
  arr
}

#[divan::bench(consts = sizes())]
fn bench<const N: u64>(bencher: divan::Bencher) {
  bencher
    .with_inputs(|| {
      (
        gap_2::generator::generate_random_tree(N),
        gap_2::generator::generate_random_tree(N),
      )
    })
    .bench_values(|input| {
      gap_2::algo(divan::black_box(&input.0), divan::black_box(&input.1));
    });
}
