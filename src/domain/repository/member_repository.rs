use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;

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
//
// #[cfg_attr(test, automock)]:
//   テストビルド時のみ #[automock] が適用され、MockMemberRepository が自動生成される。
//   通常ビルドでは Mock 型は生成されない (バイナリサイズに影響しない)。
//   重要: automock は async_trait より外側に置く必要がある。
#[cfg_attr(test, automock)]
#[async_trait]
pub trait MemberRepository: Send + Sync + std::fmt::Debug {
    async fn search(&self, condition: MemberSearchCondition) -> anyhow::Result<Vec<Member>>;
    async fn count(&self) -> anyhow::Result<Count>;
}
