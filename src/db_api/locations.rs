//! admin api to manage locations
use crate::db_api::{Bilingual, DB};
use crate::prisma::{duck, location};
use serde::Deserialize;

/// query struct for POST request
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewLocationData {
    description: Bilingual,
    duck_id: Option<String>,
}

/// query struct for PATCH request
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateLocationData {
    description: Option<Bilingual>,
    duck_id: Option<String>,
}

impl NewLocationData {
    fn into_db_data(self) -> anyhow::Result<(serde_json::Value, Vec<location::SetParam>)> {
        let mut params = Vec::with_capacity(1);
        if let Some(duck_id) = self.duck_id {
            params.push(location::SetParam::ConnectDuck(
                duck::UniqueWhereParam::IdEquals(duck_id),
            ));
        }
        Ok((serde_json::to_value(self.description)?, params))
    }
}

impl UpdateLocationData {
    fn into_db_data(self) -> anyhow::Result<Vec<location::SetParam>> {
        let mut params = Vec::with_capacity(2);
        if let Some(description) = self.description {
            params.push(location::SetParam::SetDescription(serde_json::to_value(
                description,
            )?));
        }
        if let Some(duck_id) = self.duck_id {
            params.push(location::SetParam::ConnectDuck(
                duck::UniqueWhereParam::IdEquals(duck_id),
            ));
        }
        Ok(params)
    }
}

impl DB {
    // C

    pub async fn create_location(&self, data: NewLocationData) -> anyhow::Result<location::Data> {
        let (location, _) = data.into_db_data()?;
        let data = self.0.location().create(location, vec![]).exec().await?;
        Ok(data)
    }

    pub async fn create_many_locations(&self, data: Vec<NewLocationData>) -> anyhow::Result<i64> {
        let mut many_data = Vec::with_capacity(data.len());
        for d in data {
            many_data.push(d.into_db_data()?);
        }
        let data = self.0.location().create_many(many_data).exec().await?;
        Ok(data)
    }

    // R

    pub async fn get_location(&self, id: String) -> anyhow::Result<Option<location::Data>> {
        let data = self
            .0
            .location()
            .find_unique(location::UniqueWhereParam::IdEquals(id))
            .exec()
            .await?;
        Ok(data)
    }

    pub async fn get_all_locations(&self) -> anyhow::Result<Vec<location::Data>> {
        let data = self.0.location().find_many(vec![]).exec().await?;
        Ok(data)
    }

    // U

    pub async fn update_location(
        &self,
        id: String,
        data: UpdateLocationData,
    ) -> anyhow::Result<location::Data> {
        let data = self
            .0
            .location()
            .update(
                location::UniqueWhereParam::IdEquals(id),
                data.into_db_data()?,
            )
            .exec()
            .await?;
        Ok(data)
    }

    // D

    pub async fn delete_location(&self, id: String) -> anyhow::Result<location::Data> {
        let data = self
            .0
            .location()
            .delete(location::UniqueWhereParam::IdEquals(id))
            .exec()
            .await?;
        Ok(data)
    }

    pub async fn delete_all_locations(&self) -> anyhow::Result<i64> {
        let data = self.0.location().delete_many(vec![]).exec().await?;
        Ok(data)
    }
}
