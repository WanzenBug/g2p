use criterion::{BatchSize, BenchmarkId, Criterion, criterion_group, criterion_main};
use galois_2p8;
use galois_2p8::Field;
use rand;
use rand::{Rng, RngCore};
use reed_solomon_erasure;

use g2p;

g2p::g2p!(GF256, 8);

fn g2p_addition(a: &[u8], b: &[u8], dest: &mut [u8]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(b.len(), dest.len());

    for ((&l, &r), d) in Iterator::zip(Iterator::zip(a.into_iter(), b), dest) {
        *d = (GF256::from(l) + GF256::from(r)).into()
    }
}

fn galois_2p8_addition(field: &galois_2p8::PrimitivePolynomialField, a: &[u8], b: &[u8], dest: &mut [u8]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(b.len(), dest.len());

    field.add_multiword(dest, a);
    field.add_multiword(dest, b);
}

fn reed_solomon_erasure_addition(a: &[u8], b: &[u8], dest: &mut [u8]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(b.len(), dest.len());

    for ((&l, &r), d) in Iterator::zip(Iterator::zip(a.into_iter(), b), dest) {
        *d = reed_solomon_erasure::galois_8::add(l, r);
    }
}

fn g2p_multiplication(a: &[u8], b: &[u8], dest: &mut [u8]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(b.len(), dest.len());

    for ((&l, &r), d) in Iterator::zip(Iterator::zip(a.into_iter(), b), dest) {
        *d = (GF256::from(l) * GF256::from(r)).into()
    }
}

fn galois_2p8_multiplication(field: &galois_2p8::PrimitivePolynomialField, a: &[u8], b: &[u8], dest: &mut [u8]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(b.len(), dest.len());

    for ((&l, &r), d) in Iterator::zip(Iterator::zip(a.into_iter(), b), dest) {
        *d = field.mult(l, r)
    }
}

fn reed_solomon_erasure_multiplication(a: &[u8], b: &[u8], dest: &mut [u8]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(b.len(), dest.len());

    for ((&l, &r), d) in Iterator::zip(Iterator::zip(a.into_iter(), b), dest) {
        *d = reed_solomon_erasure::galois_8::mul(l, r);
    }
}

fn g2p_multiplication_const(a: &[u8], b: u8, dest: &mut [u8]) {
    assert_eq!(a.len(), dest.len());

    for (&l, d) in Iterator::zip(a.into_iter(), dest) {
        *d = (GF256::from(l) * GF256::from(b)).into()
    }
}

fn galois_2p8_multiplication_const(field: &galois_2p8::PrimitivePolynomialField, a: &[u8], b: u8, dest: &mut [u8]) {
    assert_eq!(a.len(), dest.len());

    dest.copy_from_slice(a);
    field.mult_multiword(dest, b);
}

fn reed_solomon_erasure_multiplication_const(a: &[u8], b: u8, dest: &mut [u8]) {
    assert_eq!(a.len(), dest.len());

    reed_solomon_erasure::galois_8::mul_slice(b, a, dest);
}

fn g2p_division(a: &[u8], b: &[u8], dest: &mut [u8]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(b.len(), dest.len());

    for ((&l, &r), d) in Iterator::zip(Iterator::zip(a.into_iter(), b), dest) {
        *d = (GF256::from(l) / GF256::from(r)).into()
    }
}

fn galois_2p8_division(field: &galois_2p8::PrimitivePolynomialField, a: &[u8], b: &[u8], dest: &mut [u8]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(b.len(), dest.len());

    for ((&l, &r), d) in Iterator::zip(Iterator::zip(a.into_iter(), b), dest) {
        *d = field.div(l, r)
    }
}

fn reed_solomon_erasure_division(a: &[u8], b: &[u8], dest: &mut [u8]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(b.len(), dest.len());

    for ((&l, &r), d) in Iterator::zip(Iterator::zip(a.into_iter(), b), dest) {
        *d = reed_solomon_erasure::galois_8::div(l, r);
    }
}

fn g2p_division_const(a: &[u8], b: u8, dest: &mut [u8]) {
    assert_eq!(a.len(), dest.len());

    for (&l, d) in Iterator::zip(a.into_iter(), dest) {
        *d = (GF256::from(l) / GF256::from(b)).into()
    }
}

fn galois_2p8_division_const(field: &galois_2p8::PrimitivePolynomialField, a: &[u8], b: u8, dest: &mut [u8]) {
    assert_eq!(a.len(), dest.len());

    dest.copy_from_slice(a);
    field.div_multiword(dest, b);
}

fn reed_solomon_erasure_division_const(a: &[u8], b: u8, dest: &mut [u8]) {
    assert_eq!(a.len(), dest.len());

    for (&l, d) in Iterator::zip(a.into_iter(), dest) {
        *d = reed_solomon_erasure::galois_8::div(l, b);
    }
}


