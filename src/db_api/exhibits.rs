//! admin api to manage exhibits
use crate::db_api::{Bilingual, DB};
use crate::prisma::exhibit;
use serde::Deserialize;

/// query struct for POST request
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewExhibitData {
    location: Bilingual,
    title: Bilingual,
    sign: Bilingual,
    artists: Vec<Bilingual>,
}

/// query struct for PATCH request
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateExhibitData {
    location: Option<Bilingual>,
    title: Option<Bilingual>,
    sign: Option<Bilingual>,
    artists: Option<Vec<Bilingual>>,
}

impl NewExhibitData {
    fn into_db_data(
        self,
    ) -> anyhow::Result<(
        serde_json::Value,
        serde_json::Value,
        serde_json::Value,
        serde_json::Value,
        Vec<exhibit::SetParam>,
    )> {
        Ok((
            serde_json::to_value(self.location)?,
            serde_json::to_value(self.title)?,
            serde_json::to_value(self.sign)?,
            serde_json::to_value(self.artists)?,
            vec![],
        ))
    }
}

impl UpdateExhibitData {
    fn into_db_data(self) -> anyhow::Result<Vec<exhibit::SetParam>> {
        let mut params = Vec::with_capacity(4);
        if let Some(location) = self.location {
            params.push(exhibit::SetParam::SetLocation(serde_json::to_value(
                location,
            )?));
        }
        if let Some(title) = self.title {
            params.push(exhibit::SetParam::SetTitle(serde_json::to_value(title)?));
        }
        if let Some(sign) = self.sign {
            params.push(exhibit::SetParam::SetSign(serde_json::to_value(sign)?));
        }
        if let Some(artists) = self.artists {
            params.push(exhibit::SetParam::SetArtists(serde_json::to_value(
                artists,
            )?));
        }
        Ok(params)
    }
}

impl DB {
    // C

    pub async fn create_exhibit(&self, data: NewExhibitData) -> anyhow::Result<exhibit::Data> {
        let (location, title, sign, artists, _) = data.into_db_data()?;
        let data = self
            .0
            .exhibit()
            .create(location, title, sign, artists, vec![])
            .exec()
            .await?;
        Ok(data)
    }

    pub async fn create_many_exhibits(&self, data: Vec<NewExhibitData>) -> anyhow::Result<i64> {
        let mut many_data = Vec::with_capacity(data.len());
        for d in data {
            many_data.push(d.into_db_data()?);
        }
        let data = self.0.exhibit().create_many(many_data).exec().await?;
        Ok(data)
    }

    // R

    pub async fn get_exhibit(&self, id: String) -> anyhow::Result<Option<exhibit::Data>> {
        let data = self
            .0
            .exhibit()
            .find_unique(exhibit::UniqueWhereParam::IdEquals(id))
            .exec()
            .await?;
        Ok(data)
    }

    pub async fn get_all_exhibits(&self) -> anyhow::Result<Vec<exhibit::Data>> {
        let data = self.0.exhibit().find_many(vec![]).exec().await?;
        Ok(data)
    }

    // U

    pub async fn update_exhibit(
        &self,
        id: String,
        data: UpdateExhibitData,
    ) -> anyhow::Result<exhibit::Data> {
        let data = self
            .0
            .exhibit()
            .update(
                exhibit::UniqueWhereParam::IdEquals(id),
                data.into_db_data()?,
            )
            .exec()
            .await?;
        Ok(data)
    }

    // D

    pub async fn delete_exhibit(&self, id: String) -> anyhow::Result<exhibit::Data> {
        let data = self
            .0
            .exhibit()
            .delete(exhibit::UniqueWhereParam::IdEquals(id))
            .exec()
            .await?;
        Ok(data)
    }

    pub async fn delete_all_exhibits(&self) -> anyhow::Result<i64> {
        let data = self.0.exhibit().delete_many(vec![]).exec().await?;
        Ok(data)
    }
}
