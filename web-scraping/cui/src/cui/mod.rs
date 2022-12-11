mod options;

use self::options::{Options, Target, Task};
use adapter::{dto::RequestDto, Controller};
use clap::Parser;
use infra::{RepositoryImpls, UsecaseImpls};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct Cui {
    controller: Controller<UsecaseImpls>,
    opts: Options,
}

impl Cui {
    pub(super) fn new() -> Self {
        let repository = RepositoryImpls::new();
        let usecases = UsecaseImpls::new(repository);
        let controller = Controller::new(usecases);
        Self {
            controller,
            opts: Options::parse(),
        }
    }

    #[tracing::instrument(skip_all)]
    pub(super) async fn process_cmd(&self) {
        match &self.opts.task {
            Task::HealthCheck => match &self.opts.target {
                Target::Suumo => self.process_health_check_suumo(&self.opts).await,
            },
            Task::WebScrape => match &self.opts.target {
                Target::Suumo => self.process_web_scrape_suumo(&self.opts).await,
            },
        }
    }

    #[tracing::instrument(skip_all)]
    async fn process_health_check_suumo(&self, opts: &Options) {
        tracing::debug!("web_scrape args : {:#?}", opts);
        let dto = RequestDto::new(opts.dry_run);
        // エラーにならなかったらOK扱い
        self.controller.health_check_suumo(dto).await;

        // 結果表示
        tracing::info!("Summo health check is ok.");
    }

    #[tracing::instrument(skip_all)]
    async fn process_web_scrape_suumo(&self, opts: &Options) {
        tracing::debug!("web_scrape args : {:#?}", opts);
        let _dto = RequestDto::new(opts.dry_run);

        todo!()

        // let res = self.controller.

        // // 結果表示
        // tracing::info!("{}", res);
    }
}
