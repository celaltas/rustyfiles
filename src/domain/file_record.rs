use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Serialize, Deserialize)]
pub struct DBFileRecord {
    pub id: Thing,
    pub filename: String,
    // pub path: String,
    pub size: u64,
    pub mime_type: String,
    #[serde(with = "chorono_serde")]
    pub created_at: DateTime<Local>,
}

impl DBFileRecord {
    pub fn id(&self) -> String {
        self.id.id.to_string()
    }
}


#[derive(Debug, Serialize)]
pub struct FileRecord {
    pub filename: String,
    pub size: u64,
    pub mime_type: String,
    #[serde(with = "chorono_serde")]
    pub created_at: DateTime<Local>,
}

mod chorono_serde {
    use chrono::{DateTime, Local, NaiveDateTime};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";
    pub fn serialize<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT)
            .unwrap()
            .and_local_timezone(Local);
        Ok(dt.unwrap())
    }
}
