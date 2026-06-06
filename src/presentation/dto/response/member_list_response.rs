use serde::Serialize;

use crate::domain::model::member::Member;

use super::member_response::MemberResponse;

// Java の MemberListResponse record 相当
#[derive(Serialize)]
pub struct MemberListResponse {
    pub members: Vec<MemberResponse>,
}

impl From<Vec<Member>> for MemberListResponse {
    fn from(members: Vec<Member>) -> Self {
        Self {
            members: members.into_iter().map(MemberResponse::from).collect(),
        }
    }
}
