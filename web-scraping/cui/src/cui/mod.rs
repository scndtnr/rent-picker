mod options;

use self::options::{Options, Service, Task, WebScrape};
use adapter::{dto::SuumoRequestDto, Controller};
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
            Task::HealthCheck(args) => match args.target {
                Service::Suumo => self.process_health_check_suumo().await,
            },
            Task::WebScrape(args) => match args.service {
                Service::Suumo => self.process_web_scrape_suumo(args).await,
            },
        }
    }

    #[tracing::instrument(skip_all)]
    async fn process_health_check_suumo(&self) {
        // エラーにならなかったらOK扱い
        self.controller.health_check_suumo().await;

        // 結果表示
        tracing::info!("Summo health check is ok.");
    }

    #[tracing::instrument(skip_all)]
    async fn process_web_scrape_suumo(&self, args: &WebScrape) {
        tracing::debug!("web_scrape args : {:#?}", args);
        let dto = SuumoRequestDto::new(args.area.to_string(), args.station.clone(), args.dry_run);

        let res = self.controller.search_rent_suumo(dto).await;

        // 結果表示
        // todo!("DTOを表示する形に修正する")
        tracing::info!("{:#?}", res);
    }
}
