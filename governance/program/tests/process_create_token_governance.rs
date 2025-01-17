#![cfg(feature = "test-sbf")]
mod program_test;

use solana_program_test::*;

use program_test::*;
use solana_sdk::{signature::Keypair, signer::Signer};
use spl_governance::error::GovernanceError;
use spl_governance_tools::error::GovernanceToolsError;

use safe_token::{error::TokenError, instruction::AuthorityType};

#[tokio::test]
async fn test_create_token_governance() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    let realm_cookie = governance_test.with_realm().await;
    let governed_token_cookie = governance_test.with_governed_token().await;

    let token_owner_record_cookie = governance_test
        .with_community_token_deposit(&realm_cookie)
        .await
        .unwrap();

    // Act
    let token_governance_cookie = governance_test
        .with_token_governance(
            &realm_cookie,
            &governed_token_cookie,
            &token_owner_record_cookie,
        )
        .await
        .unwrap();

    // Assert
    let token_governance_account = governance_test
        .get_governance_account(&token_governance_cookie.address)
        .await;

    assert_eq!(token_governance_cookie.account, token_governance_account);

    let token_account = governance_test
        .get_token_account(&governed_token_cookie.address)
        .await;

    assert_eq!(token_governance_cookie.address, token_account.owner);
}

#[tokio::test]
async fn test_create_token_governance_without_transferring_token_owner() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    let realm_cookie = governance_test.with_realm().await;
    let mut governed_token_cookie = governance_test.with_governed_token().await;

    let token_owner_record_cookie = governance_test
        .with_community_token_deposit(&realm_cookie)
        .await
        .unwrap();

    governed_token_cookie.transfer_token_owner = false;

    // Act
    let token_governance_cookie = governance_test
        .with_token_governance(
            &realm_cookie,
            &governed_token_cookie,
            &token_owner_record_cookie,
        )
        .await
        .unwrap();

    // Assert
    let token_governance_account = governance_test
        .get_governance_account(&token_governance_cookie.address)
        .await;

    assert_eq!(token_governance_cookie.account, token_governance_account);

    let token_account = governance_test
        .get_token_account(&governed_token_cookie.address)
        .await;

    assert_eq!(
        governed_token_cookie.token_owner.pubkey(),
        token_account.owner
    );
}

#[tokio::test]
async fn test_create_token_governance_without_transferring_token_owner_with_invalid_token_owner_error(
) {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    let realm_cookie = governance_test.with_realm().await;
    let mut governed_token_cookie = governance_test.with_governed_token().await;

    let token_owner_record_cookie = governance_test
        .with_community_token_deposit(&realm_cookie)
        .await
        .unwrap();

    governed_token_cookie.transfer_token_owner = false;
    governed_token_cookie.token_owner = Keypair::new();

    // Act
    let err = governance_test
        .with_token_governance(
            &realm_cookie,
            &governed_token_cookie,
            &token_owner_record_cookie,
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_eq!(err, GovernanceError::InvalidTokenOwner.into());
}

#[tokio::test]
async fn test_create_token_governance_without_transferring_token_owner_with_owner_not_signed_error()
{
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    let realm_cookie = governance_test.with_realm().await;
    let mut governed_token_cookie = governance_test.with_governed_token().await;

    let token_owner_record_cookie = governance_test
        .with_community_token_deposit(&realm_cookie)
        .await
        .unwrap();

    governed_token_cookie.transfer_token_owner = false;

    // Act
    let err = governance_test
        .with_token_governance_using_instruction(
            &realm_cookie,
            &governed_token_cookie,
            &token_owner_record_cookie,
            |i| {
                i.accounts[3].is_signer = false; // governed_token_owner
            },
            Some(&[&token_owner_record_cookie.token_owner]),
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_eq!(err, GovernanceError::TokenOwnerMustSign.into());
}

#[tokio::test]
async fn test_create_token_governance_with_invalid_token_owner_error() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    let realm_cookie = governance_test.with_realm().await;
    let mut governed_token_cookie = governance_test.with_governed_token().await;

    let token_owner_record_cookie = governance_test
        .with_community_token_deposit(&realm_cookie)
        .await
        .unwrap();

    governed_token_cookie.token_owner = Keypair::new();

    // Act
    let err = governance_test
        .with_token_governance(
            &realm_cookie,
            &governed_token_cookie,
            &token_owner_record_cookie,
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_eq!(err, TokenError::OwnerMismatch.into());
}

#[tokio::test]
async fn test_create_token_governance_with_invalid_realm_error() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    let mut realm_cookie = governance_test.with_realm().await;
    let governed_token_cookie = governance_test.with_governed_token().await;

    let token_owner_record_cookie = governance_test
        .with_community_token_deposit(&realm_cookie)
        .await
        .unwrap();

    let token_governance_cookie = governance_test
        .with_token_governance(
            &realm_cookie,
            &governed_token_cookie,
            &token_owner_record_cookie,
        )
        .await
        .unwrap();

    // try to use Governance account other than Realm as realm
    realm_cookie.address = token_governance_cookie.address;

    // Act
    let err = governance_test
        .with_token_governance(
            &realm_cookie,
            &governed_token_cookie,
            &token_owner_record_cookie,
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_eq!(err, GovernanceToolsError::InvalidAccountType.into());
}

#[tokio::test]
async fn test_create_token_governance_with_close_authority_transfer() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    let realm_cookie = governance_test.with_realm().await;
    let governed_token_cookie = governance_test.with_governed_token().await;

    governance_test
        .bench
        .set_safe_token_account_authority(
            &governed_token_cookie.address,
            &governed_token_cookie.token_owner,
            Some(&governed_token_cookie.token_owner.pubkey()),
            AuthorityType::CloseAccount,
        )
        .await;

    let token_owner_record_cookie = governance_test
        .with_community_token_deposit(&realm_cookie)
        .await
        .unwrap();

    // Act
    let token_governance_cookie = governance_test
        .with_token_governance(
            &realm_cookie,
            &governed_token_cookie,
            &token_owner_record_cookie,
        )
        .await
        .unwrap();

    // Assert
    let token_governance_account = governance_test
        .get_governance_account(&token_governance_cookie.address)
        .await;

    assert_eq!(token_governance_cookie.account, token_governance_account);

    let token_account = governance_test
        .get_token_account(&governed_token_cookie.address)
        .await;

    assert_eq!(token_governance_cookie.address, token_account.owner);
    assert_eq!(
        token_governance_cookie.address,
        token_account.close_authority.unwrap()
    );
}
