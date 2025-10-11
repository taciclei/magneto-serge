use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use magneto_serge::{MagnetoProxy, ProxyMode};
use std::time::Duration;
use tempfile::TempDir;

/// Helper to create a test proxy instance
fn create_test_proxy(mode: ProxyMode) -> (MagnetoProxy, TempDir) {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let cassette_dir = temp_dir.path().to_path_buf();

    let proxy = MagnetoProxy::new_internal(&cassette_dir).expect("Failed to create proxy");

    proxy.set_mode(mode);
    proxy.set_port(0); // Use random available port

    (proxy, temp_dir)
}

/// Benchmark: Create and configure proxy instance
fn bench_proxy_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("proxy_creation");

    group.bench_function("new_proxy", |b| {
        b.iter(|| {
            let temp_dir = TempDir::new().expect("Failed to create temp dir");
            let cassette_dir = temp_dir.path().to_path_buf();
            black_box(MagnetoProxy::new_internal(&cassette_dir).expect("Failed to create proxy"));
        });
    });

    group.bench_function("configure_proxy", |b| {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let cassette_dir = temp_dir.path().to_path_buf();
        let proxy = MagnetoProxy::new_internal(&cassette_dir).expect("Failed to create proxy");

        b.iter(|| {
            proxy.set_port(8888);
            black_box(());
            proxy.set_mode(ProxyMode::Auto);
            black_box(());
        });
    });

    group.finish();
}

/// Benchmark: Recording lifecycle operations
fn bench_recording_lifecycle(c: &mut Criterion) {
    let mut group = c.benchmark_group("recording_lifecycle");

    group.bench_function("start_recording", |b| {
        let (proxy, _temp_dir) = create_test_proxy(ProxyMode::Record);
        let mut counter = 0;

        b.iter(|| {
            counter += 1;
            let cassette_name = format!("benchmark-{}", counter);
            proxy
                .start_recording_internal(cassette_name)
                .expect("Failed to start recording");
            black_box(());
            proxy
                .stop_recording_internal()
                .expect("Failed to stop recording");
        });
    });

    group.bench_function("stop_recording", |b| {
        let (proxy, _temp_dir) = create_test_proxy(ProxyMode::Record);

        b.iter(|| {
            proxy
                .start_recording_internal("test".to_string())
                .expect("Failed to start recording");
            proxy
                .stop_recording_internal()
                .expect("Failed to stop recording");
            black_box(());
        });
    });

    group.finish();
}

/// Benchmark: Mode switching overhead
fn bench_mode_switching(c: &mut Criterion) {
    let mut group = c.benchmark_group("mode_switching");

    let modes = vec![
        ProxyMode::Auto,
        ProxyMode::Record,
        ProxyMode::Replay,
        ProxyMode::Passthrough,
    ];

    for mode in &modes {
        group.bench_with_input(
            BenchmarkId::new("switch_to", format!("{:?}", mode)),
            mode,
            |b, &mode| {
                let (proxy, _temp_dir) = create_test_proxy(ProxyMode::Auto);

                b.iter(|| {
                    proxy.set_mode(mode);
                    black_box(());
                });
            },
        );
    }

    group.finish();
}

/// Benchmark: Cassette operations at different sizes
fn bench_cassette_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("cassette_operations");

    // Test with different cassette sizes (number of interactions)
    for size in [1, 10, 50, 100].iter() {
        group.throughput(Throughput::Elements(*size as u64));

        group.bench_with_input(BenchmarkId::new("load_cassette", size), size, |b, &size| {
            let (proxy, _temp_dir) = create_test_proxy(ProxyMode::Record);

            // Create a cassette with 'size' interactions
            proxy
                .start_recording_internal("benchmark".to_string())
                .expect("Failed to start");

            // Simulate multiple interactions
            for _i in 0..size {
                // In a real scenario, HTTP requests would be made here
                // For now, we're just testing the recording infrastructure overhead
            }

            proxy.stop_recording_internal().expect("Failed to stop");

            b.iter(|| {
                proxy
                    .replay_internal("benchmark".to_string())
                    .expect("Failed to replay");
                black_box(());
            });
        });
    }

    group.finish();
}

/// Benchmark: Memory allocations and clones
fn bench_memory_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_operations");

    group.bench_function("proxy_mode_clone", |b| {
        let mode = ProxyMode::Auto;
        b.iter(|| {
            black_box(mode);
        });
    });

    group.bench_function("get_port", |b| {
        let (proxy, _temp_dir) = create_test_proxy(ProxyMode::Auto);
        b.iter(|| {
            black_box(proxy.port());
        });
    });

    group.bench_function("get_mode", |b| {
        let (proxy, _temp_dir) = create_test_proxy(ProxyMode::Auto);
        b.iter(|| {
            black_box(proxy.mode());
        });
    });

    group.finish();
}

/// Benchmark: Concurrent proxy operations
fn bench_concurrent_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_operations");

    group.bench_function("concurrent_mode_reads", |b| {
        let (proxy, _temp_dir) = create_test_proxy(ProxyMode::Auto);

        b.iter(|| {
            // Measure concurrent access overhead without actual async spawning
            for _ in 0..10 {
                black_box(proxy.mode());
            }
        });
    });

    group.bench_function("concurrent_port_reads", |b| {
        let (proxy, _temp_dir) = create_test_proxy(ProxyMode::Auto);

        b.iter(|| {
            // Measure concurrent access overhead without actual async spawning
            for _ in 0..10 {
                black_box(proxy.port());
            }
        });
    });

    group.finish();
}

/// Benchmark: Latency measurements
fn bench_latency(c: &mut Criterion) {
    let mut group = c.benchmark_group("latency");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(1000);

    group.bench_function("proxy_overhead", |b| {
        let (proxy, _temp_dir) = create_test_proxy(ProxyMode::Passthrough);

        b.iter(|| {
            // Measure pure proxy overhead (no actual HTTP request)
            let start = std::time::Instant::now();
            black_box(proxy.mode());
            black_box(start.elapsed());
        });
    });

    group.bench_function("recording_overhead", |b| {
        let (proxy, _temp_dir) = create_test_proxy(ProxyMode::Record);
        let mut counter = 0;

        b.iter(|| {
            counter += 1;
            let cassette_name = format!("latency-{}", counter);
            let start = std::time::Instant::now();
            proxy
                .start_recording_internal(cassette_name)
                .expect("Failed to start");
            proxy.stop_recording_internal().expect("Failed to stop");
            black_box(start.elapsed());
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_proxy_creation,
    bench_recording_lifecycle,
    bench_mode_switching,
    bench_cassette_operations,
    bench_memory_operations,
    bench_concurrent_operations,
    bench_latency,
);

criterion_main!(benches);
