//! pubic api to query user states
use crate::db_api::DB;
use crate::prisma::{duck, user};

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
                story
                duck_icon_url
                is_hidden
            }
        }
    }
}}

impl DB {
    // C/R

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
        self.0
            .user()
            .upsert(
                user::UniqueWhereParam::WechatOpenIdEquals(wechat_openid.clone()),
                (wechat_openid.clone(), vec![]),
                vec![],
            )
            .exec()
            .await?;
        self.0
            .duck_history()
            .create(
                user::UniqueWhereParam::WechatOpenIdEquals(wechat_openid.clone()),
                duck::UniqueWhereParam::IdEquals(duck_id),
                vec![],
            )
            .exec()
            .await?;
        let data = self.upsert_user_info(wechat_openid).await?;
        Ok(data)
    }
}
