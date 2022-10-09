//! The Mint that represents the native token

/// There are 10^9 lamports in one SAFE
pub const DECIMALS: u8 = 9;

// The Mint for native SAFE Token accounts
safecoin_program::declare_id!("JUFMnWZ6ksBramrswzjQNraYAgHXbQPeC57URvxdcf8");

/// Seed for the native_mint's program-derived address
pub const PROGRAM_ADDRESS_SEEDS: &[&[u8]] = &["native-mint".as_bytes(), &[255]];

#[cfg(test)]
mod tests {
    use {
        super::*,
        safecoin_program::{native_token::*, pubkey::Pubkey},
    };

    #[test]
    fn test_decimals() {
        assert!(
            (lamports_to_sol(42) - crate::amount_to_ui_amount(42, DECIMALS)).abs() < f64::EPSILON
        );
        assert_eq!(
            sol_to_lamports(42.),
            crate::ui_amount_to_amount(42., DECIMALS)
        );
    }

    #[test]
    fn expected_native_mint_id() {
        let native_mint_id =
            Pubkey::create_program_address(PROGRAM_ADDRESS_SEEDS, &crate::id()).unwrap();
        assert_eq!(id(), native_mint_id);
    }
}