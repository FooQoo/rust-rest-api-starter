use std::sync::Arc;

use axum::{routing::get, Router};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

mod application;
mod domain;
mod error;
mod infrastructure;
mod presentation;

use application::service::member_service::MemberService;
use infrastructure::db::repository::member_repository_impl::SqliteMemberRepository;
use presentation::handler::member_handler;

// axum のルーターに渡す共有状態
// Java の ApplicationContext (Spring DI コンテナ) が管理する Bean 群に相当する
//
// Clone が必要な理由: axum はリクエストごとに State をクローンするため
// Arc は参照カウントのみのクローンなので安価
#[derive(Debug, Clone)]
pub(crate) struct AppState {
    pub(crate) member_service: Arc<MemberService>,
}

// #[tokio::main] が main 関数を非同期ランタイム上で実行するエントリポイントにする
// Spring Boot の SpringApplication.run() に相当
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // .env ファイルから環境変数を読み込む (application.yml 相当)
    // ファイルが存在しなくてもエラーにしない (.ok())
    dotenvy::dotenv().ok();

    // ログ出力の初期化 (Logback 相当)
    // RUST_LOG=debug などの環境変数でログレベルを制御できる
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let database_url = std::env::var("DATABASE_URL")?;

    // SQLite 接続オプション
    // create_if_missing(true): ファイルが無ければ自動作成 (初回起動時のみ)
    let connect_options = SqliteConnectOptions::from_str(&database_url)?.create_if_missing(true);

    // コネクションプールを作成 (HikariCP 相当)
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(connect_options)
        .await?;

    // 初期スキーマ + 初期データを流し込む
    // raw_sql は ; で区切られた複数 statement を実行できる (sqlx::query は単一 statement のみ)
    // 既存テーブルを DROP してから作るので冪等 (起動するたびにリセット)
    sqlx::raw_sql(include_str!("../sql/01_schema.sql"))
        .execute(&pool)
        .await?;
    sqlx::raw_sql(include_str!("../sql/02_data.sql"))
        .execute(&pool)
        .await?;
    tracing::info!("Database initialized");

    // 依存注入の配線 (Spring の @Configuration + @Bean に相当)
    // Rust では DI フレームワークがないため、main で手動で組み立てる
    let member_repository = Arc::new(SqliteMemberRepository::new(Arc::new(pool)));
    let member_service = Arc::new(MemberService::new(member_repository));

    let state = AppState { member_service };

    // ルーティング定義 (Spring の @RequestMapping に相当)
    let app = Router::new()
        .route("/v1/member/count", get(member_handler::count))
        .route("/v1/member/search", get(member_handler::search))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("Listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
