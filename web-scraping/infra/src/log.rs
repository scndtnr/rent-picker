use domain::model::Jst;
use tracing::{metadata::LevelFilter, Level};
use tracing_appender::rolling::RollingFileAppender;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{
    filter::Targets, fmt::MakeWriter, layer::SubscriberExt, util::SubscriberInitExt,
};

use crate::env::{get_bool_of_env_var, get_env_var};

/// CUI実行時に tracing の設定をする関数
pub fn init_cui_log(log_name: &str) {
    // ログ設定にフィルタ・フォーマットを登録し適用する
    let builder = LogConfigBuilder::new(log_name);
    builder.build();
}

struct LogConfigBuilder {
    log_name: String,
    log_config_pattern: LogConfigPattern,
}

enum LogConfigPattern {
    BunyanMyAppOnly,
    BunyanAllApp,
}

impl LogConfigBuilder {
    fn new(log_name: &str) -> Self {
        Self {
            log_name: log_name.to_string(),
            log_config_pattern: if get_bool_of_env_var("IS_LOG_TARGET_MY_APP_ONLY") {
                LogConfigPattern::BunyanMyAppOnly
            } else {
                LogConfigPattern::BunyanAllApp
            },
        }
    }

    /// ログ設定にフィルタ・フォーマットを登録し適用する
    fn build(&self) {
        match self.log_config_pattern {
            LogConfigPattern::BunyanMyAppOnly => tracing_subscriber::registry()
                .with(self.only_myapp_filter())
                .with(JsonStorageLayer)
                .with(self.bunyan_stdio_format())
                .with(self.bunyan_file_format(self.make_writer_to_file()))
                .init(),
            LogConfigPattern::BunyanAllApp => tracing_subscriber::registry()
                .with(self.all_app_filter())
                .with(JsonStorageLayer)
                .with(self.bunyan_stdio_format())
                .with(self.bunyan_file_format(self.make_writer_to_file()))
                .init(),
        }
    }

    // ------------------- filter layer ------------------------------

    fn level(&self) -> Level {
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
    fn only_myapp_filter(&self) -> Targets {
        tracing_subscriber::filter::Targets::new()
            .with_target("infra", self.level())
            .with_target("adapter", self.level())
            .with_target("usecase", self.level())
            .with_target("domain", self.level())
            .with_target("cui", self.level())
    }

    /// 出力対象クレートに依存クレートも含める
    fn all_app_filter(&self) -> LevelFilter {
        tracing_subscriber::filter::LevelFilter::from_level(self.level())
    }

    //  --------------------- format layer ----------------------------------

    /// bunyan形式で標準出力に書き込むフォーマッタ
    fn bunyan_stdio_format(&self) -> BunyanFormattingLayer<fn() -> std::io::Stdout> {
        BunyanFormattingLayer::new(self.log_name.clone(), std::io::stdout)
    }

    /// bunyan形式でファイルに書き込むフォーマッタ
    fn bunyan_file_format<W>(&self, make_writer: W) -> BunyanFormattingLayer<W>
    where
        W: for<'a> MakeWriter<'a> + 'static,
    {
        BunyanFormattingLayer::new(self.log_name.clone(), make_writer)
    }

    /// ------------------ helper -----------------------------

    /// ファイル出力用のWriter
    fn make_writer_to_file(&self) -> RollingFileAppender {
        // 実行時のディレクトリを取得する
        let current_dir = match std::env::current_dir() {
            Ok(current_dir) => current_dir,
            Err(e) => panic!("Fail to get current directory\n{:#?}", e),
        };

        // 環境変数からディレクトリ、ファイル名を取得する
        let log_dir = get_env_var("LOG_DIR").unwrap();
        let today = Jst::today().format("%Y-%m-%d").to_string();
        let log_filename = get_env_var("LOG_FILENAME").unwrap();

        // ログファイルのWriterを生成する
        tracing_appender::rolling::hourly(current_dir.join(log_dir).join(today), log_filename)
    }
}
