use dexchain::Precompiles;
use dexchain_runtime as dexchain;
use dexchain_runtime::{
    constants::{currency::*, time::*},
    AccountId, ChainId, EvmAddress, GenesisConfig, Signature, WASM_BINARY,
};
use hex_literal::hex;
use jsonrpc_core::serde_json::Map;
use pallet_evm::GenesisAccount;
use sc_service::{config::TelemetryEndpoints, ChainType, Properties};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{
    crypto::{Ss58Codec, UncheckedInto},
    sr25519, Pair, Public, H160, U256,
};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    AccountId32,
};
use std::{collections::BTreeMap, str::FromStr};

// The URL for the telemetry server.
const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const DEFAULT_PROTOCOL_ID: &str = "dexio";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
    (get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

pub fn development_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "Development",
        // ID
        "dev",
        ChainType::Development,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![authority_keys_from_seed("Alice")],
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Pre-funded accounts
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                ],
                vec![H160::from_str("1DfeF571E6E4418a3E170889DE0e4E784FeA1E4a").unwrap()],
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                ],
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Dave"),
                    get_account_id_from_seed::<sr25519::Public>("Eve"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                ],
                ChainId::Testnet.u64(),
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        Some(properties()),
        // Extensions
        None,
    ))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "Local Testnet",
        // ID
        "local_testnet",
        ChainType::Local,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial PoA authorities
                vec![
                    authority_keys_from_seed("Alice"),
                    authority_keys_from_seed("Bob"),
                ],
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Pre-funded accounts
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                    get_account_id_from_seed::<sr25519::Public>("Dave"),
                    get_account_id_from_seed::<sr25519::Public>("Eve"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
                ],
                vec![H160::from_str("1DfeF571E6E4418a3E170889DE0e4E784FeA1E4a").unwrap()],
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                ],
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Dave"),
                    get_account_id_from_seed::<sr25519::Public>("Eve"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                ],
                ChainId::Testnet.u64(),
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        // Properties
        Some(properties()),
        // Extensions
        None,
    ))
}

pub fn dexchain_staging_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or("dexchain development wasm not available")?;
    let boot_nodes = vec![];

    Ok(ChainSpec::from_genesis(
        "Dexchain Staging Testnet",
        "dexchain_staging_testnet",
        ChainType::Live,
        move || dexchain_staging_config_genesis(wasm_binary),
        boot_nodes,
        Some(
            TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
                .expect("Dexchain Staging telemetry url is valid; qed"),
        ),
        Some(DEFAULT_PROTOCOL_ID),
        Some(properties()),
        Default::default(),
    ))
}

