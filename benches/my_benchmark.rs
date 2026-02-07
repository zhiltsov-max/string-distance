use criterion::{criterion_group, criterion_main, Criterion};
use rand;
use std::hint::black_box;
use string_distance;

fn generate_random_string<R: rand::Rng, D: rand::distr::Distribution<char>>(
    length: usize,
    rng: &mut R,
    distribution: &D,
) -> String {
    let mut s: String = String::with_capacity(length * 4).try_into().unwrap();
    for _ in 0..length {
        s.push(rng.sample(distribution));
    }
    return s;
}

struct AlphanumericChar {
    alnum: rand::distr::Alphanumeric,
}

impl rand::distr::Distribution<char> for AlphanumericChar {
    fn sample<R: rand::Rng + ?std::marker::Sized>(&self, rng: &mut R) -> char {
        return char::from(self.alnum.sample(rng));
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let lengths = [16, 64, 128, 256, 512, 1000];

    let ascii_chars = AlphanumericChar {
        alnum: rand::distr::Alphanumeric,
    };
    let utf8_chars = rand::distr::StandardUniform;
    let mut rng = rand::rng();

    for length in lengths {
        c.bench_function(
            &format!(
                "levenshtein for {charset} strings of length {length}",
                charset = "ASCII",
                length = length
            ),
            |b| {
                b.iter(|| {
                    string_distance::levenshtein(
                        black_box(&generate_random_string(length, &mut rng, &ascii_chars)),
                        black_box(&generate_random_string(length, &mut rng, &ascii_chars)),
                    )
                })
            },
        );
    }

    for length in lengths {
        c.bench_function(
            &format!(
                "levenshtein for {charset} strings of length {length}",
                charset = "UTF8",
                length = length
            ),
            |b| {
                b.iter(|| {
                    string_distance::levenshtein(
                        black_box(&generate_random_string(length, &mut rng, &utf8_chars)),
                        black_box(&generate_random_string(length, &mut rng, &utf8_chars)),
                    )
                })
            },
        );
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
