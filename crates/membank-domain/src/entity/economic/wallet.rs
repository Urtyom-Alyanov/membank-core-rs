use chrono::{
  DateTime,
  Utc,
};
use membank_utils::uuid_v7::uuid_v7_with_utc_gen;
use rust_decimal::Decimal;
use thiserror::Error;
use uuid::Uuid;

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
  #[error("This name for wallet is incorrecnt")]
  IncorrectName,
}

/// Кошелёк, содержит баланс, и метаданные типа имени и описания
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
    let uuid_with_utc = uuid_v7_with_utc_gen();

    Self { id: uuid_with_utc.0,
           owner_id,
           balance,
           created_at: uuid_with_utc.1,
           name,
           description }
  }

  /// Добавить денег в кошелёк
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

  /// Вычесть деньги из кошелька
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

  /// Изменить имя кошелька
  pub fn change_name(&mut self, new_name: String) -> Result<(), WalletError>
  {
    if new_name.len() < 3
    {
      return Err(WalletError::IncorrectName);
    }
    self.name = new_name;
    Ok(())
  }

  /// Изменить описание кошелька
  pub fn change_descriprtion(&mut self, new_description: Option<String>)
  {
    self.description = new_description;
  }
}
