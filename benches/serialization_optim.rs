///! Benchmark for serialization optimizations (JSON vs MessagePack)

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use magneto_serge::cassette::{Cassette, HttpRequest, HttpResponse, InteractionKind};
use std::collections::HashMap;

/// Helper to create test cassette with N interactions
fn create_test_cassette(name: &str, interactions: usize) -> Cassette {
    let mut cassette = Cassette::new(name.to_string());

    for i in 0..interactions {
        let request = HttpRequest {
            method: "GET".to_string(),
            url: format!("https://api.example.com/resource/{}", i),
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type".to_string(), "application/json".to_string());
                h.insert("Authorization".to_string(), format!("Bearer token-{}", i));
                h
            },
            body: Some(format!("{{\"request\": {}, \"data\": \"test data\"}}", i).into_bytes()),
        };

        let response = HttpResponse {
            status: 200,
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type".to_string(), "application/json".to_string());
                h
            },
            body: Some(format!("{{\"response\": {}, \"result\": [1,2,3,4,5], \"message\": \"success\"}}", i).into_bytes()),
        };

        cassette.add_interaction(InteractionKind::Http { request, response });
    }

    cassette
}

/// Benchmark: JSON serialization (baseline)
fn bench_json_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialization_json");

    for size in [1, 10, 50, 100, 500].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, &size| {
                let cassette = create_test_cassette("json-bench", size);

                b.iter(|| {
                    let bytes = serde_json::to_vec(&cassette).unwrap();
                    black_box(bytes);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark: JSON deserialization
fn bench_json_deserialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("deserialization_json");

    for size in [1, 10, 50, 100, 500].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        let cassette = create_test_cassette("json-bench", *size);
        let bytes = serde_json::to_vec(&cassette).unwrap();

        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &bytes,
            |b, bytes| {
                b.iter(|| {
                    let cassette: Cassette = serde_json::from_slice(bytes).unwrap();
                    black_box(cassette);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark: MessagePack serialization (optimized)
#[cfg(feature = "msgpack")]
fn bench_msgpack_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialization_msgpack");

    for size in [1, 10, 50, 100, 500].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, &size| {
                let cassette = create_test_cassette("msgpack-bench", size);

                b.iter(|| {
                    let bytes = rmp_serde::to_vec(&cassette).unwrap();
                    black_box(bytes);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark: MessagePack deserialization
#[cfg(feature = "msgpack")]
fn bench_msgpack_deserialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("deserialization_msgpack");

    for size in [1, 10, 50, 100, 500].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        let cassette = create_test_cassette("msgpack-bench", *size);
        let bytes = rmp_serde::to_vec(&cassette).unwrap();

        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &bytes,
            |b, bytes| {
                b.iter(|| {
                    let cassette: Cassette = rmp_serde::from_slice(bytes).unwrap();
                    black_box(cassette);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark: File size comparison
#[cfg(feature = "msgpack")]
fn bench_file_size_comparison(c: &mut Criterion) {
    println!("\n=== File Size Comparison (JSON vs MessagePack) ===\n");

    for size in [10, 50, 100, 500].iter() {
        let cassette = create_test_cassette("size-comparison", *size);

        let json_bytes = serde_json::to_vec(&cassette).unwrap();
        let msgpack_bytes = rmp_serde::to_vec(&cassette).unwrap();

        let json_size = json_bytes.len();
        let msgpack_size = msgpack_bytes.len();
        let compression_ratio = ((json_size as f64 - msgpack_size as f64) / json_size as f64) * 100.0;

        println!("{} interactions:", size);
        println!("  JSON:        {:>8} bytes", json_size);
        println!("  MessagePack: {:>8} bytes ({:>5.1}% smaller)", msgpack_size, compression_ratio);
        println!("  Speedup:     {:.2}x smaller\n", json_size as f64 / msgpack_size as f64);
    }

    // Dummy benchmark to satisfy Criterion
    let mut group = c.benchmark_group("file_size_comparison");
    group.bench_function("dummy", |b| b.iter(|| black_box(1)));
    group.finish();
}

#[cfg(feature = "msgpack")]
criterion_group!(
    benches,
    bench_json_serialization,
    bench_json_deserialization,
    bench_msgpack_serialization,
    bench_msgpack_deserialization,
    bench_file_size_comparison,
);

#[cfg(not(feature = "msgpack"))]
criterion_group!(
    benches,
    bench_json_serialization,
    bench_json_deserialization,
);

criterion_main!(benches);
