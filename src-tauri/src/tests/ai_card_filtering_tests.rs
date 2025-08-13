//! Tests for AI card filtering functionality

#[cfg(test)]
mod tests {
    use crate::database::operations::ai_card_ops::AICardOps;
use crate::models::ai_card::{AICardFilter, CreateAICardRequest};
use crate::database::{init_test_db, get_pool};
use chrono::{Utc, Duration};

    #[tokio::test]
    async fn test_ai_card_date_range_filtering() {
        init_test_db().await.expect("Failed to init test db");
        let pool = get_pool().expect("Failed to get pool");
        let project_id = "test-project-1";
        
        // Create test cards with different dates
        let now = Utc::now();
        let yesterday = now - Duration::days(1);
        let week_ago = now - Duration::days(7);
        
        // Create cards with different timestamps
        let card1 = CreateAICardRequest {
            project_id: project_id.to_string(),
            document_id: None,
            feature_type: "write".to_string(),
            prompt_context: "Test prompt 1".to_string(),
            response_text: "Test response 1".to_string(),
            model_used: Some("gpt-4".to_string()),
            token_count: Some(100),
            cost_estimate: Some(0.01),
            tags: None,
        };
        
        let card2 = CreateAICardRequest {
            project_id: project_id.to_string(),
            document_id: None,
            feature_type: "rewrite".to_string(),
            prompt_context: "Test prompt 2".to_string(),
            response_text: "Test response 2".to_string(),
            model_used: Some("claude-3".to_string()),
            token_count: Some(150),
            cost_estimate: Some(0.02),
            tags: None,
        };
        
        // Create the cards
        let created_card1 = AICardOps::create(&pool, card1).await.unwrap();
        let created_card2 = AICardOps::create(&pool, card2).await.unwrap();
        
        // Test date range filtering
        let start_date = yesterday.to_rfc3339();
        let end_date = now.to_rfc3339();
        
        let filtered_cards = AICardOps::get_by_date_range(&pool, project_id, &start_date, &end_date)
            .await
            .unwrap();
        
        // Should return both cards as they were created recently
        assert!(filtered_cards.len() >= 2);
        
        // Test filtering with a narrow date range that should exclude cards
        let narrow_start = week_ago.to_rfc3339();
        let narrow_end = (week_ago + Duration::hours(1)).to_rfc3339();
        
        let narrow_filtered = AICardOps::get_by_date_range(&pool, project_id, &narrow_start, &narrow_end)
            .await
            .unwrap();
        
        // Should return no cards as they were created after this range
        assert_eq!(narrow_filtered.len(), 0);
    }

    #[tokio::test]
    async fn test_ai_card_provider_filtering() {
        init_test_db().await.expect("Failed to init test db");
        let pool = get_pool().expect("Failed to get pool");
        let project_id = "test-project-2";
        
        // Create cards with different providers
        let gpt_card = CreateAICardRequest {
            project_id: project_id.to_string(),
            document_id: None,
            feature_type: "write".to_string(),
            prompt_context: "GPT prompt".to_string(),
            response_text: "GPT response".to_string(),
            model_used: Some("gpt-4".to_string()),
            token_count: Some(100),
            cost_estimate: Some(0.01),
            tags: None,
        };
        
        let claude_card = CreateAICardRequest {
            project_id: project_id.to_string(),
            document_id: None,
            feature_type: "rewrite".to_string(),
            prompt_context: "Claude prompt".to_string(),
            response_text: "Claude response".to_string(),
            model_used: Some("claude-3-sonnet".to_string()),
            token_count: Some(150),
            cost_estimate: Some(0.02),
            tags: None,
        };
        
        // Create the cards
        AICardOps::create(&pool, gpt_card).await.unwrap();
        AICardOps::create(&pool, claude_card).await.unwrap();
        
        // Test provider filtering
        let gpt_cards = AICardOps::get_by_provider(&pool, project_id, "gpt")
            .await
            .unwrap();
        
        assert_eq!(gpt_cards.len(), 1);
        assert!(gpt_cards[0].model_used.as_ref().unwrap().starts_with("gpt"));
        
        let claude_cards = AICardOps::get_by_provider(&pool, project_id, "claude")
            .await
            .unwrap();
        
        assert_eq!(claude_cards.len(), 1);
        assert!(claude_cards[0].model_used.as_ref().unwrap().starts_with("claude"));
    }

