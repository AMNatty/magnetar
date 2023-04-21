//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.7

use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Copy)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "antenna_src_enum")]
pub enum AntennaSrcEnum {
    #[sea_orm(string_value = "all")]
    All,
    #[sea_orm(string_value = "group")]
    Group,
    #[sea_orm(string_value = "home")]
    Home,
    #[sea_orm(string_value = "instances")]
    Instances,
    #[sea_orm(string_value = "list")]
    List,
    #[sea_orm(string_value = "users")]
    Users,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Copy)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "meta_sensitivemediadetection_enum"
)]
pub enum MetaSensitivemediadetectionEnum {
    #[sea_orm(string_value = "all")]
    All,
    #[sea_orm(string_value = "local")]
    Local,
    #[sea_orm(string_value = "none")]
    None,
    #[sea_orm(string_value = "remote")]
    Remote,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Copy)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "meta_sensitivemediadetectionsensitivity_enum"
)]
pub enum MetaSensitivemediadetectionsensitivityEnum {
    #[sea_orm(string_value = "high")]
    High,
    #[sea_orm(string_value = "low")]
    Low,
    #[sea_orm(string_value = "medium")]
    Medium,
    #[sea_orm(string_value = "veryHigh")]
    VeryHigh,
    #[sea_orm(string_value = "veryLow")]
    VeryLow,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Copy)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "muted_note_reason_enum"
)]
pub enum MutedNoteReasonEnum {
    #[sea_orm(string_value = "manual")]
    Manual,
    #[sea_orm(string_value = "other")]
    Other,
    #[sea_orm(string_value = "spam")]
    Spam,
    #[sea_orm(string_value = "word")]
    Word,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Copy)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "note_visibility_enum"
)]
pub enum NoteVisibilityEnum {
    #[sea_orm(string_value = "followers")]
    Followers,
    #[sea_orm(string_value = "home")]
    Home,
    #[sea_orm(string_value = "public")]
    Public,
    #[sea_orm(string_value = "specified")]
    Specified,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Copy)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "notification_type_enum"
)]
pub enum NotificationTypeEnum {
    #[sea_orm(string_value = "app")]
    App,
    #[sea_orm(string_value = "follow")]
    Follow,
    #[sea_orm(string_value = "followRequestAccepted")]
    FollowRequestAccepted,
    #[sea_orm(string_value = "groupInvited")]
    GroupInvited,
    #[sea_orm(string_value = "mention")]
    Mention,
    #[sea_orm(string_value = "pollEnded")]
    PollEnded,
    #[sea_orm(string_value = "pollVote")]
    PollVote,
    #[sea_orm(string_value = "quote")]
    Quote,
    #[sea_orm(string_value = "reaction")]
    Reaction,
    #[sea_orm(string_value = "receiveFollowRequest")]
    ReceiveFollowRequest,
    #[sea_orm(string_value = "renote")]
    Renote,
    #[sea_orm(string_value = "reply")]
    Reply,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Copy)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "page_visibility_enum"
)]
pub enum PageVisibilityEnum {
    #[sea_orm(string_value = "followers")]
    Followers,
    #[sea_orm(string_value = "public")]
    Public,
    #[sea_orm(string_value = "specified")]
    Specified,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Copy)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "poll_notevisibility_enum"
)]
pub enum PollNotevisibilityEnum {
    #[sea_orm(string_value = "followers")]
    Followers,
    #[sea_orm(string_value = "home")]
    Home,
    #[sea_orm(string_value = "public")]
    Public,
    #[sea_orm(string_value = "specified")]
    Specified,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Copy)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "relay_status_enum")]
pub enum RelayStatusEnum {
    #[sea_orm(string_value = "accepted")]
    Accepted,
    #[sea_orm(string_value = "rejected")]
    Rejected,
    #[sea_orm(string_value = "requesting")]
    Requesting,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Copy)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "user_profile_ffvisibility_enum"
)]
pub enum UserProfileFfvisibilityEnum {
    #[sea_orm(string_value = "followers")]
    Followers,
    #[sea_orm(string_value = "private")]
    Private,
    #[sea_orm(string_value = "public")]
    Public,
}