use serde::Serialize;

use crate::domain::model::count::Count;

// Java の MemberCountResponse record 相当
#[derive(Serialize)]
pub struct MemberCountResponse {
    pub count: i64,
}

impl From<Count> for MemberCountResponse {
    fn from(count: Count) -> Self {
        Self { count: count.value }
    }
}
