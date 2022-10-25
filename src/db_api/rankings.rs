//! admin api to manage rankings
use crate::db_api::DB;
use crate::prisma::{ranking, user};

impl DB {
    pub async fn upsert_ranking(&self, wechat_id: String) -> anyhow::Result<ranking::Data> {
        let new_ranking = self.0.ranking().count(vec![]).exec().await? + 1;
        let data = self
            .0
            .ranking()
            .upsert(
                ranking::UniqueWhereParam::UserWechatOpenIdEquals(wechat_id.clone()),
                (
                    user::UniqueWhereParam::WechatOpenIdEquals(wechat_id),
                    new_ranking as i32,
                    vec![],
                ),
                vec![],
            )
            .exec()
            .await?;
        Ok(data)
    }

    // R
    pub async fn get_all_rankings(&self) -> anyhow::Result<Vec<ranking::Data>> {
        let data = self.0.ranking().find_many(vec![]).exec().await?;
        Ok(data)
    }

    // D
    pub async fn delete_all_rankings(&self) -> anyhow::Result<i64> {
        let data = self.0.ranking().delete_many(vec![]).exec().await?;
        Ok(data)
    }
}