fn dexchain_staging_config_genesis(wasm_binary: &[u8]) -> GenesisConfig {
    testnet_genesis(
        wasm_binary,
        vec![
            (
                hex!["062e8d3f21c7cc417c6eca7ebdc271f1958eedd4dc635eb9bafc9a795e7ce702"]
                    .unchecked_into(),
                hex!["96f9f64bf2e1fee469b4fc730ddb1344e890cdb731e3e3c6f90a42a78cb82fed"]
                    .unchecked_into(),
            ),
            (
                hex!["8ebbc1a5394866ea9d17a69de6812001da0b7be2b2c1c354518774ba2e36aa25"]
                    .unchecked_into(),
                hex!["65cdd2f9d41d3d1f2ed805ec4b65a33bf922d944527991349ec4242cf6f34303"]
                    .unchecked_into(),
            ),
            (
                hex!["8e1cf7aa17f84ca4aae26989bf32ff6005af30579d76d4459e4edf210c9f3a3f"]
                    .unchecked_into(),
                hex!["bb003d1c37026cf92e30805bfecaf7bd2d88e23aba351d60923d145e3c430ea9"]
                    .unchecked_into(),
            ),
        ],
        // 5E9bvHKRqsBygRTgZ5Yr4cy1f14ntDoV7Q4PGaHBxGBr1qM9
        hex!["5c34b4b2762e0e3680a8336ef822d4e3ac6d46fff209d4dd3a8220c7f3697848"].into(),
        vec![
            // 5E9bvHKRqsBygRTgZ5Yr4cy1f14ntDoV7Q4PGaHBxGBr1qM9
            hex!["5c34b4b2762e0e3680a8336ef822d4e3ac6d46fff209d4dd3a8220c7f3697848"].into(),
        ],
        vec![H160::from_str("373944b86Bc07887f2cdcC5EF5E055ee33AC2d3C").unwrap()],
        vec![],
        vec![],
        ChainId::Testnet.u64(),
        true,
    )
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AuraId, GrandpaId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    evm_accounts: Vec<EvmAddress>,
    council: Vec<AccountId>,
    technical_committee: Vec<AccountId>,
    chain_id: u64,
    _enable_println: bool,
) -> GenesisConfig {
    // This is supposed the be the simplest bytecode to revert without returning any data.
    // We will pre-deploy it under all of our precompiles to ensure they can be called from
    // within contracts. TODO We should have a test to ensure this is the right bytecode.
    // (PUSH1 0x00 PUSH1 0x00 REVERT)
    let revert_bytecode = vec![0x60, 0x00, 0x60, 0x00, 0xFD];

    GenesisConfig {
        frame_system: dexchain::SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
            changes_trie_config: Default::default(),
        },
        pallet_balances: dexchain::BalancesConfig {
            // Configure endowed accounts with initial balance of 1 << 60.
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 200_000_000 * DOLLARS))
                .collect(),
        },
        pallet_aura: dexchain::AuraConfig {
            authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
        },
        pallet_grandpa: dexchain::GrandpaConfig {
            authorities: initial_authorities
                .iter()
                .map(|x| (x.1.clone(), 1))
                .collect(),
        },
        pallet_sudo: dexchain::SudoConfig {
            // Assign network admin rights.
            key: root_key,
        },
        pallet_ethereum_chain_id: dexchain::EthereumChainIdConfig { chain_id },
        pallet_evm: dexchain::EVMConfig {
            accounts: compose_evm_address_to_accounts(evm_accounts, Precompiles::used_addresses()),
            // accounts: BTreeMap::new(),
        },
        pallet_ethereum: Default::default(),
        pallet_collective_Instance1: dexchain::CouncilConfig {
            members: council,
            phantom: Default::default(),
        },
        pallet_collective_Instance2: Default::default(),
        pallet_membership_Instance1: dexchain::TechnicalMembershipConfig {
            members: technical_committee,
            phantom: Default::default(),
        },
        pallet_elections_phragmen: Default::default(),
        pallet_treasury: Default::default(),
        pallet_democracy: Default::default(),
        pallet_contracts: dexchain::ContractsConfig {
            current_schedule: pallet_contracts::Schedule {
                enable_println: _enable_println,
                ..Default::default()
            },
        },
    }
}

fn properties() -> Properties {
    let mut properties = Map::new();
    properties.insert("tokenSymbol".into(), "DEX".into());
    properties.insert("tokenDecimals".into(), 18.into());
    return properties;
}

fn compose_evm_address_to_accounts(
    evm_accounts: Vec<EvmAddress>,
    precompiles_accounts: impl Iterator<Item = H160>,
) -> BTreeMap<EvmAddress, GenesisAccount> {
    // This is supposed the be the simplest bytecode to revert without returning any data.
    // We will pre-deploy it under all of our precompiles to ensure they can be called from
    // within contracts. TODO We should have a test to ensure this is the right bytecode.
    // (PUSH1 0x00 PUSH1 0x00 REVERT)
    let revert_bytecode = vec![0x60, 0x00, 0x60, 0x00, 0xFD];

    let accounts = evm_accounts
        .iter()
        .cloned()
        .map(|k| {
            let account = GenesisAccount {
                nonce: U256::from(0),
                balance: U256::from(200_000_000 * DOLLARS),
                storage: Default::default(),
                code: vec![],
            };
            (k, account)
        })
        .collect::<BTreeMap<EvmAddress, GenesisAccount>>()
        .into_iter()
        .chain(
            precompiles_accounts
                .map(|addr| {
                    (
                        addr,
                        GenesisAccount {
                            nonce: Default::default(),
                            balance: Default::default(),
                            storage: Default::default(),
                            code: revert_bytecode.clone(),
                        },
                    )
                })
                .collect::<BTreeMap<EvmAddress, GenesisAccount>>(),
        )
        .collect();

    accounts
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[cfg(feature = "aura")]
    fn test_compose_evm_address_to_accounts() {

        let evm_accounts =
            vec![H160::from_str("1DfeF571E6E4418a3E170889DE0e4E784FeA1E4a").unwrap()];
        let precompiles_accounts = Precompiles::used_addresses();
        let accounts = compose_evm_address_to_accounts(evm_accounts, precompiles_accounts);
        println!("accounts: {}", accounts.len());
        for (address, account) in &accounts {
            println!("address: {}", hex::encode(address));
        }
    }
}
