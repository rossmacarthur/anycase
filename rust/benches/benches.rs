use criterion::{criterion_group, criterion_main, Criterion};

criterion_main! { benches }

criterion_group! { benches, bench_simple }

pub fn bench_simple(c: &mut Criterion) {
    let mut g = c.benchmark_group("simple");

    let input = "thisIsACamelCaseString".repeat(1000);
    let expect = "this_is_a_camel_case_string".repeat(1000);

    assert_eq!(anycase::to_snake(&input), expect);
    assert_eq!(anycase::as_snake(&input).to_string(), expect);

    g.bench_with_input("string", &input, |b, input| {
        b.iter(|| anycase::to_snake(input));
    });

    g.bench_with_input("fmt", &input, |b, input| {
        b.iter(|| anycase::as_snake(input).to_string());
    });
}
