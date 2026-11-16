//! Клиент Deezer API на базе `reqwest` 0.13:
//! - Полный режим по куке `arl` через `gw-light.php` + `media.deezer.com/v1/get_url` (FLAC/MP3_320);
//! - Гостевой режим (`GUEST`) через публичный `api.deezer.com` (поиск, чарты, обложки и 30с MP3-превью).

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
const PUBLIC_API_URL: &str = "https://api.deezer.com";

#[derive(Debug, Clone)]
pub struct Session {
    pub api_token: String,
    pub license_token: String,
    pub user_id: String,
    pub user_name: String,
}

#[derive(Clone)]
pub struct DeezerClient {
    pub(crate) http: Client,
    pub(crate) cookie_jar: Arc<Jar>,
    pub(crate) session: Option<Session>,
}

impl DeezerClient {
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

    pub fn http(&self) -> &Client {
        &self.http
    }

    pub fn is_authenticated(&self) -> bool {
        self.session.is_some()
    }

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
            return Err(DeezerError::Auth(
                "ARL-токен недействителен или устарел (USER_ID == 0)".into(),
            ));
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

    /// Парсинг массива треков из публичного API `https://api.deezer.com`.
    fn parse_public_tracks(arr: &[serde_json::Value]) -> Vec<TrackData> {
        arr.iter()
            .filter_map(|item| {
                let id = item
                    .get("id")
                    .and_then(|v| v.as_u64().map(|n| n.to_string()))?;
                let title = item
                    .get("title")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown")
                    .to_string();
                let artist = item
                    .get("artist")
                    .and_then(|a| a.get("name"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown")
                    .to_string();
                let album = item
                    .get("album")
                    .and_then(|a| a.get("title"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown")
                    .to_string();
                let cover_url_direct = item
                    .get("album")
                    .and_then(|a| {
                        a.get("cover_big")
                            .or_else(|| a.get("cover_medium"))
                            .or_else(|| a.get("cover"))
                    })
                    .and_then(|v| v.as_str())
                    .map(str::to_string);
                let duration_secs = item.get("duration").and_then(|v| v.as_u64()).unwrap_or(30);
                let preview_url = item
                    .get("preview")
                    .and_then(|v| v.as_str())
                    .filter(|s| !s.is_empty())
                    .map(str::to_string);

                let date_add = item
                    .get("DATE_ADD")
                    .or_else(|| item.get("TIMELINE"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0);
                Some(TrackData {
                    track_id: id,
                    date_add,
                    title,
                    artist,
                    album,
                    duration: format!("{}:{:02}", duration_secs / 60, duration_secs % 60),
                    album_picture: String::new(),
                    track_token: None,
                    md5_origin: String::new(),
                    nb_listen: 0,
                    explicit_lyrics: item
                        .get("explicit_lyrics")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false),
                    fallback: None,
                    preview_url,
                    cover_url_direct,
                })
            })
            .collect()
    }

    /// Публичный поиск треков без авторизации (`https://api.deezer.com/search`).
    pub async fn search_public(&self, query: &str) -> Result<SearchResults, DeezerError> {
        let resp = self
            .http
            .get(format!("{PUBLIC_API_URL}/search"))
            .query(&[("q", query), ("limit", "40")])
            .send()
            .await
            .map_err(|e| DeezerError::Http(e.to_string()))?;

        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| DeezerError::Http(e.to_string()))?;

        let arr = body
            .get("data")
            .and_then(|d| d.as_array())
            .ok_or_else(|| DeezerError::Api("Missing 'data' in public search".into()))?;

        let data = Self::parse_public_tracks(arr);
        let total = data.len() as u64;
        Ok(SearchResults { data, total })
    }

    /// Публичный топ-чарт Deezer для заполнения экранов в режиме `GUEST`.
    pub async fn get_public_chart(&self) -> Result<Vec<TrackData>, DeezerError> {
        let resp = self
            .http
            .get(format!("{PUBLIC_API_URL}/chart/0/tracks?limit=30"))
            .send()
            .await
            .map_err(|e| DeezerError::Http(e.to_string()))?;

        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| DeezerError::Http(e.to_string()))?;

        let arr = body
            .get("data")
            .and_then(|d| d.as_array())
            .ok_or_else(|| DeezerError::Api("Missing 'data' in public chart".into()))?;

        Ok(Self::parse_public_tracks(arr))
    }

    /// Поиск треков: использует `gw-light.php` при наличии сессии, иначе публичный API Deezer.
    pub async fn search(&self, query: &str) -> Result<SearchResults, DeezerError> {
        if !self.is_authenticated() {
            return self.search_public(query).await;
        }

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

    pub async fn add_favorite(&self, song_id: &str) -> Result<(), DeezerError> {
        let params = json!({ "SNG_ID": song_id });
        self.gw_call_void("favorite_song.add", params).await
    }

    pub async fn remove_favorite(&self, song_id: &str) -> Result<(), DeezerError> {
        let params = json!({ "SNG_ID": song_id });
        self.gw_call_void("favorite_song.remove", params).await
    }

    pub async fn get_playlists(&self) -> Result<Vec<PlaylistData>, DeezerError> {
        let session = self
            .session
            .as_ref()
            .ok_or_else(|| DeezerError::Auth("Not authenticated".into()))?;

        let mut collected: Vec<PlaylistData> = Vec::new();

        // 1. Запрос через приватный gw-light.php (deezer.pageProfile)
        let params = json!({
            "user_id": session.user_id,
            "tab": "playlists",
            "nb": 2000,
        });

        if let Ok(results) = self.gw_call("deezer.pageProfile", params).await
            && let Some(arr) = results
                .get("TAB")
                .and_then(|t| t.get("playlists"))
                .and_then(|p| p.get("data"))
                .or_else(|| results.get("data"))
                .and_then(|d| d.as_array())
        {
            for item in arr {
                if let Ok(pl) = serde_json::from_value::<PlaylistData>(item.clone())
                    && !pl.playlist_id.is_empty()
                    && !pl.playlist_title.is_empty()
                {
                    collected.push(pl);
                }
            }
        }

        // 2. Дополняем / fallback через REST API https://api.deezer.com/user/{user_id}/playlists
        let public_url = format!(
            "{PUBLIC_API_URL}/user/{}/playlists?limit=100",
            session.user_id
        );
        if let Ok(resp) = self.http.get(&public_url).send().await
            && let Ok(body) = resp.json::<serde_json::Value>().await
            && let Some(arr) = body.get("data").and_then(|d| d.as_array())
        {
            for item in arr {
                if let Ok(pl) = serde_json::from_value::<PlaylistData>(item.clone())
                    && !pl.playlist_id.is_empty()
                    && !pl.playlist_title.is_empty()
                    && !collected.iter().any(|existing| existing.playlist_id == pl.playlist_id)
                {
                    collected.push(pl);
                }
            }
        }

        Ok(collected)
    }


    /// Загружает треки выбранного плейлиста:
    /// - в авторизованном режиме вызывает `playlist.getSongs` через `gw-light.php`;
    /// - в гостевом режиме (или как fallback) вызывает публичный `https://api.deezer.com/playlist/{id}/tracks`.
    pub async fn get_playlist_tracks(&self, playlist_id: &str) -> Result<Vec<TrackData>, DeezerError> {
        if self.is_authenticated() {
            let params = json!({
                "playlist_id": playlist_id,
                "start": 0,
                "nb": 2000,
            });
            if let Ok(results) = self.gw_call("playlist.getSongs", params).await {
                let data = results.get("data").unwrap_or(&results);
                if let Ok(tracks) = serde_json::from_value::<Vec<TrackData>>(data.clone())
                    && !tracks.is_empty()
                {
                    return Ok(tracks);
                }
            }
        }

        let resp = self
            .http
            .get(format!("{PUBLIC_API_URL}/playlist/{playlist_id}/tracks?limit=200"))
            .send()
            .await
            .map_err(|e| DeezerError::Http(e.to_string()))?;

        let body: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| DeezerError::Http(e.to_string()))?;

        let arr = body
            .get("data")
            .and_then(|d| d.as_array())
            .ok_or_else(|| DeezerError::Api("Missing 'data' in playlist tracks response".into()))?;

        Ok(Self::parse_public_tracks(arr))
    }

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

    pub async fn get_track(&self, song_id: &str) -> Result<TrackData, DeezerError> {
        let params = json!({ "SNG_ID": song_id, "sng_id": song_id });
        let results = self.gw_call("song.getData", params).await?;

        serde_json::from_value(results)
            .map_err(|e| DeezerError::Api(format!("Failed to parse track data: {e}")))
    }

    pub async fn get_flow(&self) -> Result<Vec<TrackData>, DeezerError> {
        if !self.is_authenticated() {
            return self.get_public_chart().await;
        }
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

    pub async fn get_track_radio(&self, track_id: &str) -> Result<Vec<TrackData>, DeezerError> {
        if self.is_authenticated() {
            let params = json!({
                "SNG_ID": track_id,
                "START": 0,
                "NB": 25,
            });
            if let Ok(results) = self.gw_call("radio.getSongRadio", params).await {
                let data = results.get("data").unwrap_or(&results);
                if let Ok(tracks) = serde_json::from_value::<Vec<TrackData>>(data.clone()) {
                    if !tracks.is_empty() {
                        return Ok(tracks);
                    }
                }
            }
        }

        // Fallback: поиск треков по текущему артисту
        if let Ok(track) = self.get_track(track_id).await {
            let q = format!("artist:\"{}\"", track.artist);
            if let Ok(results) = self.search(&q).await {
                if !results.data.is_empty() {
                    return Ok(results.data);
                }
            }
        }
        self.get_public_chart().await
    }

    pub async fn get_mood_flow(&self, mood_id: &str) -> Result<Vec<TrackData>, DeezerError> {
        if self.is_authenticated() {
            if let Some(ref session) = self.session {
                let params = json!({
                    "user_id": session.user_id,
                    "config_id": mood_id,
                });
                if let Ok(results) = self.gw_call("radio.getUserRadio", params).await {
                    let data = results.get("data").unwrap_or(&results);
                    if let Ok(tracks) = serde_json::from_value::<Vec<TrackData>>(data.clone()) {
                        if !tracks.is_empty() {
                            return Ok(tracks);
                        }
                    }
                }
            }
        }
        // Fallback: поиск по названию настроения
        if let Ok(results) = self.search(mood_id).await {
            if !results.data.is_empty() {
                return Ok(results.data);
            }
        }
        self.get_public_chart().await
    }

    pub async fn get_genre_flow(&self, genre_id: &str) -> Result<Vec<TrackData>, DeezerError> {
        if self.is_authenticated() {
            if let Some(ref session) = self.session {
                let params = json!({
                    "user_id": session.user_id,
                    "genre_id": genre_id,
                });
                if let Ok(results) = self.gw_call("radio.getUserRadio", params).await {
                    let data = results.get("data").unwrap_or(&results);
                    if let Ok(tracks) = serde_json::from_value::<Vec<TrackData>>(data.clone()) {
                        if !tracks.is_empty() {
                            return Ok(tracks);
                        }
                    }
                }
            }
        }

        // Fallback: получение радиостанций жанра через публичный API Deezer
        let radios_url = format!("https://api.deezer.com/genre/{genre_id}/radios");
        if let Ok(resp) = self.http.get(&radios_url).send().await {
            if let Ok(json) = resp.json::<serde_json::Value>().await {
                if let Some(first_radio) = json.get("data").and_then(|d| d.get(0)) {
                    if let Some(radio_id) = first_radio.get("id").and_then(|v| v.as_u64()) {
                        let tracks_url = format!("https://api.deezer.com/radio/{radio_id}/tracks");
                        if let Ok(t_resp) = self.http.get(&tracks_url).send().await {
                            if let Ok(t_json) = t_resp.json::<serde_json::Value>().await {
                                if let Some(arr) = t_json.get("data").and_then(|d| d.as_array()) {
                                    let parsed = Self::parse_public_tracks(arr);
                                    if !parsed.is_empty() {
                                        return Ok(parsed);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        self.get_public_chart().await
    }

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
