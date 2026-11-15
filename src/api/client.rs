//! Клиент Deezer API: аутентификация по ARL, поиск, плейлисты, лайки,
//! Flow, Lyrics и получение URL потока.
//
use std::sync::Arc;

use reqwest::{Client, cookie::Jar};
use serde_json::json;
use tracing::{debug, warn};

use super::models::{
    AudioQuality, DeezerError, Lyrics, LyricLine, PlaylistData, SearchResults, TrackData, UserData,
};

const GW_LIGHT_URL: &str = "https://www.deezer.com/ajax/gw-light.php";
const DEEZER_URL: &str = "https://www.deezer.com";
const MEDIA_URL: &str = "https://media.deezer.com/v1/get_url";

/// Сессия Deezer после `getUserData`.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Session {
    /// Токен для gw-light.php запросов.
    pub api_token: String,
    /// Лицензионный токен для `media.deezer.com/v1/get_url`.
    pub license_token: String,
    /// ID пользователя (строка).
    pub user_id: String,
    pub user_name: String,
}

/// Клиент Deezer API.
pub struct DeezerClient {
    pub(crate) http: Client,
    pub(crate) cookie_jar: Arc<Jar>,
    pub(crate) session: Option<Session>,
}

impl DeezerClient {
    /// Создаёт новый клиент с cookie-паром.
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

    /// Возвращает `Client` (для внешних HTTP-запросов, например `get_stream_url`).
    #[allow(dead_code)]
    pub fn http(&self) -> &Client {
        &self.http
    }

    /// Есть ли активная сессия.
    pub fn is_authenticated(&self) -> bool {
        self.session.is_some()
    }

    /// Текущая сессия (если есть).
    #[allow(dead_code)]
    pub fn session(&self) -> Option<&Session> {
        self.session.as_ref()
    }

    /// Аутентификация по ARL-токену: вставляет `arl`-куку в jar и вызывает
    /// `deezer.getUserData`. Сервер также поднимает `sid`-куку, которую jar
    /// сам сохраняет для следующих запросов.
    #[allow(dead_code)]
    pub async fn login_arl(&mut self, arl: &str) -> Result<Session, DeezerError> {
        debug!(arl_len = arl.len(), "Authenticating with ARL token");

        let cookie = format!("arl={arl}; Domain=.deezer.com; Path=/");
        self.cookie_jar
            .add_cookie_str(&cookie, &DEEZER_URL.parse().unwrap());

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
            .ok_or_else(|| DeezerError::Api("Missing 'results' in response".into()))?;

        let user_data: UserData = serde_json::from_value(results.clone())
            .map_err(|e| DeezerError::Api(format!("Failed to parse user data: {e}")))?;

        let session = Session {
            api_token: user_data.api_token,
            license_token: user_data.user.options.license_token,
            user_id: user_data.user.user_id,
            user_name: user_data.user.user_name,
        };

        let offer_name = user_data
            .offer
            .as_ref()
            .map_or("unknown", |o| &o.offer_name);

        debug!(
            user_id = session.user_id,
            name = %session.user_name,
            offer = %offer_name,
            web_streaming = user_data.user.options.web_streaming,
            web_hq = user_data.user.options.web_hq,
            web_lossless = user_data.user.options.web_lossless,
            "Authenticated successfully"
        );

        self.session = Some(session.clone());
        Ok(session)
    }

    /// Токен API из сессии.
    fn api_token(&self) -> Result<&str, DeezerError> {
        self.session
            .as_ref()
            .map(|s| s.api_token.as_str())
            .ok_or_else(|| DeezerError::Auth("Not authenticated".into()))
    }

    /// POST-запрос к gw-light.php. Возвращает `results`.
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

    /// POST-запрос к gw-light.php для write-операций (без `results`).
    async fn gw_call_void(
        &self,
        method: &str,
        params: serde_json::Value,
    ) -> Result<(), DeezerError> {
        let api_token = self.api_token()?;
        let url =
            format!("{GW_LIGHT_URL}?method={method}&input=3&api_version=1.0&api_token={api_token}");

        debug!(method, "Gateway API call (void)");

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

        Ok(())
    }

