//! admin api to manage ducks
use crate::db_api::{Bilingual, DB};
use crate::prisma::{duck, exhibit};
use serde::Deserialize;

/// query struct for POST request
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewDuckData {
    title: Bilingual,
    story: Bilingual,
    location: Bilingual,
    topics: Vec<Bilingual>,
    duck_icon_url: String,
    #[serde(default)]
    is_hidden: bool,
    related_exhibit_id: Option<String>,
    prev_duck_story_id: Option<String>,
}

/// query struct for PATCH request
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDuckData {
    title: Option<Bilingual>,
    story: Option<Bilingual>,
    location: Option<Bilingual>,
    topics: Option<Vec<Bilingual>>,
    duck_icon_url: Option<String>,
    is_hidden: Option<bool>,
    related_exhibit_id: Option<String>,
    prev_duck_story_id: Option<String>,
}

impl NewDuckData {
    fn into_db_data(
        self,
    ) -> anyhow::Result<(
        serde_json::Value,
        serde_json::Value,
        serde_json::Value,
        serde_json::Value,
        String,
        Vec<duck::SetParam>,
    )> {
        let mut params = vec![];
        if let Some(prev_duck_story_id) = self.prev_duck_story_id {
            params.push(duck::SetParam::ConnectPrevDuckStory(
                duck::UniqueWhereParam::IdEquals(prev_duck_story_id),
            ));
        }
        if let Some(related_exhibit_id) = self.related_exhibit_id {
            params.push(duck::SetParam::ConnectRelatedExhibit(
                exhibit::UniqueWhereParam::IdEquals(related_exhibit_id),
            ));
        }
        params.push(duck::SetParam::SetIsHidden(self.is_hidden));
        Ok((
            serde_json::to_value(self.title)?,
            serde_json::to_value(self.story)?,
            serde_json::to_value(self.location)?,
            serde_json::to_value(self.topics)?,
            self.duck_icon_url.to_string(),
            params,
        ))
    }
}

impl UpdateDuckData {
    fn into_db_data(self) -> anyhow::Result<Vec<duck::SetParam>> {
        let mut params = vec![];
        if let Some(title) = self.title {
            params.push(duck::SetParam::SetTitle(serde_json::to_value(title)?));
        }
        if let Some(story) = self.story {
            params.push(duck::SetParam::SetStory(serde_json::to_value(story)?));
        }
        if let Some(location) = self.location {
            params.push(duck::SetParam::SetLocation(serde_json::to_value(location)?));
        }
        if let Some(topics) = self.topics {
            params.push(duck::SetParam::SetLocation(serde_json::to_value(topics)?));
        }
        if let Some(duck_icon_url) = self.duck_icon_url {
            params.push(duck::SetParam::SetDuckIconUrl(duck_icon_url));
        }
        if let Some(is_hidden) = self.is_hidden {
            params.push(duck::SetParam::SetIsHidden(is_hidden));
        }
        if let Some(prev_duck_story_id) = self.prev_duck_story_id {
            params.push(duck::SetParam::ConnectPrevDuckStory(
                duck::UniqueWhereParam::IdEquals(prev_duck_story_id),
            ));
        }
        if let Some(related_exhibit_id) = self.related_exhibit_id {
            params.push(duck::SetParam::ConnectRelatedExhibit(
                exhibit::UniqueWhereParam::IdEquals(related_exhibit_id),
            ));
        }
        Ok(params)
    }
}

// response struct for reading duck
duck::select! { duck_info {
    id
    title
    story
    location
    topics
    duck_icon_url
    is_hidden
    related_exhibit: select {
        location
        title
        sign
        artists
    }
    next_duck_story: select {
        id
        title
        location
        topics
        is_hidden
    }
}}

impl DB {
    // C

    pub async fn create_duck(&self, data: NewDuckData) -> anyhow::Result<duck::Data> {
        let (title, story, location, topics, duck_icon_url, params) = data.into_db_data()?;
        let data = self
            .0
            .duck()
            .create(title, story, location, topics, duck_icon_url, params)
            .exec()
            .await?;
        Ok(data)
    }

    pub async fn create_many_ducks(&self, data: Vec<NewDuckData>) -> anyhow::Result<i64> {
        let mut many_data = Vec::with_capacity(data.len());
        for duck in data {
            many_data.push(duck.into_db_data()?);
        }
        let data = self.0.duck().create_many(many_data).exec().await?;
        Ok(data)
    }

    // R

    pub async fn get_duck(&self, id: String) -> anyhow::Result<Option<duck_info::Data>> {
        let data = self
            .0
            .duck()
            .find_unique(duck::UniqueWhereParam::IdEquals(id))
            .select(duck_info::select())
            .exec()
            .await?;
        Ok(data)
    }

    pub async fn get_all_ducks(&self) -> anyhow::Result<Vec<duck_info::Data>> {
        let data = self
            .0
            .duck()
            .find_many(vec![])
            .select(duck_info::select())
            .exec()
            .await?;
        Ok(data)
    }

    // U

    pub async fn update_duck(
        &self,
        id: String,
        data: UpdateDuckData,
    ) -> anyhow::Result<duck::Data> {
        let data = self
            .0
            .duck()
            .update(duck::UniqueWhereParam::IdEquals(id), data.into_db_data()?)
            .exec()
            .await?;
        Ok(data)
    }

    // D

    pub async fn delete_duck(&self, id: String) -> anyhow::Result<duck::Data> {
        let data = self
            .0
            .duck()
            .delete(duck::UniqueWhereParam::IdEquals(id))
            .exec()
            .await?;
        Ok(data)
    }

    pub async fn delete_all_ducks(&self) -> anyhow::Result<i64> {
        let data = self.0.duck().delete_many(vec![]).exec().await?;
        Ok(data)
    }
}
