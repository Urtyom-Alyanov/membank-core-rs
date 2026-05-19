use chrono::{
  DateTime,
  Utc,
};
use serde::{
  Deserialize,
  Serialize,
};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PhotoType
{
  Item,    // 512x512 (1/1)
  Avatar,  // 128x128 (1/1)
  Cover,   // 1280x512 (5/2)
  Icon,    // 64x64 (1/1)
  Unknown, // Any other size or type
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoMetadata
{
  pub width: i32,
  pub height: i32,
  pub aspect_ratio: f64,
}

pub struct Photo
{
  // Уникальный идентификатор для каждой фотографии, генерируется при запросе загрузки (также является частью URL для загрузки и доступа к фото)
  pub id: Uuid,
  pub owner_id: i32,
  pub photo_type: PhotoType,
  pub metadata: Option<PhotoMetadata>, // Заполняется после успешного анализа загруженного файла
  pub requested_at: DateTime<Utc>,
  pub uploaded_at: Option<DateTime<Utc>>, // None - если ссылку создали, но файл еще не залили
}

impl Photo
{
  /// Запросить загрузку фото. Возвращает сущность с уникальным ID и типом, но без метаданных.
  pub fn request_upload(owner_id: i32, photo_type: PhotoType) -> Self
  {
    Self { id: Uuid::now_v7(),
           owner_id,
           photo_type,
           metadata: None,
           requested_at: Utc::now(),
           uploaded_at: None }
  }

  /// Проверить, был ли файл уже загружен для этого фото. Если uploaded_at is None, значит файл еще не загружен.
  pub fn is_uploaded(&self) -> bool { self.uploaded_at.is_some() }

  /// Завершить загрузку фото, заполнив метаданные. Это должно вызываться после успешного анализа загруженного файла.
  pub fn complete_upload(&mut self, metadata: PhotoMetadata)
  {
    self.metadata = Some(metadata);
    self.uploaded_at = Some(Utc::now());
  }
}
