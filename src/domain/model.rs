// ドメインモデルを集約。
// 関連する型はまとめて1ファイルに置くのが Rust 流。
// (Java の "1 file = 1 public class" は引きずらない)

// ─────────────────────────────────────────────
// 役職
// ─────────────────────────────────────────────
pub struct CompanyPosition {
    pub company_position_id: i32,
    pub name: String,
}

// ─────────────────────────────────────────────
// 社員
// ─────────────────────────────────────────────
pub struct Member {
    pub id: i32,
    pub name: String,
    pub company_position: CompanyPosition,
}

// ─────────────────────────────────────────────
// 社員検索条件
// ─────────────────────────────────────────────
pub struct MemberSearchCondition {
    pub name: Option<String>,
    pub company_position_id: Option<i32>,
}

// ─────────────────────────────────────────────
// Newtype による Value Object: u32 で非負を保証
// ─────────────────────────────────────────────
pub struct Count(u32);

impl Count {
    // i64 → u32 変換 (負数や u32 範囲超えは弾く)
    pub fn new(value: i64) -> anyhow::Result<Self> {
        let value: u32 = value.try_into()?;
        Ok(Self(value))
    }

    pub const fn value(&self) -> u32 {
        self.0
    }
}
