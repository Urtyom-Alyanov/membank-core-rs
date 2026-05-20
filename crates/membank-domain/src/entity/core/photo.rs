use chrono::{
  DateTime,
  Utc,
};
use membank_utils::uuid_v7::uuid_v7_with_utc_gen;
use serde::{
  Deserialize,
  Serialize,
};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum PhotoError
{
  #[error("Photo is uploaded at {uploaded_at}")]
  Uploaded
  {
    uploaded_at: DateTime<Utc>
  },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhotoId(pub Uuid);

impl PhotoId
{
  pub fn to_url(&self, base_url: &str) -> String { format!("{}/{}.webp", base_url, self.0) }
}

impl Into<Uuid> for PhotoId
{
  fn into(self) -> Uuid { self.0 }
}

impl From<Uuid> for PhotoId
{
  fn from(uuid: Uuid) -> Self { Self(uuid) }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PhotoType
{
  Item,    // 512x512 (1/1)
  Avatar,  // 128x128 (1/1)
  Cover,   // 1280x512 (5/2)
  Icon,    // 64x64 (1/1)
  Unknown, // Any other size or type
}

pub enum UploadState
{
  Requested
  {
    requested_at: DateTime<Utc>
  },

  Uploaded
  {
    requested_at: DateTime<Utc>,
    uploaded_at: DateTime<Utc>,
    metadata: PhotoMetadata,
  },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoMetadata
{
  pub width: i32,
  pub height: i32,
}

impl PhotoMetadata
{
  pub fn aspect_ratio(&self) -> f64
  {
    let width_f64: f64 = self.width.into();
    let height_f64: f64 = self.height.into();

    width_f64 / height_f64
  }
}

pub struct Photo
{
  // Уникальный идентификатор для каждой фотографии, генерируется при запросе загрузки (также является частью URL для загрузки и доступа к фото)
  pub id: PhotoId,
  pub owner_id: i32,
  pub photo_type: PhotoType,
  pub upload: UploadState,
}

impl Photo
{
  /// Запросить загрузку фото. Возвращает сущность с уникальным ID и типом, но без метаданных.
  pub fn request_upload(owner_id: i32, photo_type: PhotoType) -> Self
  {
    let uuid_with_utc = uuid_v7_with_utc_gen();

    Self { id: PhotoId(uuid_with_utc.0),
           owner_id,
           photo_type,
           upload: UploadState::Requested { requested_at: uuid_with_utc.1 } }
  }

  /// Завершить загрузку фото, заполнив метаданные. Это должно вызываться после успешного анализа загруженного файла.
  pub fn complete_upload(&mut self, metadata: PhotoMetadata) -> Result<(), PhotoError>
  {
    match self.upload
    {
      UploadState::Requested { requested_at } =>
      {
        self.upload = UploadState::Uploaded { requested_at,
                                              uploaded_at: Utc::now(),
                                              metadata };
        Ok(())
      }
      UploadState::Uploaded { uploaded_at, .. } => Err(PhotoError::Uploaded { uploaded_at }),
    }
  }
}
