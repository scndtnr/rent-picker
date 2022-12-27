use tracing::Level;
use tracing_subscriber::filter::{LevelFilter, Targets};

use crate::env::get_env_var;

fn level(env_level_filter: bool) -> Level {
    // 環境変数を使わない場合はTRACEを返す
    if !env_level_filter {
        return Level::TRACE;
    };

    // 環境変数を使う場合
    match get_env_var("LOG_LEVEL").unwrap() {
        s if s.to_uppercase() == "ERROR" => Level::ERROR,
        s if s.to_uppercase() == "WARN" => Level::WARN,
        s if s.to_uppercase() == "INFO" => Level::INFO,
        s if s.to_uppercase() == "DEBUG" => Level::DEBUG,
        s if s.to_uppercase() == "TRACE" => Level::TRACE,
        _ => Level::TRACE,
    }
}

/// 出力対象クレートを自分のクレートのみとする
/// （※ハイフンはアンダースコアに置き換えないと認識されない）
pub(super) fn app_only(env_level_filter: bool) -> Targets {
    Targets::new()
        .with_target("infra", level(env_level_filter))
        .with_target("adapter", level(env_level_filter))
        .with_target("usecase", level(env_level_filter))
        .with_target("domain", level(env_level_filter))
        .with_target("cui", level(env_level_filter))
}

/// 出力対象クレートをデータベース操作クレートのみとする
/// （※ハイフンはアンダースコアに置き換えないと認識されない）
pub(super) fn db_only(env_level_filter: bool) -> Targets {
    Targets::new().with_target("sqlx", level(env_level_filter))
}

/// 出力対象クレートに依存クレートも含める（全てのログを出力する）
#[allow(unused)]
pub(super) fn system(env_level_filter: bool) -> LevelFilter {
    LevelFilter::from_level(level(env_level_filter))
}
