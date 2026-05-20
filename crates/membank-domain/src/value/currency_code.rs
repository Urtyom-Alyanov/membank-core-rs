use std::fmt::Display;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CurrencyCodeError
{
  #[error("Currency code uses not ASCII symbols")]
  NotAscii,
  #[error("Currency code length is {current_len}. Required: 3")]
  Length
  {
    current_len: usize
  },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CurrencyCode(pub String);

impl Into<String> for CurrencyCode
{
  fn into(self) -> String { self.0 }
}

impl Display for CurrencyCode
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.0) }
}

impl TryFrom<String> for CurrencyCode
{
  type Error = CurrencyCodeError;
  fn try_from(code: String) -> Result<Self, CurrencyCodeError>
  {
    let current_len = code.len();
    if code.len() != 3
    {
      return Err(CurrencyCodeError::Length { current_len });
    }

    if !code.chars().all(|c| c.is_ascii_uppercase())
    {
      return Err(CurrencyCodeError::NotAscii);
    }

    Ok(Self(code.to_uppercase()))
  }
}
