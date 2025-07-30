use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct QueryPager {
    pub page: i32,
    pub limit: i32,
}
