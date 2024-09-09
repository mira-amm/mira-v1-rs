use fuels::prelude::*;

pub const AMM_CONTRACT_BINARY_PATH: &str =
    "sway-abis/mira_amm_contract/release/mira_amm_contract.bin";
pub const ADD_LIQUIDITY_SCRIPT_BINARY_PATH: &str =
    "sway-abis/add_liquidity_script/release/add_liquidity_script.bin";
pub const CREATE_POOL_AND_ADD_LIQUIDITY_SCRIPT_BINARY_PATH: &str =
    "sway-abis/create_pool_and_add_liquidity_script/release/create_pool_and_add_liquidity_script.bin";
pub const REMOVE_LIQUIDITY_SCRIPT_BINARY_PATH: &str =
    "sway-abis/remove_liquidity_script/release/remove_liquidity_script.bin";
pub const SWAP_EXACT_INPUT_SCRIPT_BINARY_PATH: &str =
    "sway-abis/swap_exact_input_script/release/swap_exact_input_script.bin";
pub const SWAP_EXACT_OUTPUT_SCRIPT_BINARY_PATH: &str =
    "sway-abis/swap_exact_output_script/release/swap_exact_output_script.bin";

abigen!(
    Script(
        name = "AddLiquidityScript",
        abi = "sway-abis/add_liquidity_script/release/add_liquidity_script-abi.json"
    ),
    Script(
        name = "CreatePoolAndAddLiquidityScript",
        abi = "sway-abis/create_pool_and_add_liquidity_script/release/create_pool_and_add_liquidity_script-abi.json"
    ),
    Script(
        name = "RemoveLiquidityScript",
        abi = "sway-abis/remove_liquidity_script/release/remove_liquidity_script-abi.json"
    ),
    Script(
        name = "SwapExactInputScript",
        abi = "sway-abis/swap_exact_input_script/release/swap_exact_input_script-abi.json"
    ),
    Script(
        name = "SwapExactOutputScript",
        abi = "sway-abis/swap_exact_output_script/release/swap_exact_output_script-abi.json"
    ),
    Contract(
        name = "MiraAmmContract",
        abi = "sway-abis/mira_amm_contract/release/mira_amm_contract-abi.json"
    ),
);

pub type PoolId = (AssetId, AssetId, bool);

pub struct AmmFees {
    pub lp_fee_volatile: u64,
    pub lp_fee_stable: u64,
    pub protocol_fee_volatile: u64,
    pub protocol_fee_stable: u64,
}

pub struct LpAssetInfo {
    pub asset_id: AssetId,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: u64,
}
