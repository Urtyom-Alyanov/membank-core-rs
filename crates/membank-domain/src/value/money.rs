use std::ops::{
  Add,
  Div,
  Mul,
  Sub,
};

use rust_decimal::Decimal;
use uuid::Uuid;

#[derive(Clone, Debug, Copy)]
pub struct Money
{
  pub amount: Decimal,
  pub currency_id: Uuid,
}

impl Money
{
  pub fn new(amount: Decimal, currency_id: Uuid) -> Self
  {
    Self { amount,
           currency_id }
  }

  pub fn zero(currency_id: Uuid) -> Self
  {
    Self { amount: Decimal::ZERO,
           currency_id }
  }

  pub fn try_add(self, other: Self) -> Result<Self, String>
  {
    if self.currency_id != other.currency_id
    {
      return Err("Несоответствие валют при сложении".into());
    }
    Ok(Self::new(self.amount + other.amount, self.currency_id))
  }
}

impl Add for Money
{
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output
  {
    assert_eq!(self.currency_id, rhs.currency_id,
               "Attempted to add different currencies");
    Self::new(self.amount + rhs.amount, self.currency_id)
  }
}

impl Sub for Money
{
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output
  {
    assert_eq!(self.currency_id, rhs.currency_id,
               "Attempted to subtract different currencies");
    Self::new(self.amount - rhs.amount, self.currency_id)
  }
}

impl Mul<Decimal> for Money
{
  type Output = Self;
  fn mul(self, rhs: Decimal) -> Self::Output { Self::new(self.amount * rhs, self.currency_id) }
}

impl Div<Decimal> for Money
{
  type Output = Self;
  fn div(self, rhs: Decimal) -> Self::Output { Self::new(self.amount / rhs, self.currency_id) }
}

impl std::fmt::Display for Money
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "{} (currency {})", self.amount, self.currency_id)
  }
}
