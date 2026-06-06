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
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            Self::Internal(err) => {
                // 内部エラーはサーバー側で詳細をログに、クライアントには簡素なメッセージを
                tracing::error!(error = ?err, "internal error");
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
        };
        (status, message).into_response()
    }
}
