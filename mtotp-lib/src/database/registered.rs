use std::ops::Deref;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, EntityTrait, IntoActiveModel};
use sea_orm::ActiveValue::Set;
use crate::database::{CONNECT, create_index, create_table_if_not_exists, index_exists};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "registered")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub uuid: String,
    pub label: String,
    pub secret: String,
    pub issuer: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub(crate) async fn init() {
    let db = CONNECT.get().unwrap().lock().await;
    create_table_if_not_exists(db.deref(), Entity).await;
    if !index_exists(db.deref(), "registered", "idx_label").await {
        create_index(db.deref(), "registered", vec!["label"], "idx_label").await;
    }
    if !index_exists(db.deref(), "registered", "idx_issuer").await {
        create_index(db.deref(), "registered", vec!["issuer"], "idx_issuer").await;
    }
}

pub async fn find_all() -> Vec<Model> {
    let db = CONNECT.get().unwrap().lock().await;
    Entity::find().all(db.deref()).await.unwrap()
}

pub async fn insert(model: Model) {
    let db = CONNECT.get().unwrap().lock().await;
    model.into_active_model().insert(db.deref()).await.unwrap();
}

pub async fn delete_by_uuid(uuid: &str) {
    let db = CONNECT.get().unwrap().lock().await;
    Entity::delete_by_id(uuid).exec(db.deref()).await.unwrap();
}

pub async fn update_label_by_uuid(
    uuid: &str,
    label: &str,
) {
    let db = CONNECT.get().unwrap().lock().await;
    let mut am = ActiveModel::new();
    am.label = Set(label.to_string());
    Entity::update_many()
        .filter(Column::Uuid.eq(uuid))
        .set(am)
        .exec(db.deref())
        .await
        .unwrap();
}

