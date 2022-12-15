use std::sync::Arc;

use anyhow::Result;
use domain::{
    model::{AsVec, RoomHeader, RoomHeaders, TargetArea},
    repository::{Repositories, RoomHeaderRepository, SuumoRepository},
};
use futures::{stream, StreamExt, TryStreamExt};

use crate::env::get_usize_of_env_var;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScrapeRoomHeadersUsecase<R: Repositories> {
    suumo_repo: R::SuumoRepo,
    room_header_repo: R::RoomHeaderRepo,
}

impl<R: Repositories> ScrapeRoomHeadersUsecase<R> {
    pub fn new(suumo_repo: R::SuumoRepo, room_header_repo: R::RoomHeaderRepo) -> Self {
        Self {
            suumo_repo,
            room_header_repo,
        }
    }

    #[tracing::instrument(skip_all, err(Debug))]
    pub async fn scrape_room_headers_from_suumo(
        &self,
        area: TargetArea,
        station: &str,
        save: bool,
    ) -> Result<RoomHeaders> {
        let room_headers = self.room_headers_one_shot(&area, station, save).await?;
        // let room_headers = self.room_headers_step_by_step(&area, station, save).await?;

        tracing::info!("{:#?}", room_headers.len());

        Ok(room_headers)
    }

    #[tracing::instrument(skip_all, err(Debug))]
    async fn room_headers_one_shot(
        &self,
        area: &TargetArea,
        station: &str,
        save: bool,
    ) -> Result<RoomHeaders> {
        // 前準備
        let crawler = self.suumo_repo.new_crawler().await;

        // 指定した地域・通勤先の駅を元に賃貸情報を取得する
        let room_headers = self
            .suumo_repo
            .room_headers_by_area_and_station(&crawler, area, station)
            .await?;

        // 賃貸概要をデータベースに保存する
        if save {
            self.save_room_headers_to_load_table("all", &room_headers)
                .await?;
            self.save_room_headers_to_main_table().await?;
        }

        Ok(room_headers)
    }

    #[tracing::instrument(skip_all, err(Debug))]
    async fn room_headers_step_by_step(
        &self,
        area: &TargetArea,
        station: &str,
        save: bool,
    ) -> Result<RoomHeaders> {
        // 前準備
        let crawler = self.suumo_repo.new_crawler().await;

        // 検索条件を選択し、賃貸一覧ページの1ページ目のURLを取得する
        let mut url = self
            .suumo_repo
            .url_of_room_list_by_area_and_station(&crawler, area, station)
            .await?;

        // 最後のページ番号を確認し、各ページのURLを生成する
        let urls = self
            .suumo_repo
            .urls_of_room_list_by_one_url(&crawler, &mut url)
            .await?;

        // 各賃貸一覧ページから住居情報や詳細ページへのURLを取得する
        tracing::info!("Urls length: {}", urls.len());
        let buffered_n = get_usize_of_env_var("MAX_CONCURRENCY");
        let max_page = get_usize_of_env_var("MAX_PAGE");
        let urls = if urls.len() <= (max_page) {
            urls
        } else {
            urls[0..max_page].to_vec()
        };

        // 各ページをスクレイピングし、
        // save = true であれば、ロードテーブルにinsertする
        let room_headers_vec: Result<Vec<RoomHeaders>> = stream::iter(urls)
            .map(|url| (Arc::clone(&crawler), url, area.clone(), station.to_string()))
            .map(|(crawler, url, area, station)| async move {
                // ページ番号を取得する
                let page_num = url
                    .query_pairs()
                    .find(|(key, _)| key.as_ref() == "page")
                    .map(|(_, page_num)| page_num.to_string())
                    .expect("Fail to find page number params.");

                // ページ毎にスクレイピングする
                let room_headers = self
                    .suumo_repo
                    .room_headers_by_url(crawler, url, area, station)
                    .await?;
                // もし save = true であれば、ロードテーブルにinsertする
                if save {
                    self.save_room_headers_to_load_table(&page_num, &room_headers)
                        .await?;
                }
                Ok(room_headers)
            })
            .buffer_unordered(buffered_n)
            .try_collect()
            .await;

        // 作業用ロードテーブルのデータをPKで集約し、
        // 本テーブルの同PKデータをdeleteしてから、
        // 本テーブルに集約データをinsertする
        if save {
            self.save_room_headers_to_main_table().await?;
        }

        let room_headers: RoomHeaders = room_headers_vec?
            .into_iter()
            .flat_map(|room_headers| room_headers.into_inner())
            .collect::<Vec<RoomHeader>>()
            .into();
        Ok(room_headers)
    }

    #[tracing::instrument(skip(self, room_headers), err(Debug))]
    async fn save_room_headers_to_load_table(
        &self,
        page_num: &str,
        room_headers: &RoomHeaders,
    ) -> Result<()> {
        // 作業用ロードテーブルにスクレイピングデータを入れ込む
        self.room_header_repo
            .insert_many(room_headers, true)
            .await?;
        Ok(())
    }

    #[tracing::instrument(skip_all, err(Debug))]
    async fn save_room_headers_to_main_table(&self) -> Result<()> {
        // 作業用ロードテーブルからPKで集約したデータを取り出す
        let room_header_group_by_pk = self.room_header_repo.group_by_pk_from_load_table().await?;

        // 集約データとPKが一致するレコードを本テーブルから削除する
        self.room_header_repo
            .delete_many_by_pk(&room_header_group_by_pk, false)
            .await?;

        // 集約データを本テーブルに入れ込む
        self.room_header_repo
            .insert_many(&room_header_group_by_pk, false)
            .await
    }
}
