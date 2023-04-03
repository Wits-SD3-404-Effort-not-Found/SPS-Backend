use super::account::Account;
use hex;

#[tokio::test]
async fn test_generate_session_token_expected_input_correct_token() {
    let mut test_account = Account::default();
    test_account.email = "testEmailAddress@gmail.com".to_string();
    test_account.hashed_password = "test_password+1".to_string();

    let token_result = super::session_token::generate_session_token(&test_account).await;
    let hex_result = hex::decode(&token_result.token); // if this fails then its an invalid hex string
    // and concequently an invalid sha256
    
    assert!(!hex_result.is_err());
    let hex_str = hex_result.unwrap();
    assert_eq!(hex_str.len(), 32);
}
