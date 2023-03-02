mod pool_actions;
mod utils;

#[test]
#[ignore]
fn test_pool_atlas_usdc_v1() {
    pool_actions::run_test(
        "ORC.ATLAS-USDC-V1",
        vec![
            utils::Swap {
                protocol: "ORC",
                from_token: "SAFE",
                to_token: "USDC",
                amount: 0.222,
            },
            utils::Swap {
                protocol: "ORC",
                from_token: "USDC",
                to_token: "ATLAS",
                amount: -0.5,
            },
        ],
        vec![
            utils::Swap {
                protocol: "ORC",
                from_token: "ATLAS",
                to_token: "USDC",
                amount: 0.0,
            },
            utils::Swap {
                protocol: "ORC",
                from_token: "USDC",
                to_token: "SAFE",
                amount: 0.0,
            },
        ],
        false,
    );
}

#[test]
#[ignore]
fn test_pool_ray_sol_latest() {
    pool_actions::run_test(
        "ORC.RAY-SAFE",
        vec![utils::Swap {
            protocol: "ORC",
            from_token: "SAFE",
            to_token: "RAY",
            amount: 0.111,
        }],
        vec![utils::Swap {
            protocol: "ORC",
            from_token: "RAY",
            to_token: "SAFE",
            amount: 0.0,
        }],
        false,
    );
}

#[test]
#[ignore]
fn test_pool_sol_usdc_latest() {
    pool_actions::run_test(
        "ORC.SAFE-USDC",
        vec![utils::Swap {
            protocol: "ORC",
            from_token: "SAFE",
            to_token: "USDC",
            amount: 0.111,
        }],
        vec![utils::Swap {
            protocol: "ORC",
            from_token: "USDC",
            to_token: "SAFE",
            amount: 0.0,
        }],
        false,
    );
}

#[test]
#[ignore]
fn test_pool_msol_sol_latest() {
    pool_actions::run_test(
        "ORC.MSAFE-SAFE",
        vec![
            utils::Swap {
                protocol: "ORC",
                from_token: "SAFE",
                to_token: "USDC",
                amount: 0.119,
            },
            utils::Swap {
                protocol: "ORC",
                from_token: "USDC",
                to_token: "MSAFE",
                amount: -0.5,
            },
        ],
        vec![
            utils::Swap {
                protocol: "ORC",
                from_token: "MSAFE",
                to_token: "USDC",
                amount: 0.0,
            },
            utils::Swap {
                protocol: "ORC",
                from_token: "USDC",
                to_token: "SAFE",
                amount: 0.0,
            },
        ],
        false,
    );
}
