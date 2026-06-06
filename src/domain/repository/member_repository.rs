use async_trait::async_trait;

use crate::domain::model::count::Count;
use crate::domain::model::member::Member;
use crate::domain::model::member_search_condition::MemberSearchCondition;

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
#[async_trait]
pub trait MemberRepository: Send + Sync {
    async fn search(&self, condition: MemberSearchCondition) -> anyhow::Result<Vec<Member>>;
    async fn count(&self) -> anyhow::Result<Count>;
}
