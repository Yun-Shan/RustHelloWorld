use sea_orm::entity::prelude::*;
use my_macro::entity;

#[entity]
#[derive(Debug, Clone, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique, column_type = "String(Some(16))")]
    pub username: String,
}

#[derive(Debug, Clone, Copy, EnumIter,DeriveRelation)]
pub enum Relation {}

// impl RelationTrait for Relation {
//     fn def(&self) -> RelationDef { panic!("No Relation") }
// }

impl ActiveModelBehavior for ActiveModel {}