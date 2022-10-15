//! pubic api to query user states
use crate::db_api::DB;
use crate::prisma::{duck, duck_history, user};

duck::select! { duck_preview {
    title
    location: select {
        id
        coordinate
    }
    topics
    is_hidden
}}

user::select! { user_info {
    id
    created_at
    wechat_open_id
    duck_history: select {
        created_at
        duck: select {
            id
            title
            story
            location: select {
                id
                coordinate
                description
            }
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
                location: select {
                    id
                    coordinate
                }
                topics
                is_hidden
            }
        }
    }
}}

impl DB {
    // C/R

    pub async fn preview_ducks(&self) -> anyhow::Result<Vec<duck_preview::Data>> {
        let data = self
            .0
            .duck()
            .find_many(vec![])
            .select(duck_preview::select())
            .exec()
            .await?;
        Ok(data)
    }

    pub async fn upsert_user_info(&self, wechat_openid: String) -> anyhow::Result<user_info::Data> {
        let data = self
            .0
            .user()
            .upsert(
                user::UniqueWhereParam::WechatOpenIdEquals(wechat_openid.clone()),
                (wechat_openid, vec![]),
                vec![],
            )
            .select(user_info::select())
            .exec()
            .await?;
        Ok(data)
    }

    pub async fn record_duck_view(
        &self,
        wechat_openid: String,
        duck_id: String,
    ) -> anyhow::Result<user_info::Data> {
        // try creating new user if wechat_openid is new
        let user = self
            .0
            .user()
            .upsert(
                user::UniqueWhereParam::WechatOpenIdEquals(wechat_openid.clone()),
                (wechat_openid.clone(), vec![]),
                vec![],
            )
            .exec()
            .await?;
        // create new record if no record exists
        self.0
            .duck_history()
            .upsert(
                duck_history::UniqueWhereParam::UserIdDuckIdEquals(
                    user.id.clone(),
                    duck_id.clone(),
                ),
                (
                    user::UniqueWhereParam::IdEquals(user.id),
                    duck::UniqueWhereParam::IdEquals(duck_id),
                    vec![],
                ),
                vec![],
            )
            .exec()
            .await?;
        let data = self.upsert_user_info(wechat_openid).await?;
        Ok(data)
    }
}
