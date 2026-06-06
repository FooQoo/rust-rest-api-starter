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
pub struct MemberService {
    member_repository: Arc<dyn MemberRepository>,
}

impl MemberService {
    pub fn new(member_repository: Arc<dyn MemberRepository>) -> Self {
        Self { member_repository }
    }

    // #[tracing::instrument]: 関数の入退出を自動でログ出力する
    //   skip(self): self は Debug ログから除外 (うるさいので)
    //   err: エラー時に Err の中身もログに出す
    #[tracing::instrument(skip(self), err)]
    pub async fn search(&self, condition: MemberSearchCondition) -> anyhow::Result<Vec<Member>> {
        self.member_repository.search(condition).await
    }

    #[tracing::instrument(skip(self), err)]
    pub async fn count(&self) -> anyhow::Result<Count> {
        self.member_repository.count().await
    }
}

// ─────────────────────────────────────────────
// テスト: mockall で trait のモックを使う
// ─────────────────────────────────────────────
#[cfg(test)]
mod tests {
    // テスト内では unwrap() を許可 (失敗したらテストが落ちる、それで十分)
    #![allow(clippy::unwrap_used)]

    use super::*;
    use crate::domain::model::CompanyPosition;
    use crate::domain::repository::member_repository::MockMemberRepository;

    // テスト用の Member を作るヘルパー (テストごとに書くと冗長なので関数化)
    fn sample_member(id: i32, name: &str) -> Member {
        Member {
            id,
            name: name.to_string(),
            company_position: CompanyPosition {
                company_position_id: 1,
                name: "CEO".to_string(),
            },
        }
    }

    // #[tokio::test] は #[test] の async 版。tokio ランタイム上でテスト関数を実行する。
    // service のメソッドが async なので、これがないと .await できない。
    #[tokio::test]
    async fn search_returns_members_from_repository() {
        // 1. Mock を作る
        let mut mock = MockMemberRepository::new();

        // 2. 期待する挙動を設定: search が呼ばれたら 1件の Member を返す
        // returning(|引数| 戻り値) のクロージャでレスポンスを定義する
        mock.expect_search()
            .returning(|_| Ok(vec![sample_member(1, "John")]));

        // 3. Service にモックを注入 (Arc<dyn MemberRepository> として渡せる)
        let service = MemberService::new(Arc::new(mock));

        // 4. メソッドを呼んで結果を検証
        let result = service
            .search(MemberSearchCondition::default())
            .await
            .unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "John");
    }

    #[tokio::test]
    async fn count_returns_count_from_repository() {
        let mut mock = MockMemberRepository::new();
        mock.expect_count()
            .returning(|| Ok(Count::new(42).unwrap()));

        let service = MemberService::new(Arc::new(mock));
        let count = service.count().await.unwrap();

        assert_eq!(count.value(), 42);
    }

    #[tokio::test]
    async fn search_propagates_repository_error() {
        let mut mock = MockMemberRepository::new();
        // エラーを返すケース: anyhow::anyhow! マクロで Error を作る
        mock.expect_search()
            .returning(|_| Err(anyhow::anyhow!("simulated db error")));

        let service = MemberService::new(Arc::new(mock));
        let result = service.search(MemberSearchCondition::default()).await;

        // Service はエラーをそのまま伝播するので Err になっているはず
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn search_returns_empty_when_repository_returns_empty() {
        let mut mock = MockMemberRepository::new();
        mock.expect_search().returning(|_| Ok(vec![]));

        let service = MemberService::new(Arc::new(mock));
        let result = service
            .search(MemberSearchCondition::default())
            .await
            .unwrap();

        assert!(result.is_empty());
    }
}
