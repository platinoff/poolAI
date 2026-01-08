//! Integration tests for Telegram Bot Module

use poolai::tgbot;

#[tokio::test]
async fn test_start_bot() {
    // Test that start_bot can be called without panicking
    tgbot::start_bot("test-token-123").await;
}

#[tokio::test]
async fn test_start_bot_with_empty_token() {
    // Test that start_bot handles empty token
    tgbot::start_bot("").await;
}

#[tokio::test]
async fn test_send_notification() {
    // Test that send_notification can be called without panicking
    tgbot::send_notification("123456789", "Test message").await;
}

#[tokio::test]
async fn test_send_notification_with_empty_chat_id() {
    // Test that send_notification handles empty chat_id
    tgbot::send_notification("", "Test message").await;
}

#[tokio::test]
async fn test_send_notification_with_empty_message() {
    // Test that send_notification handles empty message
    tgbot::send_notification("123456789", "").await;
}

#[tokio::test]
async fn test_send_notification_long_message() {
    // Test that send_notification handles long messages
    let long_message = "A".repeat(4096);
    tgbot::send_notification("123456789", &long_message).await;
}
