// ドメインモデルを集約。
// 関連する型はまとめて1ファイルに置くのが Rust 流。
// (Java の "1 file = 1 public class" は引きずらない)

// ─────────────────────────────────────────────
// 役職
// id は現状未使用だが、ドメインモデルとして「役職には ID がある」を表現するため残す。
// #[allow(dead_code)] で「未使用警告」を抑制 (将来 ID を使う API が増えたら外す)
// ─────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct CompanyPosition {
    #[allow(dead_code)]
    pub company_position_id: i32,
    pub name: String,
}

// ─────────────────────────────────────────────
// 社員
// ─────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct Member {
    pub id: i32,
    pub name: String,
    pub company_position: CompanyPosition,
}

// ─────────────────────────────────────────────
// 社員検索条件
// Default を実装することで MemberSearchCondition::default() で
// 「全部 None (条件なし)」のインスタンスが作れる。
// ─────────────────────────────────────────────
#[derive(Debug, Clone, Default)]
pub struct MemberSearchCondition {
    pub name: Option<String>,
    pub company_position_id: Option<i32>,
}

// ─────────────────────────────────────────────
// Newtype による Value Object: u32 で非負を保証
// ─────────────────────────────────────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Count(u32);

impl Count {
    // i64 → u32 変換 (負数や u32 範囲超えは弾く)
    // must_use: 生成した Count を使わずに捨てると警告が出る
    #[must_use = "Count::new returns a Result that must be handled"]
    pub fn new(value: i64) -> anyhow::Result<Self> {
        let value: u32 = value.try_into()?;
        Ok(Self(value))
    }

    // 値渡し (self) を採用。Copy 型なので呼び出し側は元の Count を再利用できる。
    // u32 は 4 byte で参照より値渡しの方が効率的 (clippy::trivially_copy_pass_by_ref)
    pub const fn value(self) -> u32 {
        self.0
    }
}

// ─────────────────────────────────────────────
// テスト (Rust では実装ファイルの末尾に同居させるのが慣習)
// ─────────────────────────────────────────────
#[cfg(test)]
mod tests {
    // テストでは Result::unwrap() を使うのが慣習 (失敗したらテストが落ちる、それで十分)
    // 通常コードでは禁止している lint を、ここだけ許可する
    #![allow(clippy::unwrap_used)]

    // 親モジュール (= このファイル全体) の中身を取り込む
    use super::*;

    #[test]
    fn count_new_accepts_zero() {
        let count = Count::new(0).unwrap();
        assert_eq!(count.value(), 0);
    }

    #[test]
    fn count_new_accepts_positive() {
        let count = Count::new(42).unwrap();
        assert_eq!(count.value(), 42);
    }

    #[test]
    fn count_new_accepts_u32_max() {
        let count = Count::new(i64::from(u32::MAX)).unwrap();
        assert_eq!(count.value(), u32::MAX);
    }

    #[test]
    fn count_new_rejects_negative() {
        assert!(Count::new(-1).is_err());
    }

    #[test]
    fn count_new_rejects_u32_overflow() {
        // u32::MAX = 4_294_967_295。5_000_000_000 は超えるので弾かれる。
        assert!(Count::new(5_000_000_000).is_err());
    }

    #[test]
    fn member_search_condition_default_is_all_none() {
        let cond = MemberSearchCondition::default();
        assert!(cond.name.is_none());
        assert!(cond.company_position_id.is_none());
    }
}
