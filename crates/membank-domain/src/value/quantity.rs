use thiserror::Error;

#[derive(Error, Debug)]
pub enum QuantityTypeError
{
  #[error("Mismatched quantity types.")]
  MismatchedQuantityTypes,
  #[error("Not enough items in inventory. Available: {available:?}, requested: {requested:?}")]
  InsufficientQuantity
  {
    available: QuantityType,
    requested: QuantityType,
  },
  #[error("Quantity delta must be positive")]
  InvalidDelta,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum QuantityType
{
  Pieces(i32),
  Weight(f64),
  Volume(f64),
}
