use chrono::{
  DateTime,
  Utc,
};

#[derive(Debug, Clone, PartialEq)]
pub enum SettingValue
{
  Boolean(bool),
  String(String),
  Integer(i32),
  Decimal(f64),
}

pub struct Setting
{
  pub key: String,
  pub display_name: String,
  pub value: SettingValue,

  pub created_at: DateTime<Utc>,
  pub changed_at: DateTime<Utc>,
  pub changed_by_id: Option<i32>,
}

impl Setting
{
  /// Создать новую настройку. Ключ должен быть уникальным, а значение не должно быть null.
  pub fn new(key: String,
             value: SettingValue,
             display_name: Option<String>,
             changed_by_id: Option<i32>)
             -> Self
  {
    let now = Utc::now();
    Self { key: key.clone(),
           display_name: display_name.unwrap_or(key),
           value,
           created_at: now,
           changed_at: now,
           changed_by_id }
  }

  /// Обновить значение настройки. Если новое значение такое же, то ничего не происходит.
  pub fn set_value(&mut self, new_value: SettingValue, changer_id: Option<i32>)
  {
    if self.value == new_value
    {
      return;
    }
    self.value = new_value;
    self.changed_by_id = changer_id;
    self.changed_at = Utc::now();
  }
}
