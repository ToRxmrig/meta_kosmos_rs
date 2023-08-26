use ethers::prelude::*;
use eyre::Result;
use std::collections::BTreeMap;

use meta_dex::pool::Pool;

#[derive(Debug, Clone)]
/// Holds all info needed to for sandwich simulations
pub struct RawIngredients {
    /// the token that we start and end the sandwich with
    pub startend_token: Address,
    /// the token that bot holds only for duration of sandwich
    pub intermediary_token: Address,
    pub from: Address,
    pub sandwich_contract: Address,
    pub meats: Vec<Transaction>,
    pub target_pool: Pool,
    /// holds the state diffs produced from meats
    pub state_diffs: BTreeMap<H160, AccountDiff>,
}

impl RawIngredients {
    // Create a new `RawIngredients` instance
    pub async fn new(
        target_pair: &Pool,
        victim_txs: Vec<Transaction>,
        start_token: Address, // which token to start trade from (input_token)
        state_diffs: BTreeMap<H160, AccountDiff>,
        sandwich_contract_address: Address,
        searcher_address: Address,
    ) -> Result<RawIngredients> {
        let (mut input_token, mut output_token) = (target_pair.token_0, target_pair.token_1);

        // swap if input_token is equal to token_1
        if start_token == target_pair.token_1 {
            (input_token, output_token) = (output_token, input_token);
        }

        Ok(RawIngredients {
            from: searcher_address,
            meats: victim_txs,
            target_pool: *target_pair,
            sandwich_contract: sandwich_contract_address,
            startend_token: input_token,
            intermediary_token: output_token,
            state_diffs,
        })
    }
}
