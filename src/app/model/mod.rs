pub mod content;

pub use super::entity::{content_entities::Model as ContentEntities, contents::Model as Content};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(default)]
pub struct Paginate {
    pub page: usize,
    pub per_page: usize,
}

impl Default for Paginate {
    fn default() -> Self {
        Paginate {
            page: 1,
            per_page: 20
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct Paginated<T> {
    pub data: Vec<T>,
    pub page: usize,
    pub per_page: usize,
    pub total: usize,
    pub last_page: usize,
}

impl<T> Paginated<T> {
    pub fn map_into<R: From<T>>(self) -> Paginated<R> {
        Paginated {
            data: self.data.into_iter().map(Into::into).collect(),
            page: self.page,
            per_page: self.per_page,
            total: self.total,
            last_page: self.last_page,
        }
    }
}
