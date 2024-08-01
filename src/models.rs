// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use alloy_primitives::{Address, TxHash};
use crate::schema::posts::dsl::posts;
use diesel::prelude::*;

use diesel::Queryable;
use bigdecimal::BigDecimal;
use chrono::DateTime;
use chrono::offset::Utc;
use serde::{Deserialize, Serialize};
use bson::serde_helpers::*;

#[derive(Debug)]
#[derive(Queryable)]
#[diesel(table_name = crate::schema::following_order)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FollowingOrder {
  pub id: i64,
  pub deleted: bool,
  pub create_time: DateTime<Utc>,
  pub update_time: Option<DateTime<Utc>>,
}

#[derive(Queryable, Selectable, Insertable, Debug)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
  pub id: i32,
  pub title: String,
  pub body: String,
  pub published: bool,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::tg_user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TgUser {
  pub id: i64,
  pub deleted: bool,
  // #[serde(with = "chrono_datetime_as_bson_datetime")]
  pub create_time: DateTime<Utc>,
  pub update_time: Option<DateTime<Utc>>,
  pub address: Address,
  pub private_key: Option<String>,
  pub fee_staged: Option<BigDecimal>,
  pub fee_received: Option<BigDecimal>,
  pub parent: Option<Address>,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::trading_order)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TradingOrder {
  pub id: i64,
  pub deleted: bool,
  pub create_time: DateTime<Utc>,
  pub update_time: Option<DateTime<Utc>>,
  pub sell_or_buy: String,
  pub target_token: Address,
  pub from_token: Address,
  pub trading_uer: i64,
  pub boost_mode: bool,
  pub mev_protected: bool,
  pub priority_fee: Option<BigDecimal>,
  pub is_succeed: Option<bool>,
  pub tx_hash: Option<TxHash>,
  pub tx_receipt: Option<serde_json::Value>,
  pub target_amount: Option<BigDecimal>,
  pub from_token_amount: Option<BigDecimal>,
  pub order_type: Option<String>,
  pub pending_target_price: Option<BigDecimal>,
  pub expire_at: Option<DateTime<Utc>>,
  pub fee: Option<BigDecimal>,
}

