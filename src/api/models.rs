//! Модели Deezer API: типы данных и ошибки.

#![allow(dead_code)]

use serde::Deserialize;
use thiserror::Error;

/// Ошибки Deezer API.
#[derive(Debug, Error)]
pub enum DeezerError {
    #[error("HTTP error: {0}")]
    Http(String),
    #[error("API error: {0}")]
    Api(String),
    #[error("Authentication failed: {0}")]
    Auth(String),
    #[error("Decryption error: {0}")]
    Decrypt(String),
    #[error("Quality unavailable")]
    QualityUnavailable,
    #[error("Track unavailable: {0}")]
    TrackUnavailable(String),
    #[error("No lyrics found")]
    NoLyrics,
    #[error("Playlist unavailable: {0}")]
    PlaylistUnavailable(String),
}

/// Качество аудио.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Default)]
pub enum AudioQuality {
    #[serde(rename = "MP3_64")]
    Mp3_64,
    #[default]
    #[serde(rename = "MP3_128")]
    Mp3_128,
    #[serde(rename = "MP3_320")]
    Mp3_320,
    #[serde(rename = "FLAC")]
    Flac,
}

impl AudioQuality {
    pub fn as_api_format(&self) -> &'static str {
        match self {
            Self::Mp3_64 => "MP3_64",
            Self::Mp3_128 => "MP3_128",
            Self::Mp3_320 => "MP3_320",
            Self::Flac => "FLAC",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Mp3_64 => "MP3 | 64 kbps",
            Self::Mp3_128 => "MP3 | 128 kbps",
            Self::Mp3_320 => "MP3 | 320 kbps",
            Self::Flac => "FLAC | 1411 kbps",
        }
    }

    pub const ALL: &'static [AudioQuality] =
        &[Self::Mp3_64, Self::Mp3_128, Self::Mp3_320, Self::Flac];

    pub fn fallback(&self) -> Option<Self> {
        match self {
            Self::Flac => Some(Self::Mp3_320),
            Self::Mp3_320 => Some(Self::Mp3_128),
            Self::Mp3_128 => Some(Self::Mp3_64),
            Self::Mp3_64 => None,
        }
    }

    pub fn all_fallbacks(&self) -> Vec<Self> {
        match self {
            Self::Mp3_64 => vec![Self::Mp3_64],
            Self::Mp3_128 => vec![Self::Mp3_128, Self::Mp3_64],
            Self::Mp3_320 => vec![Self::Mp3_320, Self::Mp3_128, Self::Mp3_64],
            Self::Flac => vec![Self::Flac, Self::Mp3_320, Self::Mp3_128, Self::Mp3_64],
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserData {
    #[serde(rename = "USER")]
    pub user: UserInfo,
    #[serde(rename = "checkForm")]
    pub api_token: String,
    #[serde(rename = "OFFER")]
    #[serde(default)]
    pub offer: Option<UserOffer>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserInfo {
    #[serde(rename = "USER_ID")]
    #[serde(deserialize_with = "deserialize_id_to_string")]
    pub user_id: String,
    #[serde(rename = "BLOG_NAME")]
    #[serde(default)]
    pub user_name: String,
    #[serde(rename = "OPTIONS")]
    pub options: UserOptions,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserOptions {
    pub license_token: String,
    #[serde(rename = "web_streaming")]
    #[serde(default)]
    pub web_streaming: bool,
    #[serde(rename = "web_hq")]
    #[serde(default)]
    pub web_hq: bool,
    #[serde(rename = "web_lossless")]
    #[serde(default)]
    pub web_lossless: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserOffer {
    #[serde(rename = "OFFER_NAME")]
    #[serde(default)]
    pub offer_name: String,
}

/// Данные трека Deezer.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct TrackData {
    #[serde(rename = "SNG_ID")]
    #[serde(deserialize_with = "deserialize_id_to_string")]
    #[serde(default)]
    pub track_id: String,
    #[serde(rename = "SNG_TITLE")]
    #[serde(default)]
    pub title: String,
    #[serde(rename = "ART_NAME")]
    #[serde(default)]
    pub artist: String,
    #[serde(rename = "ALB_TITLE")]
    #[serde(default)]
    pub album: String,
    #[serde(rename = "DURATION")]
    #[serde(deserialize_with = "deserialize_id_to_string")]
    #[serde(default)]
    pub duration: String,
    #[serde(rename = "ALB_PICTURE")]
    #[serde(default)]
    pub album_picture: String,
    #[serde(rename = "TRACK_TOKEN")]
    #[serde(default)]
    pub track_token: Option<String>,
    #[serde(rename = "MD5_ORIGIN")]
    #[serde(default)]
    pub md5_origin: String,
    #[serde(rename = "NB_LISTEN")]
    #[serde(default)]
    pub nb_listen: u64,
    #[serde(rename = "EXPLICIT_LYRICS")]
    #[serde(deserialize_with = "deserialize_bool_flexible")]
    #[serde(default)]
    pub explicit_lyrics: bool,
    #[serde(rename = "FALLBACK")]
    #[serde(default)]
    pub fallback: Option<TrackFallback>,
    /// Прямая ссылка на 30-секундное превью (для режима ГОСТЯ или при отсутствии HiFi).
    #[serde(skip)]
    pub preview_url: Option<String>,
    /// Прямая ссылка на обложку (из публичного API).
    #[serde(skip)]
    pub cover_url_direct: Option<String>,
}

impl TrackData {
    pub fn has_track_token(&self) -> bool {
        self.track_token.as_deref().is_some_and(|t| !t.is_empty())
    }

    /// Форматирует длительность в `MM:SS` (если пришло число секунд вроде `"235"`).
    pub fn formatted_duration(&self) -> String {
        if self.duration.contains(':') {
            return self.duration.clone();
        }
        if let Ok(secs) = self.duration.parse::<u64>() {
            return format!("{}:{:02}", secs / 60, secs % 60);
        }
        if self.duration.is_empty() {
            "0:30".to_string()
        } else {
            self.duration.clone()
        }
    }

    /// Длительность трека в секундах.
    pub fn duration_secs(&self) -> u64 {
        if let Ok(secs) = self.duration.parse::<u64>() {
            return secs.max(1);
        }
        if let Some((m, s)) = self.duration.split_once(':') {
            let mins = m.parse::<u64>().unwrap_or(0);
            let secs = s.parse::<u64>().unwrap_or(0);
            return (mins * 60 + secs).max(1);
        }
        30
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TrackFallback {
    #[serde(rename = "SNG_ID")]
    #[serde(deserialize_with = "deserialize_id_to_string")]
    #[serde(default)]
    pub track_id: String,
    #[serde(rename = "SNG_TITLE")]
    #[serde(default)]
    pub title: String,
    #[serde(rename = "ART_NAME")]
    #[serde(default)]
    pub artist: String,
    #[serde(rename = "ALB_TITLE")]
    #[serde(default)]
    pub album: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchResults {
    #[serde(default)]
    pub data: Vec<TrackData>,
    #[serde(default)]
    pub total: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PlaylistData {
    #[serde(rename = "PLAYLIST_ID", alias = "LIST_ID")]
    #[serde(deserialize_with = "deserialize_id_to_string")]
    #[serde(default)]
    pub playlist_id: String,
    #[serde(rename = "TITLE", alias = "PLAYLIST_TITLE")]
    #[serde(default)]
    pub playlist_title: String,
    #[serde(rename = "NB_SONG")]
    #[serde(default)]
    pub num_tracks: u64,
    #[serde(rename = "DATE_ADD", alias = "CREATED_AT")]
    #[serde(default)]
    pub created_at: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LyricLine {
    #[serde(rename = "TimeMS")]
    #[serde(default)]
    pub time_ms: u64,
    pub line: String,
}

#[derive(Debug, Clone, Default)]
pub struct Lyrics {
    pub plain: String,
    pub synced: Vec<LyricLine>,
}

fn deserialize_id_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de;
    struct IdToString;
    impl<'de> de::Visitor<'de> for IdToString {
        type Value = String;
        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("a string or number")
        }
        fn visit_u64<E: de::Error>(self, v: u64) -> Result<String, E> {
            Ok(v.to_string())
        }
        fn visit_i64<E: de::Error>(self, v: i64) -> Result<String, E> {
            Ok(v.to_string())
        }
        fn visit_str<E: de::Error>(self, v: &str) -> Result<String, E> {
            Ok(v.to_string())
        }
    }
    deserializer.deserialize_any(IdToString)
}

fn deserialize_bool_flexible<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de;
    struct BoolFlex;
    impl<'de> de::Visitor<'de> for BoolFlex {
        type Value = bool;
        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("a boolean, integer, or string")
        }
        fn visit_bool<E: de::Error>(self, v: bool) -> Result<bool, E> {
            Ok(v)
        }
        fn visit_u64<E: de::Error>(self, v: u64) -> Result<bool, E> {
            Ok(v != 0)
        }
        fn visit_i64<E: de::Error>(self, v: i64) -> Result<bool, E> {
            Ok(v != 0)
        }
        fn visit_str<E: de::Error>(self, v: &str) -> Result<bool, E> {
            Ok(v == "1" || v.eq_ignore_ascii_case("true"))
        }
    }
    deserializer.deserialize_any(BoolFlex)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_track_data_deserialization_number_id() {
        let track: TrackData = serde_json::from_str(
            r#"{"SNG_ID":3135556,"SNG_TITLE":"Around the World","ART_NAME":"Daft Punk","ALB_TITLE":"Discovery","DURATION":"4:00","TRACK_TOKEN":"AAAAA"}"#,
        )
        .unwrap();
        assert_eq!(track.track_id, "3135556");
        assert_eq!(track.title, "Around the World");
        assert!(track.has_track_token());
    }

    #[test]
    fn test_track_data_deserialization_string_id() {
        let track: TrackData = serde_json::from_str(r#"{"SNG_ID":"abc","SNG_TITLE":"x"}"#).unwrap();
        assert_eq!(track.track_id, "abc");
    }

    #[test]
    fn test_user_id_deserialization() {
        let data: UserData = serde_json::from_str(
            r#"{"USER":{"USER_ID":123456789,"OPTIONS":{"license_token":"tok"}},"checkForm":"api"}"#,
        )
        .unwrap();
        assert_eq!(data.user.user_id, "123456789");
        assert_eq!(data.api_token, "api");
    }

    #[test]
    fn test_audio_quality_format() {
        assert_eq!(AudioQuality::Flac.as_api_format(), "FLAC");
        assert_eq!(AudioQuality::Mp3_128.as_api_format(), "MP3_128");
    }
}
