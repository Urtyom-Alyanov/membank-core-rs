use chrono::{
  DateTime,
  Utc,
};
use membank_utils::uuid_v7::uuid_v7_with_utc_gen;
use thiserror::Error;
use uuid::Uuid;

use crate::entity::core::photo::PhotoId;

#[derive(Error, Debug)]
pub enum CategoryError
{
  #[error("Название категории не может быть пустым или состоять только из пробелов")]
  EmptyName,
}

pub struct Category
{
  pub id: Uuid,
  pub name: String,
  pub chromo_icon_id: Option<PhotoId>,

  pub created_at: DateTime<Utc>,
  pub changed_at: DateTime<Utc>,
}

impl Category
{
  pub fn new(name: String, chromo_icon_id: Option<PhotoId>) -> Result<Self, CategoryError>
  {
    if name.trim().is_empty()
    {
      return Err(CategoryError::EmptyName);
    }

    let uuid_with_utc = uuid_v7_with_utc_gen();

    Ok(Self { id: uuid_with_utc.0,
              name,
              chromo_icon_id,
              created_at: uuid_with_utc.1,
              changed_at: uuid_with_utc.1 })
  }

  /// Изменить название категории
  pub fn change_name(&mut self, new_name: String) -> Result<(), CategoryError>
  {
    if new_name.trim().is_empty()
    {
      return Err(CategoryError::EmptyName);
    }

    self.name = new_name;
    self.changed_at = Utc::now();
    Ok(())
  }

  /// Изменить иконку категории. Если new_icon_id = None, то иконка будет удалена.
  pub fn change_icon(&mut self, new_icon_id: Option<PhotoId>)
  {
    self.chromo_icon_id = new_icon_id;
    self.changed_at = Utc::now();
  }
}
