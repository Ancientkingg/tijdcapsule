
use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;

#[derive(Clone)]
pub struct AppState {
    pub cookie_key: Key,
}

impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.cookie_key.clone()
    }
}