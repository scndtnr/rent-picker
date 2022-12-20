# rent-picker

Rustで賃貸情報をスクレイピングし、Pythonで機械学習させて、コスパの良い賃貸物件を探すプロジェクト

## 構成

一部省略したディレクトリ構成は下記の通り。

```tree
 .
├──  web-scraping
├──  machine-learning
├──  sql-ddl
├──  litestream
├──  docker-compose.yml
└──  sqlite3def.exe
```

`web-scraping` では、Rustを用いて住宅情報サイトから賃貸情報を取得し、DBに保存する。  
`machine-learning` では、Pythonを用いてDBのデータを読み出し、LightGBMにて学習と推論を行う。  
`sql-ddl` では、sqldef (sqlite3def.exe) で扱うデータベースのテーブル定義ファイル (DDL) を管理する。  
`litestream` では、SQLiteファイルをS3/GCSにレプリケーションするための設定ファイルを管理する。  
`docker-compose.yml` では、litestreamコンテナとアプリケーションコンテナの設定をする。  


### `web-scraping` (Rust)

レイヤードアーキテクチャで作成し、当面はバイナリクレート`cui`から利用する。  
いずれは[pyo3](https://github.com/PyO3/pyo3)を利用してPythonから呼び出せるようにしたい。

- domain  : モデル定義やrepositoryのtraitを書く
- usecase : domainに依存し、usecase毎の処理を書く
- adapter : usecaseを呼び出すControllerの定義やDTO定義を書く
- infra   : repositoryやDIコンテナの実装を書く
- cui     : 各層の処理を呼び出し、コマンドライン引数に沿った処理を行う


### `machine-learning` (Python)

色々未定だが、[python-fire](https://github.com/google/python-fire)を利用したCLIツールにする予定。



### `sql-ddl` (sqldef によるスキーマ管理)

[k0kubun/sqldef](https://github.com/k0kubun/sqldef)で利用するためのDDLを置く。  
sqldef のバイナリは[リリースページ](https://github.com/k0kubun/sqldef/releases)からダウンロードしてプロジェクトルートに配置し利用する。

なお、ローカルファイルを対象とする場合は下記の通りにすればよいが、  
litestreamを利用する場合は、事前にDockerコンテナを立ち上げておくこと。

```
# get current scheme
./sqlite3def  data/rent-picker.sqlite3 --export

# apply scheme file (dry run)
./sqlite3def --file=sql-ddl/create_table.sql data/rent-picker.sqlite3 --dry-run

# apply scheme file
./sqlite3def --file=sql-ddl/create_table.sql data/rent-picker.sqlite3
```

### `litestream` (SQLite利用時にクラウドへレプリケーションを行う)

Dockerコンテナ経由でlitestreamを扱い、sqliteファイルをS3やGCSにレプリケーションする。  
基本的な操作は docker-compose.yml のコマンドに設定しておくが、手動で状況確認したい時は下記のように利用する。

```sh
# まずコンテナを起動して litestream を動かす
docker-compose -f docker-compose.yml up -d

# restore処理は重いので、data配下の様子を見ながら数分待つ。
docker exec -it rent-picker-sqlite-backup-1 ls -al /data

# generation 一覧を表示する
docker exec -it rent-picker-sqlite-backup-1 litestream generations -config /opt/litestream/litestream.yaml /data/rent-picker.sqlite3

# 作業が終わったらコンテナを削除する
docker-compose -f docker-compose.yml down
```

### `docker-compose.yml` 

スクレイピング処理をVMインスタンス上で実行するために追加した。  
litestreamコンテナ2つとアプリケーションコンテナを立ち上げるサイドカー構成にしている。  

利用方法は下記の通り （`litestream`欄と一部重複している）

```sh
# まずコンテナを起動して litestream を動かす
docker-compose -f docker-compose.yml up -d

# restore処理は重いので、data配下の様子を見ながら数分待つ。
docker exec -it rent-picker-sqlite-backup-1 ls -al /data

# health-check
docker exec -t rent-picker-web-scraping-1 /usr/local/bin/cui health-check | bunyan

# read-db (temp_room_headerテーブルのサマリ出力)
docker exec -t rent-picker-web-scraping-1 /usr/local/bin/cui read-db --action summary --table room-header --table-type temp | bunyan

# scraping (1ページのみ、標準出力はINFO以上に限る)
docker exec -t --env MAX_PAGE=1 --env LOG_LEVEL="INFO" rent-picker-web-scraping-1 /usr/local/bin/cui web-scrape --area tokyo 新宿 --save | bunyan

# 作業が終わったらコンテナを削除する
docker-compose -f docker-compose.yml down
```

## スクレイピング対象サービスの利用規約

### [SUUMO](https://suumo.jp/)

[SUUMO(スーモ) ご利用規約](https://suumo.jp/edit/kiyaku/)を見ると、
2022年12月11日現在は特にスクレイピングを禁止する条項は無いと思われるが、
スクレイピングの頻度によっては下記に当たるかもしれないので、sleepを十分入れるよう注意しておく。

> 第3条　ユーザーの禁止行為
>
> (6) 本サイトの運営を妨げる行為