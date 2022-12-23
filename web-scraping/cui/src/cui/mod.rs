mod options;

use self::options::{
    web::{RawRoom, RoomHeader, TargetPage},
    DataAction, Db, Options, Service, Task, Web,
};
use adapter::{
    dto::{ReadDbRequestDto, ScrapeSuumoRawRoomParamsDto, ScrapeSuumoRoomHeaderParamsDto},
    Controller,
};
use clap::Parser;
use infra::{persistence::sqlite::SqliteDb, RepositoryImpls, UsecaseImpls};

#[derive(Clone, Debug)]
pub(super) struct Cui {
    controller: Controller<UsecaseImpls>,
    opts: Options,
}

impl Cui {
    pub(super) async fn new() -> Self {
        let db = SqliteDb::new().await;
        let repository = RepositoryImpls::new(db);
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
            Task::HealthCheck(args) => match args.service {
                Service::Suumo => self.process_health_check_suumo().await,
            },
            Task::Web(args) => match args.service {
                Service::Suumo => match &args.target_page {
                    TargetPage::RoomHeader(params) => {
                        self.process_scrape_suumo_room_headers(args, params).await
                    }
                    TargetPage::RawRoom(params) => {
                        self.process_scrape_suumo_raw_rooms(args, params).await
                    }
                },
            },
            Task::Db(args) => match args.action {
                DataAction::Summary => self.process_read_db_for_summary(args).await,
                DataAction::Top => todo!(),
                DataAction::Export => todo!(),
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
    async fn process_scrape_suumo_room_headers(&self, args: &Web, params: &RoomHeader) {
        tracing::debug!("web_scrape args : {:#?}", args);
        let dto = ScrapeSuumoRoomHeaderParamsDto::new(
            params.area.to_string(),
            params.station.clone(),
            params.max_page,
            params.save,
            params.dry_run,
        );

        let _res = self.controller.scrape_room_headers_from_suumo(dto).await;

        // 結果表示
        // todo!("DTOを表示する形に修正する")
        // tracing::info!("{:#?}", res);
    }

    #[tracing::instrument(skip_all)]
    async fn process_scrape_suumo_raw_rooms(&self, args: &Web, params: &RawRoom) {
        tracing::debug!("web_scrape args : {:#?}", args);
        let dto = ScrapeSuumoRawRoomParamsDto::new(
            params.area.to_string(),
            params.max_page,
            params.chunk_size,
            params.save,
            params.dry_run,
        );

        let _res = self.controller.scrape_raw_rooms_from_suumo(dto).await;

        // 結果表示
        // todo!("DTOを表示する形に修正する")
        // tracing::info!("{:#?}", res);
    }

    #[tracing::instrument(skip_all)]
    async fn process_read_db_for_summary(&self, args: &Db) {
        tracing::debug!("read_db args : {:#?}", args);
        let dto = ReadDbRequestDto::new(args.table.to_string(), args.table_type.to_string());

        // サマリを表示する
        self.controller.read_db_for_summary(dto).await;
    }
}