fn g2p_inverse(a: &[u8], dest: &mut [u8]) {
    assert_eq!(a.len(), dest.len());

    for (&inv, d) in Iterator::zip(a.into_iter(), dest) {
        *d = (GF256::from(1) / GF256::from(inv)).into()
    }
}

fn galois_2p8_inverse(field: &galois_2p8::PrimitivePolynomialField, a: &[u8], dest: &mut [u8]) {
    assert_eq!(a.len(), dest.len());

    for (&inv, d) in Iterator::zip(a.into_iter(), dest) {
        *d = field.div(1, inv);
    }
}

fn reed_solomon_erasure_inverse(a: &[u8], dest: &mut [u8]) {
    assert_eq!(a.len(), dest.len());

    for (&inv, d) in Iterator::zip(a.into_iter(), dest) {
        *d = reed_solomon_erasure::galois_8::div(1, inv);
    }
}


fn all_benches(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let galois_2p8_field = galois_2p8::PrimitivePolynomialField::new_might_panic(galois_2p8::IrreducablePolynomial::Poly84320);

    let input_sizes = [64, 1_024, 16_384];

    let mut group = c.benchmark_group("addition");
    for &i in input_sizes.iter() {
        let mut a = vec![0; i];
        let mut b = vec![0; i];
        let dest = vec![0; i];
        rng.fill_bytes(&mut a[..]);
        rng.fill_bytes(&mut b[..]);

        group.bench_function(
            BenchmarkId::new("g2p", i),
            |bencher| {
                bencher.iter_batched(
                    || (a.clone(), b.clone(), dest.clone()),
                    |(a, b, mut dest)| {
                        g2p_addition(&a, &b, &mut dest);
                        dest
                    },
                    BatchSize::SmallInput,
                )
            });
        group.bench_function(
            BenchmarkId::new("galois_2p8", i),
            |bencher| {
                bencher.iter_batched(
                    || (a.clone(), b.clone(), dest.clone()),
                    |(a, b, mut dest)| {
                        galois_2p8_addition(&galois_2p8_field, &a, &b, &mut dest);
                        dest
                    },
                    BatchSize::SmallInput,
                )
            });
        group.bench_function(
            BenchmarkId::new("reed_solomon_erasure", i),
            |bencher| {
                bencher.iter_batched(
                    || (a.clone(), b.clone(), dest.clone()),
                    |(a, b, mut dest)| {
                        reed_solomon_erasure_addition(&a, &b, &mut dest);
                        dest
                    },
                    BatchSize::SmallInput,
                )
            });
    }
    group.finish();

    let mut group = c.benchmark_group("multiplication");
    for &i in input_sizes.iter() {
        let mut a = vec![0; i];
        let mut b = vec![0; i];
        let dest = vec![0; i];
        rng.fill_bytes(&mut a[..]);
        rng.fill_bytes(&mut b[..]);

        group.bench_function(
            BenchmarkId::new("g2p", i),
            |bencher| {
                bencher.iter_batched(
                    || (a.clone(), b.clone(), dest.clone()),
                    |(a, b, mut dest)| {
                        g2p_multiplication(&a, &b, &mut dest);
                        dest
                    },
                    BatchSize::SmallInput,
                )
            });
        group.bench_function(
            BenchmarkId::new("galois_2p8", i),
            |bencher| {
                bencher.iter_batched(
                    || (a.clone(), b.clone(), dest.clone()),
                    |(a, b, mut dest)| {
                        galois_2p8_multiplication(&galois_2p8_field, &a, &b, &mut dest);
                        dest
                    },
                    BatchSize::SmallInput,
                )
            });
        group.bench_function(
            BenchmarkId::new("reed_solomon_erasure", i),
            |bencher| {
                bencher.iter_batched(
                    || (a.clone(), b.clone(), dest.clone()),
                    |(a, b, mut dest)| {
                        reed_solomon_erasure_multiplication(&a, &b, &mut dest);
                        dest
                    },
                    BatchSize::SmallInput,
                )
            });
    }
    group.finish();

    let mut group = c.benchmark_group("multiplication_const");
    for &i in input_sizes.iter() {
        let mut a = vec![0; i];
        let b = rng.gen();
        let dest = vec![0; i];
        rng.fill_bytes(&mut a[..]);

        group.bench_function(
            BenchmarkId::new("g2p", i),
            |bencher| {
                bencher.iter_batched(
                    || (a.clone(), dest.clone()),
                    |(a, mut dest)| {
                        g2p_multiplication_const(&a, b, &mut dest);
                        dest
                    },
                    BatchSize::SmallInput,
                )
            });
        group.bench_function(
            BenchmarkId::new("galois_2p8", i),
            |bencher| {
                bencher.iter_batched(
                    || (a.clone(), dest.clone()),
                    |(a, mut dest)| {
                        galois_2p8_multiplication_const(&galois_2p8_field, &a, b, &mut dest);
                        dest
                    },
                    BatchSize::SmallInput,
                )
            });
        group.bench_function(
            BenchmarkId::new("reed_solomon_erasure", i),
            |bencher| {
                bencher.iter_batched(
                    || (a.clone(), dest.clone()),
                    |(a, mut dest)| {
                        reed_solomon_erasure_multiplication_const(&a, b, &mut dest);
                        dest
                    },
                    BatchSize::SmallInput,
                )
            });
    }
    group.finish();

    let mut group = c.benchmark_group("inverse");
    for &i in input_sizes.iter() {
        let mut a = vec![0; i];
        let dest = vec![0; i];
        rng.fill_bytes(&mut a[..]);

        for divisor in &mut a {
            while *divisor == 0 {
                *divisor = rng.gen();
            }
        }

        group.bench_function(
            BenchmarkId::new("g2p", i),
            |bencher| {
                bencher.iter_batched(
                    || (a.clone(), dest.clone()),
                    |(a, mut dest)| {
                        g2p_inverse(&a, &mut dest);
                        dest
                    },
                    BatchSize::SmallInput,
                )
            });
        group.bench_function(
            BenchmarkId::new("galois_2p8", i),
            |bencher| {
                bencher.iter_batched(
                    || (a.clone(), dest.clone()),
                    |(a, mut dest)| {
                        galois_2p8_inverse(&galois_2p8_field, &a, &mut dest);
                        dest
                    },
                    BatchSize::SmallInput,
                )
            });
        group.bench_function(
            BenchmarkId::new("reed_solomon_erasure", i),
            |bencher| {
                bencher.iter_batched(
                    || (a.clone(), dest.clone()),
                    |(a, mut dest)| {
                        reed_solomon_erasure_inverse(&a, &mut dest);
                        dest
                    },
                    BatchSize::SmallInput,
                )
            });
    }
    group.finish();

    let mut group = c.benchmark_group("division");
    for &i in input_sizes.iter() {
        let mut a = vec![0; i];
        let mut b = vec![0; i];
        let dest = vec![0; i];
        rng.fill_bytes(&mut a[..]);
        rng.fill_bytes(&mut b[..]);

        for divisor in &mut b {
            while *divisor == 0 {
                *divisor = rng.gen();
            }
        }

        group.bench_function(
            BenchmarkId::new("g2p", i),
            |bencher| {
                bencher.iter_batched(
                    || (a.clone(), b.clone(), dest.clone()),
                    |(a, b, mut dest)| {
                        g2p_division(&a, &b, &mut dest);
                        dest
                    },
                    BatchSize::SmallInput,
                )
            });
        group.bench_function(
            BenchmarkId::new("galois_2p8", i),
            |bencher| {
                bencher.iter_batched(
                    || (a.clone(), b.clone(), dest.clone()),
                    |(a, b, mut dest)| {
                        galois_2p8_division(&galois_2p8_field, &a, &b, &mut dest);
                        dest
                    },
                    BatchSize::SmallInput,
                )
            });
        group.bench_function(
            BenchmarkId::new("reed_solomon_erasure", i),
            |bencher| {
                bencher.iter_batched(
                    || (a.clone(), b.clone(), dest.clone()),
                    |(a, b, mut dest)| {
                        reed_solomon_erasure_division(&a, &b, &mut dest);
                        dest
                    },
                    BatchSize::SmallInput,
                )
            });
    }
    group.finish();

    let mut group = c.benchmark_group("division_const");
    for &i in input_sizes.iter() {
        let mut a = vec![0; i];
        let b = rng.gen_range(1..=255);
        let dest = vec![0; i];
        rng.fill_bytes(&mut a[..]);

        group.bench_function(
            BenchmarkId::new("g2p", i),
            |bencher| {
                bencher.iter_batched(
                    || (a.clone(), dest.clone()),
                    |(a, mut dest)| {
                        g2p_division_const(&a, b, &mut dest);
                        dest
                    },
                    BatchSize::SmallInput,
                )
            });
        group.bench_function(
            BenchmarkId::new("galois_2p8", i),
            |bencher| {
                bencher.iter_batched(
                    || (a.clone(), dest.clone()),
                    |(a, mut dest)| {
                        galois_2p8_division_const(&galois_2p8_field, &a, b, &mut dest);
                        dest
                    },
                    BatchSize::SmallInput,
                )
            });
        group.bench_function(
            BenchmarkId::new("reed_solomon_erasure", i),
            |bencher| {
                bencher.iter_batched(
                    || (a.clone(), dest.clone()),
                    |(a, mut dest)| {
                        reed_solomon_erasure_division_const(&a, b, &mut dest);
                        dest
                    },
                    BatchSize::SmallInput,
                )
            });
    }
    group.finish();
}

criterion_group!(benches, all_benches);
criterion_main!(benches);
