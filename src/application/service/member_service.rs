use std::sync::Arc;

use crate::domain::model::{Count, Member, MemberSearchCondition};
use crate::domain::repository::member_repository::MemberRepository;

// Java の MemberService.java 相当
//
// Arc<dyn MemberRepository> のポイント:
//   Arc  = 参照カウントによる共有所有権 (Spring の DI コンテナが管理する Bean に相当)
//   dyn  = 動的ディスパッチ。実装型を知らなくてもトレイト経由で呼び出せる
//          → テスト時にモック実装に差し替えられる (Java の @MockBean に相当)
#[derive(Debug, Clone)]
pub(crate) struct MemberService {
    member_repository: Arc<dyn MemberRepository>,
}

impl MemberService {
    pub(crate) fn new(member_repository: Arc<dyn MemberRepository>) -> Self {
        Self { member_repository }
    }

    // #[tracing::instrument]: 関数の入退出を自動でログ出力する
    //   skip(self): self は Debug ログから除外 (うるさいので)
    //   err: エラー時に Err の中身もログに出す
    #[tracing::instrument(skip(self), err)]
    pub(crate) async fn search(
        &self,
        condition: MemberSearchCondition,
    ) -> anyhow::Result<Vec<Member>> {
        self.member_repository.search(condition).await
    }

    #[tracing::instrument(skip(self), err)]
    pub(crate) async fn count(&self) -> anyhow::Result<Count> {
        self.member_repository.count().await
    }
}
