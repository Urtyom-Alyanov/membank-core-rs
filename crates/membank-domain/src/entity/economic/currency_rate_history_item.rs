use chrono::{
  DateTime,
  Utc,
};
use membank_utils::uuid_v7::uuid_v7_with_utc_gen;
use rust_decimal::Decimal;
use uuid::Uuid;

use crate::value::currency_code::CurrencyCode;

pub struct CurrencyRateHistoryItem
{
  pub id: Uuid,
  pub currency_id: CurrencyCode,
  pub exchange_rate_to_leuro: Decimal,
  pub timestamp: DateTime<Utc>,
}

impl CurrencyRateHistoryItem
{
  pub fn new(currency_id: CurrencyCode, exchange_rate_to_leuro: Decimal) -> Self
  {
    let uuid_with_utc = uuid_v7_with_utc_gen();

    Self { id: uuid_with_utc.0,
           currency_id,
           exchange_rate_to_leuro,
           timestamp: uuid_with_utc.1 }
  }
}
