//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.7

use super::sea_orm_active_enums::PollNotevisibilityEnum;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "poll")]
pub struct Model {
    #[sea_orm(column_name = "noteId", primary_key, auto_increment = false, unique)]
    pub note_id: String,
    #[sea_orm(column_name = "expiresAt")]
    pub expires_at: Option<DateTimeWithTimeZone>,
    pub multiple: bool,
    pub choices: Vec<String>,
    pub votes: Vec<i32>,
    #[sea_orm(column_name = "noteVisibility")]
    pub note_visibility: PollNotevisibilityEnum,
    #[sea_orm(column_name = "userId")]
    pub user_id: String,
    #[sea_orm(column_name = "userHost")]
    pub user_host: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::note::Entity",
        from = "Column::NoteId",
        to = "super::note::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Note,
}

impl Related<super::note::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Note.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}