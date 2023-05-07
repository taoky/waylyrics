use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub mpris_sync_interval: String,
    pub lyric_update_interval: String,
    pub cache_lyrics: bool,
    pub allow_click_through_me: bool,
    pub full_width_lyric_bg: bool,
    pub hide_label_on_empty_text: bool,
    pub origin_lyric_in_above: bool,
    pub theme: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mpris_sync_interval: "2s".to_owned(),
            lyric_update_interval: "20ms".to_owned(),
            allow_click_through_me: true,
            full_width_lyric_bg: false,
            hide_label_on_empty_text: true,
            theme: "default".into(),
            origin_lyric_in_above: true,
            cache_lyrics: true,
        }
    }
}
