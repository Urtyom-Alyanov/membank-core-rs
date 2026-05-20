use chrono::{
  DateTime,
  Utc,
};
use uuid::Uuid;

use crate::value::quantity::{
  QuantityType,
  QuantityTypeError,
};

pub struct InventoryItem
{
  pub blueprint_id: Uuid,
  pub owner_id: i32,
  pub quantity: QuantityType,
  pub owned_since: DateTime<Utc>,
}

impl InventoryItem
{
  pub fn add(&mut self, delta: QuantityType) -> Result<(), QuantityTypeError>
  {
    match (&mut self.quantity, delta)
    {
      (QuantityType::Pieces(current), QuantityType::Pieces(amount)) =>
      {
        if amount <= 0
        {
          return Err(QuantityTypeError::InvalidDelta);
        }
        *current += amount;
      }
      (QuantityType::Weight(current), QuantityType::Weight(amount)) =>
      {
        if amount <= 0.
        {
          return Err(QuantityTypeError::InvalidDelta);
        }
        *current += amount;
      }
      (QuantityType::Volume(current), QuantityType::Volume(amount)) =>
      {
        if amount <= 0.
        {
          return Err(QuantityTypeError::InvalidDelta);
        }
        *current += amount;
      }
      _ => return Err(QuantityTypeError::MismatchedQuantityTypes),
    }
    Ok(())
  }

  pub fn consume(&mut self, delta: QuantityType) -> Result<(), QuantityTypeError>
  {
    match (&mut self.quantity, delta)
    {
      (QuantityType::Pieces(current), QuantityType::Pieces(amount)) =>
      {
        if amount <= 0
        {
          return Err(QuantityTypeError::InvalidDelta);
        }
        if *current < amount
        {
          return Err(QuantityTypeError::InsufficientQuantity { available: self.quantity,
                                                               requested: delta });
        }
        *current -= amount;
      }
      (QuantityType::Weight(current), QuantityType::Weight(amount)) =>
      {
        if amount <= 0.
        {
          return Err(QuantityTypeError::InvalidDelta);
        }
        if *current < amount
        {
          return Err(QuantityTypeError::InsufficientQuantity { available: self.quantity,
                                                               requested: delta });
        }
        *current -= amount;
      }
      (QuantityType::Volume(current), QuantityType::Volume(amount)) =>
      {
        if amount <= 0.
        {
          return Err(QuantityTypeError::InvalidDelta);
        }
        if *current < amount
        {
          return Err(QuantityTypeError::InsufficientQuantity { available: self.quantity,
                                                               requested: delta });
        }
        *current -= amount;
      }
      _ => return Err(QuantityTypeError::MismatchedQuantityTypes),
    }
    Ok(())
  }
}
