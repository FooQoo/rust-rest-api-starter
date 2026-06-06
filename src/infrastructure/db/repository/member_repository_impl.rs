use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{QueryBuilder, SqlitePool};

use crate::domain::model::{CompanyPosition, Count, Member, MemberSearchCondition};
use crate::domain::repository::member_repository::MemberRepository;
use crate::infrastructure::db::entity::member_entity::MemberRow;

// Java の MemberRepositoryImpl.java 相当
#[derive(Debug)]
pub(crate) struct SqliteMemberRepository {
    pool: Arc<SqlitePool>,
}

impl SqliteMemberRepository {
    pub(crate) const fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MemberRepository for SqliteMemberRepository {
    // Java の MemberSearchSqlHelper を使った動的 SQL 構築に相当
    // QueryBuilder で WHERE 句を条件に応じて組み立てる
    #[tracing::instrument(skip(self), err)]
    async fn search(&self, condition: MemberSearchCondition) -> anyhow::Result<Vec<Member>> {
        let mut qb: QueryBuilder<sqlx::Sqlite> = QueryBuilder::new(
            r"SELECT m.member_id, m.name, cp.company_position_id, cp.name AS position_name
               FROM member m
               JOIN company_position cp ON m.company_position_id = cp.company_position_id
               WHERE 1 = 1",
        );

        // Some の場合のみ WHERE 条件を追加 — Java の hasCondition() / Optional.ifPresent() 相当
        if let Some(name) = condition.name {
            qb.push(" AND m.name = ").push_bind(name);
        }
        if let Some(position_id) = condition.company_position_id {
            qb.push(" AND m.company_position_id = ")
                .push_bind(position_id);
        }

        let rows: Vec<MemberRow> = qb
            .build_query_as::<MemberRow>()
            // &*self.poolでも良い。デリファンス -> 借用
            .fetch_all(self.pool.as_ref())
            .await?;

        // DB の行 (MemberRow) → ドメインモデル (Member) への変換
        // Java の row.mapRow() 相当
        let members = rows
            .into_iter()
            .map(|row| Member {
                id: row.member_id,
                name: row.name,
                company_position: CompanyPosition {
                    company_position_id: row.company_position_id,
                    name: row.position_name,
                },
            })
            .collect();

        Ok(members)
    }

    #[tracing::instrument(skip(self), err)]
    async fn count(&self) -> anyhow::Result<Count> {
        // query_as でタプル型にマッピングする最もシンプルな COUNT 取得方法
        let (value,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM member")
            // &*self.poolでも良い。デリファンス -> 借用
            .fetch_one(self.pool.as_ref())
            .await?;

        // Count::new でバリデーション (負数や u32::MAX 超過を弾く)
        Count::new(value)
    }
}
