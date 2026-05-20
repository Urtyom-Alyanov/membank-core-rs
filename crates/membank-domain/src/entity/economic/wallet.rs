use chrono::{
  DateTime,
  Utc,
};
use membank_utils::uuid_v7::uuid_v7_with_utc_gen;
use rust_decimal::Decimal;
use thiserror::Error;
use uuid::Uuid;

use crate::value::money::Money;

pub struct WalletId(Uuid);

impl From<Uuid> for WalletId
{
  fn from(value: Uuid) -> Self { Self(value) }
}

impl Into<Uuid> for WalletId
{
  fn into(self) -> Uuid { self.0 }
}

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
  pub id: WalletId,
  pub owner_id: i32,
  pub balance: Money,
  pub created_at: DateTime<Utc>,
  pub name: String,
  pub description: Option<String>,
}

impl Wallet
{
  pub fn new(owner_id: i32,
             balance: Money,
             name: String,
             description: Option<String>)
             -> Result<Self, WalletError>
  {
    if name.len() < 3
    {
      return Err(WalletError::IncorrectName);
    }

    let uuid_with_utc = uuid_v7_with_utc_gen();

    Ok(Self { id: WalletId::from(uuid_with_utc.0),
              owner_id,
              balance,
              created_at: uuid_with_utc.1,
              name,
              description })
  }

  /// Добавить денег в кошелёк
  pub fn deposit(&mut self, amount: Money) -> Result<Money, WalletError>
  {
    if amount.amount <= Decimal::ZERO
    {
      return Err(WalletError::InvalidAmount);
    }
    self.balance += amount;
    Ok(self.balance.clone())
  }

  /// Вычесть деньги из кошелька
  pub fn withdraw(&mut self, amount: Money) -> Result<Money, WalletError>
  {
    if amount.amount <= Decimal::ZERO
    {
      return Err(WalletError::InvalidAmount);
    }
    if self.balance.amount < amount.amount
    {
      return Err(WalletError::InsufficientFunds { balance: self.balance.clone(),
                                                  required: amount });
    }

    self.balance.amount -= amount.amount;
    Ok(self.balance.clone())
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
