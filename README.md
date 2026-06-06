# rust-rest-api-starter

Rust の学習用 REST API スターター。
`../rest-starter` (Java / Spring Boot) を参考に、同等のレイヤードアーキテクチャを Rust で実装しています。

参考記事: [一休レストランの Rust バックエンド構成](https://user-first.ikyu.co.jp/entry/2023/12/25/132215)

## 技術スタック

| 用途 | ライブラリ | Java での相当物 |
|---|---|---|
| Web framework | [axum](https://docs.rs/axum) | Spring WebFlux |
| 非同期ランタイム | [tokio](https://tokio.rs) | Reactor |
| DB ドライバ | [sqlx](https://docs.rs/sqlx) (SQLite) | R2DBC |
| エラーハンドリング | [anyhow](https://docs.rs/anyhow) | 例外 + `@ControllerAdvice` |
| JSON | [serde](https://serde.rs) | Jackson |
| HTTP ミドルウェア | [tower-http](https://docs.rs/tower-http) | Servlet Filter |
| ログ | [tracing](https://docs.rs/tracing) | SLF4J / Logback |
| async fn in trait | [async-trait](https://docs.rs/async-trait) | (Java は標準で対応) |
| `.env` 読み込み | [dotenvy](https://docs.rs/dotenvy) | Spring の `application.yml` |

## アーキテクチャ

Java 版と同じ4層構造です。

```
src/
├── main.rs                     # エントリポイント。DI の手動配線もここ
├── error.rs                    # AppError (axum で HTTP レスポンスに変換)
│
├── presentation/               # @RestController + DTO 層
│   ├── handler/                #   ↳ Controller 相当
│   └── dto/
│       ├── request/            #   ↳ @RequestBody / @RequestParam 相当
│       └── response/           #   ↳ レスポンス DTO (record 相当)
│
├── application/                # @Service 層
│   └── service/
│
├── domain/                     # ドメインモデル & リポジトリ interface
│   ├── model/                  #   ↳ POJO (record / @Getter) 相当
│   └── repository/             #   ↳ Java の interface に相当 (trait)
│
└── infrastructure/             # 外部依存の実装層
    └── db/
        ├── entity/             #   ↳ DB 行をマップする構造体 (FromRow)
        └── repository/         #   ↳ domain/repository の trait 実装
```

依存方向: `presentation → application → domain ← infrastructure`
ドメイン層は外側に依存しないクリーンアーキテクチャの構成です。

## セットアップ

### Rust のパス

```bash
export PATH="/Users/satoshifukuyama/dev/other/goose/bin:$PATH"
```

(hermit 経由で入っているので、必要に応じて `.zshrc` に追加)

### ビルド

```bash
cargo build
```

### 起動

```bash
cargo run
```

初回起動時に `data/rest_starter.db` が自動作成され、スキーマと初期データが投入されます。
再起動するたびに DROP → CREATE が走るので、毎回まっさらな状態から始まります。

サーバーは `http://localhost:8080` で待ち受けます。

## API

### `GET /v1/member/count`

社員数を返します。

```bash
$ curl http://localhost:8080/v1/member/count
{"count":3}
```

### `GET /v1/member/search`

社員を名前で検索します。

| クエリパラメータ | 必須 | 説明 |
|---|---|---|
| `name` | ✅ | 完全一致 |
| `company_position_id` | | 部署 ID で絞り込み |

```bash
$ curl "http://localhost:8080/v1/member/search?name=John"
{"members":[{"id":1,"name":"John","position_name":"CEO"}]}
```

## DB スキーマ

```sql
company_position (company_position_id, name)
member           (member_id, name, company_position_id)
```

初期データ:

| company_position_id | name | | member_id | name | position |
|---|---|---|---|---|---|
| 1 | CEO | | 1 | John | CEO |
| 2 | Manager | | 2 | Daniel | Manager |
| 3 | Staff | | 3 | Lisa | Staff |

## Java との対応 (学習用チートシート)

### 言語機能

| Java | Rust |
|---|---|
| `null` / `@Nullable T` / `Optional<T>` | `Option<T>` (`Some(x)` / `None`) |
| `throws` / 例外 | `Result<T, E>` + `?` 演算子 |
| `interface` | `trait` |
| `class` | `struct` + `impl` |
| 継承 (`extends`) | (なし。trait の組み合わせで表現) |
| `static` メソッド | `impl` の `fn` (self なし) |
| Builder パターン (Lombok) | 構造体リテラル `Foo { a, b, c }` |
| Record | `struct` (タプル構造体も可) |
| `@Override` | (不要。trait 実装で自動チェック) |

### DI / 並行性

| Java | Rust |
|---|---|
| `@Autowired` 依存注入 | `main.rs` で手動配線 |
| Spring の Singleton Bean | `Arc<T>` (参照カウント) |
| 動的ポリモーフィズム | `Arc<dyn Trait>` |
| `Flux<T>` / `Mono<T>` | `async fn` + `Vec<T>` |
| `CompletableFuture<T>` | `Future<Output = T>` (async/await) |

### Web

| Java | Rust |
|---|---|
| `@RestController` | axum の `Router` + handler 関数 |
| `@GetMapping("/foo")` | `Router::new().route("/foo", get(handler))` |
| `@RequestParam` | `Query<T>` extractor |
| `@RequestBody` | `Json<T>` extractor |
| `@PathVariable` | `Path<T>` extractor |
| `@Autowired` の Bean | `State<T>` extractor |
| `@ExceptionHandler` | `IntoResponse` impl |

## 進行中のタスク

- [ ] `POST /v1/member` (社員追加) — 自分で書く

## ライセンス

学習目的のため未指定。