    #[tokio::test]
    async fn test_ai_card_model_filtering() {
        init_test_db().await.expect("Failed to init test db");
        let pool = get_pool().expect("Failed to get pool");
        let project_id = "test-project-3";
        
        // Create cards with specific models
        let gpt4_card = CreateAICardRequest {
            project_id: project_id.to_string(),
            document_id: None,
            feature_type: "write".to_string(),
            prompt_context: "GPT-4 prompt".to_string(),
            response_text: "GPT-4 response".to_string(),
            model_used: Some("gpt-4".to_string()),
            token_count: Some(100),
            cost_estimate: Some(0.01),
            tags: None,
        };
        
        let gpt35_card = CreateAICardRequest {
            project_id: project_id.to_string(),
            document_id: None,
            feature_type: "rewrite".to_string(),
            prompt_context: "GPT-3.5 prompt".to_string(),
            response_text: "GPT-3.5 response".to_string(),
            model_used: Some("gpt-3.5-turbo".to_string()),
            token_count: Some(150),
            cost_estimate: Some(0.005),
            tags: None,
        };
        
        // Create the cards
        AICardOps::create(&pool, gpt4_card).await.unwrap();
        AICardOps::create(&pool, gpt35_card).await.unwrap();
        
        // Test exact model filtering
        let gpt4_cards = AICardOps::get_by_model(&pool, project_id, "gpt-4")
            .await
            .unwrap();
        
        assert_eq!(gpt4_cards.len(), 1);
        assert_eq!(gpt4_cards[0].model_used.as_ref().unwrap(), "gpt-4");
        
        let gpt35_cards = AICardOps::get_by_model(&pool, project_id, "gpt-3.5-turbo")
            .await
            .unwrap();
        
        assert_eq!(gpt35_cards.len(), 1);
        assert_eq!(gpt35_cards[0].model_used.as_ref().unwrap(), "gpt-3.5-turbo");
    }

    #[tokio::test]
    async fn test_ai_card_cost_range_filtering() {
        init_test_db().await.expect("Failed to init test db");
        let pool = get_pool().expect("Failed to get pool");
        let project_id = "test-project-4";
        
        // Create cards with different costs
        let cheap_card = CreateAICardRequest {
            project_id: project_id.to_string(),
            document_id: None,
            feature_type: "write".to_string(),
            prompt_context: "Cheap prompt".to_string(),
            response_text: "Cheap response".to_string(),
            model_used: Some("gpt-3.5-turbo".to_string()),
            token_count: Some(50),
            cost_estimate: Some(0.001),
            tags: None,
        };
        
        let expensive_card = CreateAICardRequest {
            project_id: project_id.to_string(),
            document_id: None,
            feature_type: "rewrite".to_string(),
            prompt_context: "Expensive prompt".to_string(),
            response_text: "Expensive response".to_string(),
            model_used: Some("gpt-4".to_string()),
            token_count: Some(500),
            cost_estimate: Some(0.05),
            tags: None,
        };
        
        // Create the cards
        AICardOps::create(&pool, cheap_card).await.unwrap();
        AICardOps::create(&pool, expensive_card).await.unwrap();
        
        // Test cost range filtering
        let low_cost_cards = AICardOps::get_by_cost_range(&pool, project_id, 0.0, 0.01)
            .await
            .unwrap();
        
        assert_eq!(low_cost_cards.len(), 1);
        assert!(low_cost_cards[0].cost_estimate.unwrap() <= 0.01);
        
        let high_cost_cards = AICardOps::get_by_cost_range(&pool, project_id, 0.02, 1.0)
            .await
            .unwrap();
        
        assert_eq!(high_cost_cards.len(), 1);
        assert!(high_cost_cards[0].cost_estimate.unwrap() >= 0.02);
        
        // Test range that includes both
        let all_cost_cards = AICardOps::get_by_cost_range(&pool, project_id, 0.0, 1.0)
            .await
            .unwrap();
        
        assert_eq!(all_cost_cards.len(), 2);
    }

    #[tokio::test]
    async fn test_ai_card_combined_filtering() {
        init_test_db().await.expect("Failed to init test db");
        let pool = get_pool().expect("Failed to get pool");
        let project_id = "test-project-5";
        
        // Create a card that matches specific criteria
        let target_card = CreateAICardRequest {
            project_id: project_id.to_string(),
            document_id: None,
            feature_type: "write".to_string(),
            prompt_context: "Target prompt".to_string(),
            response_text: "Target response".to_string(),
            model_used: Some("gpt-4".to_string()),
            token_count: Some(200),
            cost_estimate: Some(0.02),
            tags: Some("important".to_string()),
        };
        
        // Create a card that doesn't match
        let other_card = CreateAICardRequest {
            project_id: project_id.to_string(),
            document_id: None,
            feature_type: "rewrite".to_string(),
            prompt_context: "Other prompt".to_string(),
            response_text: "Other response".to_string(),
            model_used: Some("claude-3".to_string()),
            token_count: Some(100),
            cost_estimate: Some(0.01),
            tags: None,
        };
        
        // Create the cards
        AICardOps::create(&pool, target_card).await.unwrap();
        AICardOps::create(&pool, other_card).await.unwrap();
        
        // Test combined filtering using the filter struct directly
        let filter = AICardFilter {
            project_id: Some(project_id.to_string()),
            document_id: None,
            feature_type: Some("write".to_string()),
            is_stacked: None,
            is_starred: None,
            date_start: None,
            date_end: None,
            provider: Some("gpt".to_string()),
            model_used: None,
            cost_min: Some(0.015),
            cost_max: Some(0.025),
            limit: None,
            offset: None,
        };
        
        let filtered_cards = crate::models::ai_card::AIResponseCard::get_filtered(&pool, filter)
            .await
            .unwrap();
        
        assert_eq!(filtered_cards.len(), 1);
        assert_eq!(filtered_cards[0].feature_type, "write");
        assert!(filtered_cards[0].model_used.as_ref().unwrap().starts_with("gpt"));
        assert!(filtered_cards[0].cost_estimate.unwrap() >= 0.015);
        assert!(filtered_cards[0].cost_estimate.unwrap() <= 0.025);
    }
}
