use criterion::{black_box, criterion_group, criterion_main, Criterion};
use prompt_engine::config::EngineConfig;
use prompt_engine::optimize_prompt;
use shared_types::api::{OptimizeRequest, UserPreferences};
use shared_types::domain::{DetailLevel, OptimizeMode, RequestedLanguage, TargetPlatform};

fn sample_request() -> OptimizeRequest {
    OptimizeRequest {
        raw_user_input: "corrige mon code python qui plante avec une erreur d'index".to_owned(),
        target_platform: TargetPlatform::Chatgpt,
        language: RequestedLanguage::Auto,
        mode: OptimizeMode::Preview,
        user_preferences: UserPreferences {
            tone: None,
            detail_level: DetailLevel::Normal,
        },
    }
}

fn bench_optimize(c: &mut Criterion) {
    let config = EngineConfig::default();
    c.bench_function("optimize_prompt", |b| {
        b.iter(|| optimize_prompt(black_box(sample_request()), &config))
    });
}

criterion_group!(benches, bench_optimize);
criterion_main!(benches);
