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

/// Benchmark: WebSocket proxy creation and configuration
fn bench_websocket_proxy_setup(c: &mut Criterion) {
    let mut group = c.benchmark_group("websocket_proxy_setup");

    group.bench_function("create_websocket_proxy", |b| {
        b.iter(|| {
            let temp_dir = TempDir::new().expect("Failed to create temp dir");
            let cassette_dir = temp_dir.path().to_path_buf();
            let proxy = MagnetoProxy::new_internal(&cassette_dir).expect("Failed to create proxy");
            proxy.set_mode(ProxyMode::Record);
            black_box(proxy);
        });
    });

    group.finish();
}

/// Benchmark: WebSocket recording lifecycle
fn bench_websocket_recording(c: &mut Criterion) {
    let mut group = c.benchmark_group("websocket_recording");

    group.bench_function("start_ws_recording", |b| {
        let (proxy, _temp_dir) = create_test_proxy(ProxyMode::Record);
        let mut counter = 0;

        b.iter(|| {
            counter += 1;
            let cassette_name = format!("ws-benchmark-{}", counter);
            proxy
                .start_recording_internal(cassette_name)
                .expect("Failed to start recording");
            black_box(());
            proxy
                .stop_recording_internal()
                .expect("Failed to stop recording");
        });
    });

    group.finish();
}

/// Benchmark: WebSocket message throughput simulation
fn bench_websocket_message_throughput(c: &mut Criterion) {
    let mut group = c.benchmark_group("websocket_message_throughput");

    // Test with different message counts
    for msg_count in [10, 100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*msg_count as u64));

        group.bench_with_input(
            BenchmarkId::new("simulate_messages", msg_count),
            msg_count,
            |b, &msg_count| {
                let (proxy, _temp_dir) = create_test_proxy(ProxyMode::Record);

                b.iter(|| {
                    proxy
                        .start_recording_internal("ws-messages".to_string())
                        .expect("Failed to start");

                    // Simulate processing multiple WebSocket messages
                    for _i in 0..msg_count {
                        // In real scenario, this would be actual WebSocket message processing
                        // For now, we measure the infrastructure overhead
                        black_box(proxy.mode());
                    }

                    proxy.stop_recording_internal().expect("Failed to stop");
                });
            },
        );
    }

    group.finish();
}

/// Benchmark: WebSocket message size impact
fn bench_websocket_message_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("websocket_message_sizes");

    // Test with different message sizes (simulated)
    for size in [64, 256, 1024, 4096, 16384].iter() {
        group.throughput(Throughput::Bytes(*size as u64));

        group.bench_with_input(
            BenchmarkId::new("message_size_bytes", size),
            size,
            |b, &size| {
                let (proxy, _temp_dir) = create_test_proxy(ProxyMode::Record);
                let dummy_data = vec![0u8; size];

                b.iter(|| {
                    // Simulate message processing overhead with different sizes
                    black_box(&dummy_data);
                    black_box(proxy.mode());
                });
            },
        );
    }

    group.finish();
}

/// Benchmark: WebSocket replay performance
fn bench_websocket_replay(c: &mut Criterion) {
    let mut group = c.benchmark_group("websocket_replay");

    group.bench_function("replay_websocket_cassette", |b| {
        let (proxy, _temp_dir) = create_test_proxy(ProxyMode::Record);

        // Create a cassette first
        proxy
            .start_recording_internal("ws-replay-test".to_string())
            .expect("Failed to start");
        proxy.stop_recording_internal().expect("Failed to stop");

        // Switch to replay mode
        proxy.set_mode(ProxyMode::Replay);

        b.iter(|| {
            proxy
                .replay_internal("ws-replay-test".to_string())
                .expect("Failed to replay");
            black_box(());
        });
    });

    group.finish();
}

/// Benchmark: Concurrent WebSocket operations
fn bench_concurrent_websocket_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_websocket_ops");

    group.bench_function("concurrent_ws_proxy_access", |b| {
        let (proxy, _temp_dir) = create_test_proxy(ProxyMode::Auto);

        b.iter(|| {
            // Measure concurrent access overhead without actual async spawning
            for _ in 0..10 {
                black_box(proxy.mode());
                black_box(proxy.port());
            }
        });
    });

    group.finish();
}

/// Benchmark: WebSocket latency overhead
fn bench_websocket_latency(c: &mut Criterion) {
    let mut group = c.benchmark_group("websocket_latency");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(1000);

    group.bench_function("ws_message_latency", |b| {
        let (proxy, _temp_dir) = create_test_proxy(ProxyMode::Passthrough);

        b.iter(|| {
            let start = std::time::Instant::now();
            // Simulate WebSocket message processing latency
            black_box(proxy.mode());
            black_box(start.elapsed());
        });
    });

    group.bench_function("ws_recording_latency", |b| {
        let (proxy, _temp_dir) = create_test_proxy(ProxyMode::Record);
        let mut counter = 0;

        b.iter(|| {
            counter += 1;
            let cassette_name = format!("ws-latency-{}", counter);
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

/// Benchmark: WebSocket mode switching
fn bench_websocket_mode_switching(c: &mut Criterion) {
    let mut group = c.benchmark_group("websocket_mode_switching");

    let modes = vec![
        ProxyMode::Auto,
        ProxyMode::Record,
        ProxyMode::Replay,
        ProxyMode::Passthrough,
    ];

    for mode in &modes {
        group.bench_with_input(
            BenchmarkId::new("ws_switch_to", format!("{:?}", mode)),
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

criterion_group!(
    benches,
    bench_websocket_proxy_setup,
    bench_websocket_recording,
    bench_websocket_message_throughput,
    bench_websocket_message_sizes,
    bench_websocket_replay,
    bench_concurrent_websocket_ops,
    bench_websocket_latency,
    bench_websocket_mode_switching,
);

criterion_main!(benches);
