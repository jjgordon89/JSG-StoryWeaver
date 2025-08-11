use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum ImageResolution {
    #[default]
    R1024x1024,
    R1792x1024,
    R1024x1792,
}

impl FromStr for ImageResolution {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1024x1024" => Ok(ImageResolution::R1024x1024),
            "1792x1024" => Ok(ImageResolution::R1792x1024),
            "1024x1792" => Ok(ImageResolution::R1024x1792),
            _ => Err(format!("Invalid image resolution: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum BrainstormCategory {
    #[default]
    Characters,
    Plot,
    Setting,
    Dialogue,
    Scenes,
}

impl FromStr for BrainstormCategory {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Characters" => Ok(BrainstormCategory::Characters),
            "Plot" => Ok(BrainstormCategory::Plot),
            "Setting" => Ok(BrainstormCategory::Setting),
            "Dialogue" => Ok(BrainstormCategory::Dialogue),
            "Scenes" => Ok(BrainstormCategory::Scenes),
            _ => Err(format!("Invalid brainstorm category: {}", s)),
        }
    }
}
