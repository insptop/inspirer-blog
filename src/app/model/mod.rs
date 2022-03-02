//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

pub mod prelude;

pub mod content_entities;
pub mod contents;

pub mod common {
    use serde::Deserialize;

    #[derive(Serialize, Deserialize, Clone, Copy, Debug)]
    pub struct Paginate {
        pub page: usize,
        pub per_page: usize,
    }

    #[derive(Serialize, Clone, Debug)]
    pub struct Paginated<T> {
        pub data: Vec<T>,
        pub page: usize,
        pub per_page: usize,
        pub total: usize,
        pub last_page: usize,
    }
}