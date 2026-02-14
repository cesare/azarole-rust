use std::{collections::HashMap, sync::OnceLock};

use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::cookie::{Cookie, CookieJar, Key};
use azarole::{
    config::{AppConfig, ApplicationConfig, DatabaseConfig, FrontendConfig, ServerConfig},
    context::{AppState, DatabaseContext},
    repositories::RdbRepositories,
    secrets::{ApikeyConfig, Base64Encoded, GoogleAuthConfig, Secrets, SessionConfig},
};
use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use sqlx::SqlitePool;

fn create_config() -> ApplicationConfig {
    let app = AppConfig {
        base_url: "http://localhost:3000".to_string(),
    };
    let database = DatabaseConfig {
        url: "file:sqlite.db".to_string(),
    };
    let frontend = FrontendConfig {
        base_url: "http://localhost:3001".to_string(),
    };
    let server = ServerConfig {
        bind: "127.0.0.1".to_string(),
        port: 3000,
    };
    ApplicationConfig {
        app,
        database,
        frontend,
        server,
    }
}

fn create_secrets() -> Secrets {
    let api_key = ApikeyConfig {
        digesting_secret_key: Base64Encoded::new(
            // = [0, 1, ..., 31]
            URL_SAFE
                .decode("AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8=")
                .unwrap(),
        ),
    };
    let google_auth = GoogleAuthConfig {
        client_id: "dummy-google-client-id".to_string(),
        client_secret: "dummy-google-client-secret".to_string(),
    };
    let session = SessionConfig {
        session_key: Base64Encoded::new(session_key().master().to_owned()),
    };

    Secrets {
        api_key,
        google_auth,
        session,
    }
}

pub fn create_app_state(pool: SqlitePool) -> AppState {
    let config = create_config();
    let database = DatabaseContext { pool: pool.clone() };
    let repositories = RdbRepositories::new(pool);
    let secrets = create_secrets();

    AppState {
        config,
        database,
        repositories,
        secrets,
    }
}

static SESSION_KEY: OnceLock<Key> = OnceLock::new();
pub fn session_key() -> Key {
    SESSION_KEY.get_or_init(Key::generate).clone()
}

pub fn create_session_middleware() -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::new(CookieSessionStore::default(), session_key())
}

#[allow(dead_code)]
pub fn generate_cookie_value_with_signin_user(user_id: u32) -> String {
    let mut session_state: HashMap<String, String> = HashMap::new();
    session_state.insert("user_id".to_owned(), user_id.to_string());
    let session_state_str = serde_json::to_string(&session_state).unwrap();

    let mut jar = CookieJar::new();
    let cookie = Cookie::new("id", session_state_str);

    jar.private_mut(&session_key()).add(cookie);
    let cookie_header_value = jar.get("id").unwrap().value();

    format!("id={}", cookie_header_value)
}
