use chrono::{
  DateTime,
  Utc,
};
use rust_decimal::Decimal;
use thiserror::Error;
use uuid::Uuid;

use crate::entity::economic::currency_rate_history_item::CurrencyRateHistoryItem;

#[derive(Error, Debug)]
pub enum CurrencyError
{
  #[error("Invalid currency code: {0}")]
  InvalidCode(String),
  #[error("Exchange rate must be positive")]
  InvalidExchangeRate,
  #[error("Operation amount must be positive")]
  InvalidAmount,
  #[error("Cannot burn {requested} units of currency. Total supply: {supply}")]
  InsufficientSupplyForBurn
  {
    requested: Decimal, supply: Decimal
  },
}

/// Валюта, которая может быть использована в системе.
/// Каждая валюта имеет код (например, TEH, KEK), название, эмитента (может быть организаций или государствjv),
/// иконку, а также обменный курс к Левро (MLC) - базовой валюте системы.
pub struct Currency
{
  pub code: String,
  pub issuer_id: u32,
  pub name: String,
  pub exchange_rate_to_leuro: Decimal,
  pub total_supply: Decimal,
  pub monochrome_icon_id: Option<Uuid>,
  pub created_at: DateTime<Utc>,
  pub last_rate_update: DateTime<Utc>,
  pub legal_tender_ids: Vec<u32>,
}

impl Currency
{
  /// Создать новую валюту. Код должен быть 3 символа, а обменный курс должен быть положительным числом.
  pub fn new(code: String,
             name: String,
             exchange_rate_to_leuro: Decimal,
             issuer_id: u32,
             symbol_icon_id: Option<Uuid>)
             -> Result<Self, CurrencyError>
  {
    if code.len() != 3
    {
      return Err(CurrencyError::InvalidCode(code));
    }

    if exchange_rate_to_leuro <= Decimal::ZERO
    {
      return Err(CurrencyError::InvalidExchangeRate);
    }

    Ok(Self { code: code.to_uppercase(),
              name,
              exchange_rate_to_leuro,
              issuer_id,
              monochrome_icon_id: symbol_icon_id,

              created_at: Utc::now(),
              last_rate_update: Utc::now(),
              total_supply: Decimal::new(0, 0),
              legal_tender_ids: Vec::new() })
  }

  /// Добавить страну, где это является легальным средством платежей
  pub fn add_legal_tender(&mut self, legal_tender_id: u32)
  {
    if !self.legal_tender_ids.contains(&legal_tender_id)
    {
      self.legal_tender_ids.push(legal_tender_id);
    }
  }

  /// Удалить страну, где это является легальным средством платежей
  pub fn remove_legal_tender(&mut self, legal_tender_id: u32)
  {
    self.legal_tender_ids.retain(|&id| id != legal_tender_id);
  }

  /// Изменить эмитента валюты
  pub fn change_issuer(&mut self, new_issuer_id: u32) { self.issuer_id = new_issuer_id; }

  /// Изменить название валюты
  pub fn change_name(&mut self, new_name: String) { self.name = new_name; }

  /// Изменить иконку валюты. Если new_icon_id = None, то иконка будет удалена.
  pub fn change_icon(&mut self, new_icon_id: Option<Uuid>)
  {
    self.monochrome_icon_id = new_icon_id;
  }

  /// Обновить обменный курс к Левро. Должен быть положительным числом.
  pub fn update_exchange_rate<'a>(&'a mut self,
                                  new_rate: Decimal)
                                  -> Result<CurrencyRateHistoryItem, CurrencyError>
  {
    if new_rate <= Decimal::ZERO
    {
      return Err(CurrencyError::InvalidExchangeRate);
    }
    self.exchange_rate_to_leuro = new_rate;
    self.last_rate_update = Utc::now();
    Ok(CurrencyRateHistoryItem::new(self.code.clone(), new_rate))
  }

  /// Эмитировать новую валюту, увеличивая общий объем предложения. Должно быть положительным числом.
  pub fn mint(&mut self, amount: Decimal) -> Result<(), CurrencyError>
  {
    self.total_supply += amount;
    Ok(())
  }

  /// Сжечь валюту, уменьшая общий объем предложения. Должно быть положительным числом.
  pub fn burn(&mut self, amount: Decimal) -> Result<(), CurrencyError>
  {
    if amount <= Decimal::ZERO
    {
      return Err(CurrencyError::InvalidAmount);
    }
    if amount > self.total_supply
    {
      return Err(CurrencyError::InsufficientSupplyForBurn { requested: amount,
                                                            supply: self.total_supply });
    }
    self.total_supply -= amount;
    Ok(())
  }
}
