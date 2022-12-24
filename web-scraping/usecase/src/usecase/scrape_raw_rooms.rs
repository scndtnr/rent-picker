use anyhow::Result;
use domain::{
    model::{AsVec, RawRooms, TableType, TargetArea},
    repository::{RawRoomRepository, Repositories, RoomHeaderRepository, SuumoRepository},
};

use url::Url;

use crate::progress_bar::{debug_progress, new_progress_bar};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScrapeRawRoomsUsecase<R: Repositories> {
    suumo_repo: R::SuumoRepo,
    room_header_repo: R::RoomHeaderRepo,
    raw_room_repo: R::RawRoomRepo,
}

impl<R: Repositories> ScrapeRawRoomsUsecase<R> {
    pub fn new(
        suumo_repo: R::SuumoRepo,
        room_header_repo: R::RoomHeaderRepo,
        raw_room_repo: R::RawRoomRepo,
    ) -> Self {
        Self {
            suumo_repo,
            room_header_repo,
            raw_room_repo,
        }
    }

    #[tracing::instrument(skip_all, err(Debug))]
    pub async fn scrape_raw_rooms_from_suumo(
        &self,
        area: TargetArea,
        max_page: usize,
        chunk_size: usize,
        save: bool,
        dry_run: bool,
    ) -> Result<RawRooms> {
        // 前準備
        let crawler = self.suumo_repo.new_crawler().await;

        // 住居の属する地域を指定して、データベースから詳細ページのURLを取り出す
        let urls = self.urls_from_database_by_area(area).await?;
        tracing::info!("Urls length: {}", urls.len());

        // 仮実行の場合、ここで処理を終了する
        if dry_run {
            let dummy_raw_rooms: RawRooms = Vec::new().into();
            return Ok(dummy_raw_rooms);
        }

        // 一度にスクレイピングする最大ページ数を設定する
        let urls = if urls.len() <= (max_page) {
            urls
        } else {
            urls[0..max_page].to_vec()
        };

        // プログレスバーの準備
        let chunks_len = (urls.len() as f64 / chunk_size as f64).ceil() as u64;
        let pb_message = "[RawRoom - Processing] Web-Scraping and Insert to database by chunk...";
        let pb_chunks = new_progress_bar(chunks_len).await;
        pb_chunks.set_message(pb_message);

        // chunk_size 毎に urls を分割して実行する
        // データベース処理が前後すると不整合が発生するので、
        // 必ず1チャンク毎に処理するため同期処理とする
        let mut raw_room_vec = Vec::new();
        for chunk in urls.chunks(chunk_size) {
            // プログレスバーをインクリメントしてログを出す
            pb_chunks.inc(1);
            debug_progress(&pb_chunks, pb_message).await;

            // 詳細ページのURLから賃貸の詳細情報を取得する
            let mut raw_rooms = self.suumo_repo.raw_rooms(&crawler, chunk.to_vec()).await?;

            // 賃貸詳細をデータベースに保存する
            if save {
                self.save_raw_rooms_to_temp_table(&raw_rooms).await?;
                self.save_raw_rooms_to_load_table().await?;
                self.save_raw_rooms_to_main_table().await?;
            }

            // 結果を格納する
            raw_room_vec.append(raw_rooms.as_mut_vec());
        }

        // プログレスバーの後始末
        let pb_finish_message =
            "[RawRoom - Finished] Web-Scraping and Insert to database by chunk.";
        pb_chunks.finish_with_message(pb_finish_message);
        debug_progress(&pb_chunks, pb_finish_message).await;

        tracing::info!("Scraping Page Count: {:#?}", raw_room_vec.len());

        Ok(raw_room_vec.into())
    }

    #[tracing::instrument(skip_all, err(Debug))]
    pub async fn urls_from_database_by_area(&self, area: TargetArea) -> Result<Vec<Url>> {
        // 住居の属する地域を指定して、データベースから詳細ページのURLを取り出す
        self.room_header_repo
            .find_unscraped_raw_room_urls_with_area(area)
            .await
    }

    #[tracing::instrument(skip_all, err(Debug))]
    async fn save_raw_rooms_to_temp_table(&self, raw_rooms: &RawRooms) -> Result<()> {
        // 作業用一時テーブルのデータを全削除する
        self.raw_room_repo.delete_all(TableType::Temp).await?;

        // 作業用一時テーブルにスクレイピングデータを入れ込む
        self.raw_room_repo
            .insert_many_multi(raw_rooms, TableType::Temp)
            .await?;

        Ok(())
    }

    #[tracing::instrument(skip(self), err(Debug))]
    async fn save_raw_rooms_to_load_table(&self) -> Result<()> {
        // 作業用一時テーブルから累積テーブルにデータを入れ込む
        self.raw_room_repo.insert_to_load_from_temp_all().await
    }

    #[tracing::instrument(skip_all, err(Debug))]
    async fn save_raw_rooms_to_main_table(&self) -> Result<()> {
        // is_expired = false の
        // 集約データとPKが一致するレコードを本テーブルから削除する
        self.raw_room_repo
            .delete_from_main_by_temp_not_expired_group_by_pk()
            .await?;

        // is_expired = false の
        // 集約データを本テーブルに入れ込む
        self.raw_room_repo
            .insert_to_main_from_temp_not_expired_group_by_pk()
            .await?;

        // tempテーブル上は is_expired = true で
        // 本テーブルに存在するレコードについて、
        // is_expired カラムを更新する
        self.raw_room_repo
            .update_is_expired_of_main_by_temp()
            .await?;

        // tempテーブル上は is_expired = true で
        // 本テーブルに存在しないレコードを、
        // 本テーブルに入れ込む
        self.raw_room_repo
            .insert_to_main_from_temp_only_expired_record()
            .await
    }
}
