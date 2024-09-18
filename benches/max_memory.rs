use cgroup_memory::{memory_max, memory_max_unsafe, memory_max_parse, ReadParseError, MEMORY_MAX};
use criterion::{criterion_group, criterion_main, Criterion};
use sysfs_lib::sysfs_read;

/// Reads and parses the memory max file via `std::fs::read_to_string`
fn memory_max_read_to_string() -> Result<u64, ReadParseError> {
    let mmax = std::fs::read_to_string(MEMORY_MAX).map_err(|e| ReadParseError::Io(e))?;

    mmax.trim().parse().map_err(|e| ReadParseError::Parse(e))
}

fn max_memory_sysfncrate_unsafe() -> Option<u64> {
    unsafe { sysfs_read(MEMORY_MAX, memory_max_parse).unwrap().unwrap() }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("max_memory");
    group.bench_function("max_memory_sysfncrate_unsafe", |b| {
        b.iter(|| max_memory_sysfncrate_unsafe())
    });
    group.bench_function("memory_max_unsafe", |b| b.iter(|| memory_max_unsafe()));
    group.bench_function("memory_max", |b| b.iter(|| memory_max()));
    group.bench_function("memory_max_read_to_string", |b| {
        b.iter(|| memory_max_read_to_string())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
