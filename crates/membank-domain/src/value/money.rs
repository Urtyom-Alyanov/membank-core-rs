use std::ops::{
  Add,
  AddAssign,
  Div,
  Mul,
  Sub,
  SubAssign,
};

use rust_decimal::Decimal;
use thiserror::Error;

use crate::value::currency_code::CurrencyCode;

#[derive(Error, Debug)]
pub enum MoneyError
{
  #[error("Operation with different currencies")]
  CurrencyDifference,
}

#[derive(Clone, Debug)]
pub struct Money
{
  pub amount: Decimal,
  pub currency_code: CurrencyCode,
}

impl Money
{
  pub fn new(amount: Decimal, currency_code: CurrencyCode) -> Self
  {
    Self { amount,
           currency_code }
  }

  pub fn zero(currency_code: CurrencyCode) -> Self
  {
    Self { amount: Decimal::ZERO,
           currency_code }
  }

  pub fn try_add(self, other: Self) -> Result<Self, MoneyError>
  {
    if self.currency_code != other.currency_code
    {
      return Err(MoneyError::CurrencyDifference);
    }
    Ok(Self::new(self.amount + other.amount, self.currency_code))
  }
}

impl Add for Money
{
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output
  {
    assert_eq!(self.currency_code, rhs.currency_code,
               "Operation with different currencies");
    Self::new(self.amount + rhs.amount, self.currency_code)
  }
}

impl AddAssign for Money
{
  fn add_assign(&mut self, rhs: Self)
  {
    assert_eq!(self.currency_code, rhs.currency_code,
               "Operation with different currencies");
    *self = Self { amount: self.amount + rhs.amount,
                   currency_code: self.currency_code.clone() }
  }
}

impl Sub for Money
{
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output
  {
    assert_eq!(self.currency_code, rhs.currency_code,
               "Operation with different currencies");
    Self::new(self.amount - rhs.amount, self.currency_code)
  }
}

impl SubAssign for Money
{
  fn sub_assign(&mut self, rhs: Self)
  {
    assert_eq!(self.currency_code, rhs.currency_code,
               "Operation with different currencies");
    *self = Self { amount: self.amount - rhs.amount,
                   currency_code: self.currency_code.clone() }
  }
}

impl Mul<Decimal> for Money
{
  type Output = Self;
  fn mul(self, rhs: Decimal) -> Self::Output { Self::new(self.amount * rhs, self.currency_code) }
}

impl Div<Decimal> for Money
{
  type Output = Self;
  fn div(self, rhs: Decimal) -> Self::Output { Self::new(self.amount / rhs, self.currency_code) }
}

impl std::fmt::Display for Money
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "{} (currency {})", self.amount, self.currency_code)
  }
}
