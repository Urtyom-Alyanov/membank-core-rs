use chrono::{
  DateTime,
  Utc,
};
use uuid::{
  NoContext,
  Uuid,
  timestamp,
};

pub struct UuidWithUtc(pub Uuid, pub DateTime<Utc>);

pub fn uuid_v7_with_utc_gen() -> UuidWithUtc
{
  let created_at = Utc::now();
  let timestamp = timestamp::Timestamp::from_unix(NoContext,
                                                  created_at.timestamp() as u64,
                                                  created_at.timestamp_subsec_nanos());

  let uuid = Uuid::new_v7(timestamp);

  UuidWithUtc(uuid, created_at)
}
