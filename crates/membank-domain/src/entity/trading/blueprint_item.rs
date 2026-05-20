use chrono::{
  DateTime,
  Utc,
};
use membank_utils::uuid_v7::uuid_v7_with_utc_gen;
use thiserror::Error;
use uuid::Uuid;

use crate::{
  entity::core::photo::PhotoId,
  value::quantity::QuantityType,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlueprintId(Uuid);

impl From<Uuid> for BlueprintId
{
  fn from(value: Uuid) -> Self { Self(value) }
}

impl Into<Uuid> for BlueprintId
{
  fn into(self) -> Uuid { self.0 }
}

// АВТОРСКОЕ ПРАВО

pub struct Copyright
{
  pub owner_id: i32,
  pub authorized_accounts: Vec<i32>,
  pub registered_at: DateTime<Utc>,
  pub expires_at: Option<DateTime<Utc>>,
}

pub enum BlueprintRegulation
{
  OpenRecipe,
  Copyrighted(Copyright),
}

// КРАФТИНГ

pub struct CraftStation
{
  pub max_health: i32,
}

pub struct Ingredient
{
  pub blueprint_id: BlueprintId, // На какой предмет ссылаемся
  pub amount: QuantityType,      // Сколько штук/килограмм/литров нужно
}

pub struct CraftInfo
{
  pub crafting_station_id: Option<BlueprintId>, // На какой станции крафтится (например, ID Верстака)
  pub station_wear: i32,                        // Насколько изнашивается станция
  pub crafting_time: i32,                       // Время крафта в секундах
  pub recipe: Vec<Ingredient>,                  // Список ингредиентов
}

// ПРЕДМЕТ

#[derive(Error, Debug)]
pub enum BlueprintError
{
  #[error("Только владелец авторских прав ({owner}) может выдать доступ")]
  NotAnOwner
  {
    owner: i32
  },
  #[error("Рецепт является открытым, управление авторскими правами недоступно")]
  RecipeIsOpen,
  #[error("Предмет не содержит рецепта для крафта")]
  NoRecipe,
  #[error("Предмет не может требовать самого себя в качестве ингредиента")]
  SelfReferencingIngredient,
}

/// Сущность, представляющая предмет, который может быть скрафчен или продан. Содержит информацию о его свойствах, рецепте и регулировании использования.
pub struct BlueprintItem
{
  /// Уникальный идентификатор предмета, генерируется при создании. Используется для ссылок в рецептах и инвентаре.
  pub id: BlueprintId,

  /// Название предмета, например "Меч из Млечного Пути"
  pub name: String,

  /// Описание предмета, может содержать информацию о его свойствах, истории и т.д.
  pub description: String,

  /// Идентификатор фотографии, которая служит иконкой предмета. Ссылается на сущность Photo в системе.
  pub photo_id: PhotoId,

  /// Дата и время создания предмета, устанавливается при генерации UUIDv7.
  pub created_at: DateTime<Utc>,

  /// Базовая единица измерения для этого предмета. Например, для "Меча" это может быть "1 штука", а для "Зелья" - "100 мл".
  pub base_measured: QuantityType,

  /// Регулирование использования рецепта. Открытые рецепты доступны всем, а защищенные требуют наличия прав.
  pub regulation: BlueprintRegulation,

  /// Свойства, связанные с крафтингом. Если None, то предмет не может быть использован в рецептах и не имеет свойств станции.
  pub station_properties: Option<CraftStation>,

  /// Информация о крафте, включая необходимые ингредиенты и время. Если None, то предмет не может быть скрафчен.
  pub craft_info: Option<CraftInfo>,

  /// Редкость предмета, от 0 до 255, где 0 - обычный предмет, а 255 - легендарный. Это может влиять на его стоимость, шанс выпадения и т.д.
  pub rarity: u8,
}

impl BlueprintItem
{
  /// Создать новый предмет. По умолчанию, рецепт будет открытым, без информации о крафте и без свойств станции.
  pub fn new(name: String,
             description: String,
             photo_id: PhotoId,
             base_measured: QuantityType)
             -> Self
  {
    let uuid_with_utc = uuid_v7_with_utc_gen();

    Self { id: BlueprintId(uuid_with_utc.0),
           name,
           description,
           photo_id,
           created_at: uuid_with_utc.1,
           base_measured,
           regulation: BlueprintRegulation::OpenRecipe,
           station_properties: None,
           craft_info: None,
           rarity: 0 }
  }

  /// Проверить, может ли пользователь использовать этот рецепт (для крафта или продажи).
  /// Открытые рецепты доступны всем, а для защищенных проверяется наличие прав.
  pub fn is_usable_by(&self, user_id: i32) -> bool
  {
    match &self.regulation
    {
      BlueprintRegulation::OpenRecipe => true,
      BlueprintRegulation::Copyrighted(copyright) =>
      {
        copyright.owner_id == user_id || copyright.authorized_accounts.contains(&user_id)
      }
    }
  }

  /// Выдать право использовать рецепт другому пользователю. Доступно только для защищенных рецептов и только владельцу прав.
  pub fn grant_copyright(&mut self,
                         manager_id: i32,
                         target_user_id: i32)
                         -> Result<(), BlueprintError>
  {
    match &mut self.regulation
    {
      BlueprintRegulation::OpenRecipe => Err(BlueprintError::RecipeIsOpen),
      BlueprintRegulation::Copyrighted(copyright) =>
      {
        if copyright.owner_id != manager_id
        {
          return Err(BlueprintError::NotAnOwner { owner: copyright.owner_id });
        }
        if !copyright.authorized_accounts.contains(&target_user_id)
        {
          copyright.authorized_accounts.push(target_user_id);
        }
        Ok(())
      }
    }
  }

  /// Добавить или обновить ингредиент в рецепте. Если рецепт еще не существует, он будет создан. Доступно только для предметов с рецептом.
  pub fn add_ingredient(&mut self,
                        ingredient_id: BlueprintId,
                        amount: QuantityType)
                        -> Result<(), BlueprintError>
  {
    if self.id == ingredient_id
    {
      return Err(BlueprintError::SelfReferencingIngredient);
    }

    if self.craft_info.is_none()
    {
      self.craft_info = Some(CraftInfo { crafting_station_id: None,
                                         station_wear: 0,
                                         crafting_time: 10,
                                         recipe: Vec::new() });
    }

    if let Some(info) = &mut self.craft_info
    {
      if let Some(existing) = info.recipe
                                  .iter_mut()
                                  .find(|i| i.blueprint_id == ingredient_id)
      {
        existing.amount = amount;
      }
      else
      {
        info.recipe.push(Ingredient { blueprint_id: ingredient_id,
                                      amount });
      }
    }

    Ok(())
  }
}
