use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

// axum ハンドラーから返すエラー型
// anyhow::Error をラップして HTTP レスポンスに変換できるようにする
//
// Java では @ExceptionHandler や @ControllerAdvice でやる処理に相当する
pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}

// anyhow::Error に変換できる任意の型から AppError を生成できるようにする
// これにより ? 演算子が自動変換してくれる
// 例: let result = some_fallible_fn()?;  // anyhow::Error → AppError
impl<E: Into<anyhow::Error>> From<E> for AppError {
    fn from(e: E) -> Self {
        Self(e.into())
    }
}
