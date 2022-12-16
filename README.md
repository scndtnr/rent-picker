# rent-picker

Rustで賃貸情報をスクレイピングし、Pythonで機械学習させて、コスパの良い賃貸物件を探すプロジェクト

## 構成

`web-scraping` では、Rustを用いて住宅情報サイトから賃貸情報を取得し、Sqliteに保存する。
`machine-learning` では、Pythonを用いてSqliteのデータを読み出し、LightGBMにて学習と推論を行う。
`sql-ddl` では、両者を繋ぐ sqlite のDDLを管理する。

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
sqldef のバイナリは[リリースページ](https://github.com/k0kubun/sqldef/releases)からダウンロードしてプロジェクトルートに配置し、下記のように利用する。

```
# template
sqlite3def --file=<sql_file_path> <db_name>

# example
./sqlite3def --file=sql-ddl/create_table_room_header.sql rent-picker.sqlite3
```


## スクレイピング対象サービスの利用規約

### [SUUMO](https://suumo.jp/)

[SUUMO(スーモ) ご利用規約](https://suumo.jp/edit/kiyaku/)を見ると、
2022年12月11日現在は特にスクレイピングを禁止する条項は無いと思われるが、
スクレイピングの頻度によっては下記に当たるかもしれないので、sleepを十分入れるよう注意しておく。

> 第3条　ユーザーの禁止行為
>
> (6) 本サイトの運営を妨げる行為