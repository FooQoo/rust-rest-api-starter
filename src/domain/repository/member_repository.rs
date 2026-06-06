use async_trait::async_trait;

use crate::domain::model::{Count, Member, MemberSearchCondition};

// Java の MemberRepository インターフェース相当
//
// async_trait が必要な理由:
// Rust の trait では async fn を dyn Trait として使うと、
// 返り値の型サイズがコンパイル時に決まらないため直接使えない。
// async_trait マクロがそれを Box<dyn Future> に変換して解決する。
//
// Send + Sync 境界:
// tokio のマルチスレッドランタイムでは、スレッドをまたぐ可能性がある型に
// Send + Sync が必要。Arc<dyn MemberRepository> として使うために付ける。
// trait 境界に Debug を追加:
// AppState や MemberService が #[derive(Debug)] するとき、
// 内部の Arc<dyn MemberRepository> が Debug でないとビルドが通らない。
// trait 自体に Debug 境界を要求しておけば、impl 側で derive(Debug) するだけで OK。
#[async_trait]
pub trait MemberRepository: Send + Sync + std::fmt::Debug {
    async fn search(&self, condition: MemberSearchCondition) -> anyhow::Result<Vec<Member>>;
    async fn count(&self) -> anyhow::Result<Count>;
}
