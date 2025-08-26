use serde::{Deserialize, Serialize};
use ts_rs::TS;
use crate::commands::ai_cards;

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export)]
pub struct GuidedSuggestion {
    pub title: String,
    pub description: String,
}

#[tauri::command]
pub async fn get_guided_suggestions(
    prompt: String,
) -> Result<Vec<GuidedSuggestion>, String> {
    let card_results = ai_cards::generate_ai_cards(
        prompt,
        "guided_suggestions".to_string(),
        None,
        None,
        None,
        None,
    )
    .await
    .map_err(|e| e.to_string())?;

    let suggestions = card_results
        .into_iter()
        .map(|card| GuidedSuggestion {
            title: card.title,
            description: card.description,
        })
        .collect();

    Ok(suggestions)
}