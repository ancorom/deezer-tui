//! Клиент Deezer API на базе `reqwest` 0.13: авторизация по куке `arl` в `gw-light.php`
//! (`deezer.getUserData`), получение URL потока в `media.deezer.com/v1/get_url`,
//! методы поиска, плейлистов, лайков, Flow и Lyrics.

#![allow(dead_code)]

use std::sync::Arc;

use reqwest::{cookie::Jar, Client};
use serde_json::json;
use tracing::{debug, warn};

use super::models::{
    AudioQuality, DeezerError, LyricLine, Lyrics, PlaylistData, SearchResults, TrackData, UserData,
};

const GW_LIGHT_URL: &str = "https://www.deezer.com/ajax/gw-light.php";
const DEEZER_URL: &str = "https://www.deezer.com";
const MEDIA_URL: &str = "https://media.deezer.com/v1/get_url";

/// Сессия Deezer после `deezer.getUserData`.
#[derive(Debug, Clone)]
pub struct Session {
    /// Токен `checkForm` для вызовов `gw-light.php`.
    pub api_token: String,
    /// Лицензионный токен для `media.deezer.com/v1/get_url`.
    pub license_token: String,
    /// ID пользователя.
    pub user_id: String,
    pub user_name: String,
}

/// Асинхронный клиент внутреннего API Deezer.
pub struct DeezerClient {
    pub(crate) http: Client,
    pub(crate) cookie_jar: Arc<Jar>,
    pub(crate) session: Option<Session>,
}

impl DeezerClient {
    /// Создаёт новый HTTP-клиент с хранилищем кук.
    pub fn new() -> Result<Self, DeezerError> {
        let cookie_jar = Arc::new(Jar::default());

        let http = Client::builder()
            .cookie_provider(Arc::clone(&cookie_jar))
            .user_agent(
                "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
            )
            .timeout(std::time::Duration::from_secs(30))
            .connect_timeout(std::time::Duration::from_secs(10))
            .build()
            .map_err(|e| DeezerError::Http(e.to_string()))?;

        Ok(Self {
            http,
            cookie_jar,
            session: None,
        })
    }

    /// Возвращает ссылку на внутренний `reqwest::Client`.
    pub fn http(&self) -> &Client {
        &self.http
    }

    /// Проверяет, пройдена ли аутентификация.
    pub fn is_authenticated(&self) -> bool {
        self.session.is_some()
    }

    /// Возвращает текущую сессию.
    pub fn session(&self) -> Option<&Session> {
        self.session.as_ref()
    }

    /// Аутентификация по куке `arl` через `deezer.getUserData`.
    pub async fn login_arl(&mut self, arl: &str) -> Result<Session, DeezerError> {
        debug!(arl_len = arl.len(), "Authenticating with ARL token");

        let cookie = format!("arl={}; Domain=.deezer.com; Path=/", arl.trim());
        if let Ok(base_url) = DEEZER_URL.parse() {
            self.cookie_jar.add_cookie_str(&cookie, &base_url);
        }

        let url =
            format!("{GW_LIGHT_URL}?method=deezer.getUserData&input=3&api_version=1.0&api_token=");

        let resp = self
            .http
            .post(&url)
            .json(&json!({}))
            .send()
            .await
            .map_err(|e| DeezerError::Http(e.to_string()))?;

        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| DeezerError::Http(e.to_string()))?;

        let results = body
            .get("results")
            .ok_or_else(|| DeezerError::Api("Missing 'results' in getUserData response".into()))?;

        let user_data: UserData = serde_json::from_value(results.clone())
            .map_err(|e| DeezerError::Api(format!("Failed to parse user data: {e}")))?;

        if user_data.user.user_id == "0" || user_data.api_token.is_empty() {
            return Err(DeezerError::Auth("Invalid or expired ARL token".into()));
        }

        let session = Session {
            api_token: user_data.api_token,
            license_token: user_data.user.options.license_token,
            user_id: user_data.user.user_id,
            user_name: user_data.user.user_name,
        };

