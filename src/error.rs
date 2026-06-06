use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

// axum ハンドラーから返すエラー型。
// thiserror::Error を使うと:
//   - Display 実装が自動生成される (#[error("...")] で書式指定)
//   - std::error::Error が自動実装される
//   - #[from] で他のエラー型からの変換が書ける
//
// Java では @ExceptionHandler や @ControllerAdvice でやる処理に相当する。
#[derive(Debug, Error)]
pub enum AppError {
    // anyhow::Error からの自動変換を提供
    // ? 演算子で anyhow::Error → AppError へ変換される
    #[error("internal error: {0}")]
    Internal(#[from] anyhow::Error),
}

// axum がレスポンスに変換するための実装。
// バリアントに応じて適切な HTTP ステータスとメッセージを返す。
// 将来 NotFound 等のバリアントを足すなら、ここに match の腕を追加する。
//
// セキュリティ上のポイント:
//   内部エラーの詳細 (SQL 構文、内部パス、スタック等) をそのままクライアントに
//   返さない。詳細は tracing でサーバー側ログに残し、クライアントには
//   抽象的なメッセージのみ返す。
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, public_message) = match &self {
            Self::Internal(err) => {
                // 内部エラーの詳細はログに記録 (運用者が原因を追える)
                tracing::error!(error = ?err, "internal error");
                // クライアントには抽象的なメッセージのみ返す
                (StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
            }
        };
        (status, public_message).into_response()
    }
}
