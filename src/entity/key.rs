//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use std::fmt::Debug;

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::common::crypto::types::KeyType;

#[derive(
    Clone,
    PartialEq,
    DeriveEntityModel,
    Eq,
    Serialize,
    Deserialize,
    Default,
    ToSchema,
)]
#[sea_orm(table_name = "t_key")]
pub struct Model {
    #[sea_orm(column_name = "_id", primary_key)]
    #[serde(skip)]
    pub id: i64,
    #[sea_orm(unique)]
    pub key_id: String,
    pub kms_id: String,
    pub key_type: KeyType,
    pub key_pair: Option<Json>,
    pub version: String,
    #[serde(skip)]
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub updated_at: DateTime,
    #[serde(skip_deserializing)]
    #[sea_orm(default_expr = "Expr::current_timestamp()")]
    pub created_at: DateTime,
}

impl Debug for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Model")
            .field("id", &self.id)
            .field("key_id", &self.key_id)
            .field("kms_id", &self.kms_id)
            .field("key_type", &self.key_type)
            .field("version", &self.version)
            .finish()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SymmtricKeyPair {
    pub key_pair: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AsymmtricKeyPair {
    pub private_key: String,
    pub public_key: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
