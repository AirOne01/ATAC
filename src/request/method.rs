use ratatui::prelude::Color;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Default, Debug, Copy, Clone, EnumString, Display, Serialize, Deserialize)]
pub enum Method {
    #[default]
    #[strum(to_string = "GET")]
    GET,
    #[strum(to_string = "POST")]
    POST,
    #[strum(to_string = "PUT")]
    PUT,
    #[strum(to_string = "DELETE")]
    DELETE,
    #[strum(to_string = "PATCH")]
    PATCH,
    #[strum(to_string = "OPTIONS")]
    OPTIONS
}

impl Method {
    pub fn get_color(&self) -> Color {
        match self {
            Method::GET => Color::Green,
            Method::POST => Color::Rgb(231, 186, 0),
            Method::PUT => Color::LightBlue,
            Method::DELETE => Color::LightRed,
            Method::PATCH => Color::LightCyan,
            Method::OPTIONS => Color::Magenta,
        }
    }

    pub fn to_reqwest(&self) -> reqwest::Method {
        match self {
            Method::GET => reqwest::Method::GET,
            Method::POST => reqwest::Method::POST,
            Method::PUT => reqwest::Method::PUT,
            Method::DELETE => reqwest::Method::DELETE,
            Method::PATCH => reqwest::Method::PATCH,
            Method::OPTIONS => reqwest::Method::OPTIONS
        }
    }
}

pub fn next_method(method: &Method) -> Method {
    match method {
        &Method::GET => Method::POST,
        &Method::POST => Method::PUT,
        &Method::PUT => Method::DELETE,
        &Method::DELETE => Method::PATCH,
        &Method::PATCH => Method::OPTIONS,
        &Method::OPTIONS => Method::GET
    }
}