    /// Поиск треков.
    #[allow(dead_code)]
    pub async fn search(&self, query: &str) -> Result<SearchResults, DeezerError> {
        let params = json!({
            "query": query,
            "filter": "ALL",
            "output": "TRACK",
            "start": 0,
            "nb": 40,
        });

        let results = self.gw_call("deezer.pageSearch", params).await?;

        let track_results = results
            .get("TRACK")
            .ok_or_else(|| DeezerError::Api("Missing 'TRACK' in search results".into()))?;

        let data: Vec<TrackData> = serde_json::from_value(track_results.clone())
            .map_err(|e| DeezerError::Api(format!("Failed to parse search results: {e}")))?;

        let total = data.len() as u64;
        Ok(SearchResults { data, total })
    }

    /// Понравившиеся (лайки) треки.
    #[allow(dead_code)]
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

        let data = results
            .get("data")
            .ok_or_else(|| DeezerError::Api("Missing 'data' in favorites response".into()))?;

        serde_json::from_value(data.clone())
            .map_err(|e| DeezerError::Api(format!("Failed to parse favorites: {e}")))
    }

    /// Добавить трек в лайки.
    #[allow(dead_code)]
    pub async fn add_favorite(&self, song_id: &str) -> Result<(), DeezerError> {
        let params = json!({ "SNG_ID": song_id });
        self.gw_call_void("favorite_song.add", params).await
    }

    /// Удалить трек из лайков.
    #[allow(dead_code)]
    pub async fn remove_favorite(&self, song_id: &str) -> Result<(), DeezerError> {
        let params = json!({ "SNG_ID": song_id });
        self.gw_call_void("favorite_song.remove", params).await
    }

    /// Плейлисты пользователя (из профиля).
    #[allow(dead_code)]
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

    /// Добавить трек в плейлист.
    #[allow(dead_code)]
    pub async fn add_to_playlist(
        &self,
        playlist_id: &str,
        song_ids: &[&str],
    ) -> Result<(), DeezerError> {
        let params = json!({
            "playlist_id": playlist_id,
            "songs": song_ids,
        });
        self.gw_call_void("playlist.addSongs", params).await
    }

    /// Создать плейлист.
    #[allow(dead_code)]
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
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| DeezerError::PlaylistUnavailable("Missing playlist_id".into()))?;

        Ok(playlist_id)
    }

    /// Удалить трек из плейлиста.
    #[allow(dead_code)]
    pub async fn remove_from_playlist(
        &self,
        playlist_id: &str,
        song_ids: &[&str],
    ) -> Result<(), DeezerError> {
        let params = json!({
            "playlist_id": playlist_id,
            "songs": song_ids,
        });
        self.gw_call_void("playlist.deleteSongs", params).await
    }

    /// Получить полные данные трека (с TRACK_TOKEN).
    #[allow(dead_code)]
    pub async fn get_track(&self, song_id: &str) -> Result<TrackData, DeezerError> {
        let params = json!({ "sng_id": song_id });
        let results = self.gw_call("song.getData", params).await?;

        serde_json::from_value(results)
            .map_err(|e| DeezerError::Api(format!("Failed to parse track data: {e}")))
    }

    /// Получить Deezer Flow.
    #[allow(dead_code)]
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

    /// Получить лирики (plain + synced).
    #[allow(dead_code)]
    pub async fn get_lyrics(&self, track_id: &str) -> Result<Lyrics, DeezerError> {
        let session = self
            .session
            .as_ref()
            .ok_or_else(|| DeezerError::Auth("Not authenticated".into()))?;

        let params = json!({
            "sng_id": track_id,
            "user_id": session.user_id,
        });
        let results = self.gw_call("song.getLyrics", params).await?;

        let results_val = results
            .get("results")
            .ok_or(DeezerError::NoLyrics)?;

        let plain = results_val
            .get("LYRICS_TEXT")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_default();

        // LYRICS_SYNC_JSON: массив { TimeMS: u64, Line: str }.
        let synced: Vec<LyricLine> = results_val
            .get("LYRICS_SYNC_JSON")
            .and_then(|v| v.as_array())
            .map_or(
                Vec::new(),
                |arr| {
                    arr.iter()
                        .map(|item| {
                            let time_ms = item
                                .get("TimeMS")
                                .and_then(|v| {
                                    v.as_u64()
                                        .or_else(|| v.as_str().and_then(|s| s.parse().ok()))
                                })
                                .unwrap_or_default();
                            let line = item
                                .get("Line")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string())
                                .unwrap_or_default();
                            LyricLine { time_ms, line }
                        })
                        .collect()
                },
            );

        Ok(Lyrics { plain, synced })
    }

    /// Получить URL потока для трека с фолбэком качества.
    #[allow(dead_code)]
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

        let qualities_to_try: Vec<(AudioQuality, &'static str)> = quality
            .all_fallbacks()
            .into_iter()
            .map(|q| (q, q.as_api_format()))
            .collect();

        for (q, fmt) in qualities_to_try {
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
                debug!(quality = fmt, "Got stream URL");
                return Ok((url.to_string(), q));
            }

            warn!(
                quality = fmt,
                track_id = %track.track_id,
                response = %body,
                "get_stream_url: no streaming URL in response"
            );
        }

        Err(DeezerError::QualityUnavailable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Проверка, что `DeezerClient::new()` создаёт валидный клиент.
    #[test]
    fn test_new_client() {
        let client = DeezerClient::new().expect("new client");
        assert!(!client.is_authenticated());
    }

    /// Проверка структуры `SearchResults` после парсинга.
    #[test]
    fn test_search_results_default() {
        let results: SearchResults = serde_json::from_str("{}").unwrap();
        assert!(results.data.is_empty());
    }

    /// Парсинг реалистичного `TRACK`-массива ответа `deezer.pageSearch` в
    /// `Vec<TrackData>`: числовой `SNG_ID` → `String`, `TRACK_TOKEN`, `NB_LISTEN`.
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

        let first = &data[0];
        assert_eq!(first.track_id, "51300437");
        assert_eq!(first.title, "Bohemian Rhapsody");
        assert_eq!(first.artist, "Queen");
        assert_eq!(first.album, "A Night at the Opera");
        assert_eq!(first.duration, "05:55");
        assert_eq!(first.track_token.as_deref(), Some("tok-bohemian"));
        assert_eq!(first.md5_origin, "0123456789abcdef0123456789abcdef");
        assert_eq!(first.nb_listen, 1_234_567);
        assert!(!first.explicit_lyrics);
        assert!(first.has_track_token());

        // Второй трек без `TRACK_TOKEN` и без `FALLBACK`: оба опциональны.
        let second = &data[1];
        assert_eq!(second.track_id, "51300438");
        assert!(second.explicit_lyrics);
        assert!(second.track_token.is_none());
        assert!(!second.has_track_token());
    }

    /// Парсинг реалистичного ответа `deezer.getUserData` в `UserData`:
    /// числовой `USER_ID` → `String`, `OPTIONS`, оффер.
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
        assert_eq!(user.user.options.license_token, "lic-tok");
        assert!(user.user.options.web_streaming);
        assert!(user.user.options.web_hq);
        assert!(!user.user.options.web_lossless);
        assert_eq!(user.api_token, "cf-token");
        assert_eq!(user.offer.as_ref().unwrap().offer_name, "premium");
    }

    /// Парсинг одной строки синхронных субтитров `LYRICS_SYNC_JSON` (`TimeMS`
    /// — миллисекунды, `line` — текст).
    #[test]
    fn test_parse_lyric_line() {
        let line: LyricLine = serde_json::from_value(serde_json::json!({
            "TimeMS": 1_234,
            "line": "Is this the real life?"
        }))
        .unwrap();
        assert_eq!(line.time_ms, 1_234);
        assert_eq!(line.line, "Is this the real life?");

        // Массив строк, как приходит в `LYRICS_SYNC_JSON`.
        let lines: Vec<LyricLine> = serde_json::from_value(serde_json::json!([
            { "TimeMS": 0, "line": "Is this the real life?" },
            { "TimeMS": 3_840, "line": "Is this just fantasy?" }
        ]))
        .unwrap();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[1].time_ms, 3_840);
        assert_eq!(lines[1].line, "Is this just fantasy?");
    }
}
