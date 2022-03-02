//! SeaORM Entity. Generated by sea-orm-codegen 0.6.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "contents")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u64,
    pub title: String,
    pub keywords: String,
    pub description: String,
    #[sea_orm(unique)]
    pub content_name: Option<String>,
    pub content_type: ContentType,
    pub is_published: bool,
    pub published_at: Option<DateTimeUtc>,
    pub modified_at: Option<DateTimeUtc>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "u32", db_type = "Integer")]
pub enum ContentType {
    #[sea_orm(num_value = 0)]
    Blog,
    #[sea_orm(num_value = 1)]
    Page,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_one = "super::content_entities::Entity")]
    Entity,
}

impl Related<super::content_entities::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Entity.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
