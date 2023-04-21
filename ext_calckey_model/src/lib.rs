use ck::user;
use log::LevelFilter;
use sea_orm::{ColumnTrait, ConnectOptions, DatabaseConnection, EntityTrait, QueryFilter};

#[derive(Debug)]
pub struct ConnectorConfig {
    pub url: String,
}

#[derive(Clone, Debug)]
pub struct CalckeyModel(DatabaseConnection);

impl CalckeyModel {
    pub async fn new(config: ConnectorConfig) -> anyhow::Result<Self> {
        let opt = ConnectOptions::new(config.url)
            .max_connections(64)
            .min_connections(8)
            .sqlx_logging(true)
            .sqlx_logging_level(LevelFilter::Debug)
            .to_owned();

        Ok(CalckeyModel(sea_orm::Database::connect(opt).await?))
    }

    pub async fn get_user_by_tag(
        &self,
        name: &str,
        instance: Option<&str>,
    ) -> anyhow::Result<Option<user::Model>> {
        let name = name.to_lowercase();
        let instance = instance.map(str::to_lowercase);

        let user = if let Some(instance) = instance {
            user::Entity::find()
                .filter(user::Column::UsernameLower.eq(name))
                .filter(user::Column::Host.eq(instance))
        } else {
            user::Entity::find().filter(user::Column::UsernameLower.eq(name))
        }
        .one(&self.0)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_uri(&self, uri: &str) -> anyhow::Result<Option<user::Model>> {
        Ok(user::Entity::find()
            .filter(user::Column::Uri.eq(uri))
            .one(&self.0)
            .await?)
    }
}
