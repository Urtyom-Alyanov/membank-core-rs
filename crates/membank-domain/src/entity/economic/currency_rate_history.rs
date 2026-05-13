use chrono::{
  DateTime,
  Utc,
};
use rust_decimal::Decimal;

pub struct CurrencyRateHistory<'currency_code>
{
  pub id: u32,
  pub currency_id: &'currency_code str,
  pub exchange_rate_to_leuro: Decimal,
  pub timestamp: DateTime<Utc>,
}

impl<'currency_code> CurrencyRateHistory<'currency_code>
{
  pub fn new(id: u32, currency_id: &'currency_code str, exchange_rate_to_leuro: Decimal) -> Self
  {
    Self { id,
           currency_id,
           exchange_rate_to_leuro,
           timestamp: Utc::now() }
  }
}
