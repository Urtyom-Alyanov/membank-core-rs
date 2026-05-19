use chrono::{
  DateTime,
  Utc,
};
use membank_utils::uuid_v7::uuid_v7_with_utc_gen;
use rust_decimal::Decimal;
use serde::{
  Deserialize,
  Serialize,
};
use uuid::Uuid;

use crate::value::money::Money;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TransferNoteType
{
  Personal,
  Buying,
  Tax,
  Emission,
  Subscription,
}

pub struct TransferNote
{
  pub id: Uuid,
  pub note_type: TransferNoteType,
  pub sender_id: Uuid,   // Wallet ID
  pub receiver_id: Uuid, // Wallet ID
  pub amount: Money,
  pub exchange_rate_at_transfer: Decimal,
  pub noted_at: DateTime<Utc>,
  pub description: Option<String>,
}

impl TransferNote
{
  pub fn new(sender_id: Uuid,
             receiver_id: Uuid,
             amount: Money,
             current_rate: Decimal,
             note_type: TransferNoteType,
             description: Option<String>)
             -> Self
  {
    let uuid_with_utc = uuid_v7_with_utc_gen();

    Self { id: uuid_with_utc.0,
           note_type,
           sender_id,
           receiver_id,
           amount,
           exchange_rate_at_transfer: current_rate,
           noted_at: uuid_with_utc.1,
           description }
  }
}
