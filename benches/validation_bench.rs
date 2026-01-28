use criterion::{black_box, criterion_group, criterion_main, Criterion};
use envcheck::config::{Config, ToolCheck, EnvVarCheck, FileCheck};
use envcheck::validators::run_all_validations;

fn bench_validation(c: &mut Criterion) {
    let config = Config {
        version: "1".to_string(),
        tools: vec![
            ToolCheck {
                name: "node".to_string(),
                version: Some(">=18.0.0".to_string()),
                required: true,
            },
            ToolCheck {
                name: "git".to_string(),
                version: None,
                required: true,
            },
        ],
        env_vars: vec![
            EnvVarCheck {
                name: "PATH".to_string(),
                required: true,
                pattern: None,
            },
        ],
        ports: vec![3000, 5432, 6379, 8080],
        files: vec![
            FileCheck {
                path: "Cargo.toml".to_string(),
                required: true,
                is_directory: false,
                permissions: None,
            },
        ],
    };

    c.bench_function("run_all_validations", |b| {
        b.iter(|| {
            let _ = run_all_validations(black_box(&config));
        })
    });
}

criterion_group!(benches, bench_validation);
criterion_main!(benches);
