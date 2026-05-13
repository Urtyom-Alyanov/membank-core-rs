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
}

impl std::fmt::Display for Money
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
  {
    write!(f, "{} (currency {})", self.amount, self.currency_id)
  }
}