        self.session = Some(session.clone());
        Ok(session)
    }

    fn api_token(&self) -> Result<&str, DeezerError> {
        self.session
            .as_ref()
            .map(|s| s.api_token.as_str())
            .ok_or_else(|| DeezerError::Auth("Not authenticated".into()))
    }

    async fn gw_call(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, DeezerError> {
        let api_token = self.api_token()?;
        let url =
            format!("{GW_LIGHT_URL}?method={method}&input=3&api_version=1.0&api_token={api_token}");

        debug!(method, "Gateway API call");

        let resp = self
            .http
            .post(&url)
            .json(&params)
            .send()
            .await
            .map_err(|e| DeezerError::Http(e.to_string()))?;

        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| DeezerError::Http(e.to_string()))?;

        if let Some(error) = body
            .get("error")
            .filter(|e| e.as_object().is_some_and(|o| !o.is_empty()))
        {
            return Err(DeezerError::Api(error.to_string()));
        }

        body.get("results")
            .cloned()
            .ok_or_else(|| DeezerError::Api("Missing 'results' in response".into()))
    }

    async fn gw_call_void(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<(), DeezerError> {
        let _ = self.gw_call(method, params).await?;
        Ok(())
    }

    /// Поиск треков через `deezer.pageSearch`.
    pub async fn search(&self, query: &str) -> Result<SearchResults, DeezerError> {
        let params = json!({
            "query": query,
            "filter": "ALL",
            "output": "TRACK",
            "start": 0,
            "nb": 40,
        });

        let results = self.gw_call("deezer.pageSearch", params).await?;

        let track_section = results
            .get("TRACK")
            .ok_or_else(|| DeezerError::Api("Missing 'TRACK' in search results".into()))?;

        let track_array = track_section.get("data").unwrap_or(track_section);

        let data: Vec<TrackData> = serde_json::from_value(track_array.clone())
            .map_err(|e| DeezerError::Api(format!("Failed to parse search results: {e}")))?;

        let total = track_section
            .get("total")
            .and_then(|v| v.as_u64())
            .unwrap_or(data.len() as u64);

        Ok(SearchResults { data, total })
    }

    /// Получение избранных треков (`favorite_song.getList`).
    pub async fn get_favorites(&self) -> Result<Vec<TrackData>, DeezerError> {
        let session = self
            .session
            .as_ref()
            .ok_or_else(|| DeezerError::Auth("Not authenticated".into()))?;

        let params = json!({
            "user_id": session.user_id,
            "start": 0,
            "nb": 2000,
        });

        let results = self.gw_call("favorite_song.getList", params).await?;
        let data = results.get("data").unwrap_or(&results);

        serde_json::from_value(data.clone())
            .map_err(|e| DeezerError::Api(format!("Failed to parse favorites: {e}")))
    }

    /// Добавление трека в избранное (`favorite_song.add`).
    pub async fn add_favorite(&self, song_id: &str) -> Result<(), DeezerError> {
        let params = json!({ "SNG_ID": song_id });
        self.gw_call_void("favorite_song.add", params).await
    }

    /// Удаление трека из избранного (`favorite_song.remove`).
    pub async fn remove_favorite(&self, song_id: &str) -> Result<(), DeezerError> {
        let params = json!({ "SNG_ID": song_id });
        self.gw_call_void("favorite_song.remove", params).await
    }

    /// Получение списка плейлистов пользователя (`deezer.pageProfile`).
    pub async fn get_playlists(&self) -> Result<Vec<PlaylistData>, DeezerError> {
        let session = self
            .session
            .as_ref()
            .ok_or_else(|| DeezerError::Auth("Not authenticated".into()))?;

        let params = json!({
            "user_id": session.user_id,
            "tab": "playlists",
            "nb": 2000,
        });

        let results = self.gw_call("deezer.pageProfile", params).await?;

        let data = results
            .get("TAB")
            .and_then(|t| t.get("playlists"))
            .and_then(|p| p.get("data"))
            .ok_or_else(|| DeezerError::Api("Missing TAB.playlists.data in profile".into()))?;

        serde_json::from_value(data.clone())
            .map_err(|e| DeezerError::Api(format!("Failed to parse playlists: {e}")))
    }

    /// Добавление треков в плейлист (`playlist.addSongs`).
    pub async fn add_to_playlist(
        &self,
        playlist_id: &str,
        song_ids: &[&str],
    ) -> Result<(), DeezerError> {
        let songs_tuple: Vec<(&str, u8)> = song_ids.iter().map(|&id| (id, 0)).collect();
        let params = json!({
            "playlist_id": playlist_id,
            "songs": songs_tuple,
        });
        self.gw_call_void("playlist.addSongs", params).await
    }

    /// Создание нового плейлиста (`playlist.create`).
    pub async fn create_playlist(&self, title: &str) -> Result<String, DeezerError> {
        let session = self
            .session
            .as_ref()
            .ok_or_else(|| DeezerError::Auth("Not authenticated".into()))?;

        let params = json!({
            "playlist_title": title,
            "owner_id": session.user_id,
        });
        let results = self.gw_call("playlist.create", params).await?;

        let playlist_id = results
            .get("playlist_id")
            .and_then(|v| {
                v.as_str()
                    .map(str::to_string)
                    .or_else(|| v.as_u64().map(|n| n.to_string()))
            })
            .ok_or_else(|| DeezerError::PlaylistUnavailable("Missing playlist_id".into()))?;

        Ok(playlist_id)
    }

    /// Удаление треков из плейлиста (`playlist.deleteSongs`).
    pub async fn remove_from_playlist(
        &self,
        playlist_id: &str,
        song_ids: &[&str],
    ) -> Result<(), DeezerError> {
        let songs_tuple: Vec<(&str, u8)> = song_ids.iter().map(|&id| (id, 0)).collect();
        let params = json!({
            "playlist_id": playlist_id,
            "songs": songs_tuple,
        });
        self.gw_call_void("playlist.deleteSongs", params).await
    }

    /// Получение метаданных трека (`song.getData`).
    pub async fn get_track(&self, song_id: &str) -> Result<TrackData, DeezerError> {
        let params = json!({ "SNG_ID": song_id, "sng_id": song_id });
        let results = self.gw_call("song.getData", params).await?;

        serde_json::from_value(results)
            .map_err(|e| DeezerError::Api(format!("Failed to parse track data: {e}")))
    }

    /// Получение очереди Deezer Flow (`radio.getUserRadio`).
    pub async fn get_flow(&self) -> Result<Vec<TrackData>, DeezerError> {
        let session = self
            .session
            .as_ref()
            .ok_or_else(|| DeezerError::Auth("Not authenticated".into()))?;

        let params = json!({ "user_id": session.user_id });
        let results = self.gw_call("radio.getUserRadio", params).await?;

        let data = results
            .get("data")
            .ok_or_else(|| DeezerError::Api("Missing 'data' in flow response".into()))?;

        serde_json::from_value(data.clone())
            .map_err(|e| DeezerError::Api(format!("Failed to parse flow: {e}")))
    }

    /// Получение текста песни и синхронизированных таймкодов (`song.getLyrics`).
    pub async fn get_lyrics(&self, track_id: &str) -> Result<Lyrics, DeezerError> {
        let session = self
            .session
            .as_ref()
            .ok_or_else(|| DeezerError::Auth("Not authenticated".into()))?;

        let params = json!({
            "SNG_ID": track_id,
            "sng_id": track_id,
            "user_id": session.user_id,
        });
        let results = self.gw_call("song.getLyrics", params).await?;
        let results_val = results.get("results").unwrap_or(&results);

        let plain = results_val
            .get("LYRICS_TEXT")
            .and_then(|v| v.as_str())
            .map(str::to_string)
            .unwrap_or_default();

        let synced: Vec<LyricLine> = results_val
            .get("LYRICS_SYNC_JSON")
            .and_then(|v| v.as_array())
            .map_or_else(Vec::new, |arr| {
                arr.iter()
                    .filter_map(|item| {
                        let line = item
                            .get("line")
                            .or_else(|| item.get("Line"))
                            .and_then(|v| v.as_str())
                            .map(str::to_string)?;
                        if line.trim().is_empty() {
                            return None;
                        }
                        let time_ms = item
                            .get("milliseconds")
                            .or_else(|| item.get("TimeMS"))
                            .and_then(|v| {
                                v.as_u64()
                                    .or_else(|| v.as_str().and_then(|s| s.parse().ok()))
                            })
                            .unwrap_or(0);
                        Some(LyricLine { time_ms, line })
                    })
                    .collect()
            });

        if plain.is_empty() && synced.is_empty() {
            return Err(DeezerError::NoLyrics);
        }

        Ok(Lyrics { plain, synced })
    }

    /// Получение прямой ссылки на зашифрованный поток (`media.deezer.com/v1/get_url`).
    pub async fn get_stream_url(
        &self,
        track: &TrackData,
        quality: AudioQuality,
    ) -> Result<(String, AudioQuality), DeezerError> {
        let session = self
            .session
            .as_ref()
            .ok_or_else(|| DeezerError::Auth("Not authenticated".into()))?;

        let track_token = track
            .track_token
            .as_deref()
            .ok_or_else(|| DeezerError::TrackUnavailable("Missing TRACK_TOKEN".into()))?;

        for q in quality.all_fallbacks() {
            let fmt = q.as_api_format();
            debug!(quality = fmt, track_id = %track.track_id, "Requesting stream URL");

            let payload = json!({
                "license_token": session.license_token,
                "media": [{
                    "type": "FULL",
                    "formats": [{
                        "cipher": "BF_CBC_STRIPE",
                        "format": fmt,
                    }]
                }],
                "track_tokens": [track_token],
            });

            let resp = self
                .http
                .post(MEDIA_URL)
                .json(&payload)
                .send()
                .await
                .map_err(|e| DeezerError::Http(e.to_string()))?;

            let body: serde_json::Value = resp
                .json()
                .await
                .map_err(|e| DeezerError::Http(e.to_string()))?;

            if let Some(url) = body
                .get("data")
                .and_then(|d| d.get(0))
                .and_then(|d| d.get("media"))
                .and_then(|m| m.get(0))
                .and_then(|m| m.get("sources"))
                .and_then(|s| s.get(0))
                .and_then(|s| s.get("url"))
                .and_then(|u| u.as_str())
            {
                return Ok((url.to_string(), q));
            }

            warn!(
                quality = fmt,
                track_id = %track.track_id,
                "get_stream_url: format unavailable, trying fallback"
            );
        }

        Err(DeezerError::QualityUnavailable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_client() {
        let client = DeezerClient::new().expect("new client");
        assert!(!client.is_authenticated());
    }

    #[test]
    fn test_search_results_default() {
        let results: SearchResults = serde_json::from_str("{}").unwrap();
        assert!(results.data.is_empty());
    }

    #[test]
    fn test_parse_track_search_payload() {
        let track = serde_json::json!([
            {
                "SNG_ID": 51300437,
                "SNG_TITLE": "Bohemian Rhapsody",
                "ART_NAME": "Queen",
                "ALB_TITLE": "A Night at the Opera",
                "DURATION": "05:55",
                "ALB_PICTURE": "https://e-cdns-images.dzcdn.net/images/cover/abc/264x264.jpg",
                "TRACK_TOKEN": "tok-bohemian",
                "MD5_ORIGIN": "0123456789abcdef0123456789abcdef",
                "NB_LISTEN": 1_234_567,
                "EXPLICIT_LYRICS": false
            },
            {
                "SNG_ID": 51300438,
                "SNG_TITLE": "Under Pressure",
                "ART_NAME": "Queen",
                "ALB_TITLE": "Hot Space",
                "DURATION": "03:51",
                "ALB_PICTURE": "https://e-cdns-images.dzcdn.net/images/cover/xyz/264x264.jpg",
                "MD5_ORIGIN": "fedcba9876543210fedcba9876543210",
                "NB_LISTEN": 888_000,
                "EXPLICIT_LYRICS": true
            }
        ]);
        let data: Vec<TrackData> = serde_json::from_value(track).unwrap();
        assert_eq!(data.len(), 2);
        assert_eq!(data[0].track_id, "51300437");
        assert!(data[0].has_track_token());
        assert!(!data[1].has_track_token());
    }

    #[test]
    fn test_parse_user_data_payload() {
        let payload = serde_json::json!({
            "USER": {
                "USER_ID": 51300437,
                "BLOG_NAME": "queen_fan",
                "OPTIONS": {
                    "license_token": "lic-tok",
                    "web_streaming": true,
                    "web_hq": true,
                    "web_lossless": false
                }
            },
            "checkForm": "cf-token",
            "OFFER": { "OFFER_NAME": "premium" }
        });
        let user: UserData = serde_json::from_value(payload).unwrap();
        assert_eq!(user.user.user_id, "51300437");
        assert_eq!(user.user.user_name, "queen_fan");
        assert_eq!(user.api_token, "cf-token");
    }

    #[test]
    fn test_parse_lyric_line() {
        let line: LyricLine = serde_json::from_value(serde_json::json!({
            "TimeMS": 1_234,
            "line": "Is this the real life?"
        }))
        .unwrap();
        assert_eq!(line.time_ms, 1_234);
        assert_eq!(line.line, "Is this the real life?");
    }
}
