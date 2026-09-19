use serde::Deserialize;
use colored::Colorize;

#[derive(Debug, Deserialize)]
pub struct Ship {
    #[serde(rename = "ORIGINAL SHIP")]
    pub original: String,

    #[serde(rename = "IDS")]
    pub ids: String,

    #[serde(rename = "FULL NAME")]
    pub replacement_full: String,

    #[serde(rename = "SHORT NAME")]
    pub replacement_short: Option<String>
}

impl Ship {
    pub fn correct_struct(&self) -> Result<(), String> {
        if self.ids.trim().is_empty() {
            return Err(format!("No IDS found for entry '{}', skipping", self.original).yellow().to_string());
        }

        if self.replacement_full.trim().is_empty() {
            return Err(format!("No replacement name found for entry '{}', skipping", self.original).yellow().to_string());
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct Text {
    #[serde(rename = "IDS")]
    pub ids: String,

    #[serde(rename = "REPLACEMENT")]
    pub replacement: String
}

impl Text {
    pub fn correct_struct(&self) -> Result<(), String> {
        if self.ids.trim().is_empty() {
            return Err(format!("No IDS found for, skipping").yellow().to_string());
        }

        if self.replacement.trim().is_empty() {
            return Err(format!("No replacement name found for IDS '{}', skipping", self.ids).yellow().to_string());
        }

        Ok(())
    }
}
