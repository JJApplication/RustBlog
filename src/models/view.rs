use async_trait::async_trait;
use chrono::Utc;
use sea_orm::entity::prelude::*;
use sea_orm::ActiveValue::{NotSet, Set};
use serde::Serialize;

fn now_utc() -> DateTimeUtc {
    Utc::now().into()
}

/// 访问量实体
#[derive(Clone, Debug, PartialEq, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "blog_views")]
pub struct Model {
    /// 主键
    #[sea_orm(primary_key, column_name = "primary_id")]
    pub id: i32,
    /// 创建时间
    pub create_at: DateTimeUtc,
    /// 更新时间
    pub update_at: DateTimeUtc,
    /// 创建人ID
    pub create_by: i32,
    /// 更新人ID
    pub update_by: i32,
    /// 文章名（通过name与post关联，不使用外键）
    pub name: String,
    /// 访问量
    pub view: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    /// 保存前自动维护审计字段
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        self.update_at = Set(now_utc());
        if insert {
            if matches!(self.create_at, NotSet) {
                self.create_at = Set(now_utc());
            }
            if matches!(self.create_by, NotSet) {
                self.create_by = Set(0);
            }
        }
        if matches!(self.update_by, NotSet) {
            self.update_by = Set(0);
        }
        Ok(self)
    }
}
