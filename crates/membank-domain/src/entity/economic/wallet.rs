use chrono::{
  DateTime,
  Utc,
};
use rust_decimal::Decimal;
use thiserror::Error;
use uuid::{
  NoContext,
  Uuid,
  timestamp,
};

use crate::value::money::Money;

#[derive(Error, Debug)]
pub enum WalletError
{
  #[error("Insufficient funds: balance {balance}, required {required}")]
  InsufficientFunds
  {
    balance: Money, required: Money
  },
  #[error("Currency mismatch")]
  CurrencyMismatch,
  #[error("Amount must be positive")]
  InvalidAmount,
}

pub struct Wallet
{
  id: Uuid,
  owner_id: i32,
  balance: Money,
  created_at: DateTime<Utc>,
  name: String,
  description: Option<String>,
}

impl Wallet
{
  pub fn new(owner_id: i32, balance: Money, name: String, description: Option<String>) -> Self
  {
    let created_at = Utc::now();
    let timestamp = timestamp::Timestamp::from_unix(NoContext,
                                                    created_at.timestamp() as u64,
                                                    created_at.timestamp_subsec_nanos());

    Self { id: Uuid::new_v7(timestamp),
           owner_id,
           balance,
           created_at,
           name,
           description }
  }

  pub fn deposit(&mut self, amount: Money) -> Result<Money, WalletError>
  {
    if amount.amount <= Decimal::ZERO
    {
      return Err(WalletError::InvalidAmount);
    }
    if self.balance.currency_id != amount.currency_id
    {
      return Err(WalletError::CurrencyMismatch);
    }
    self.balance.amount += amount.amount;
    Ok(self.balance)
  }

  pub fn withdraw(&mut self, amount: Money) -> Result<Money, WalletError>
  {
    if amount.amount <= Decimal::ZERO
    {
      return Err(WalletError::InvalidAmount);
    }
    if self.balance.currency_id != amount.currency_id
    {
      return Err(WalletError::CurrencyMismatch);
    }
    if self.balance.amount < amount.amount
    {
      return Err(WalletError::InsufficientFunds { balance: self.balance,
                                                  required: amount });
    }

    self.balance.amount -= amount.amount;
    Ok(self.balance)
  }
}
