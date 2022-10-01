use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, IbcEndpoint};
use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub subspace_id: u64,
    pub cyber_contract_address: Addr,
    pub root_hash: String,
}

pub const STATE: Item<State> = Item::new("state");

#[cw_serde]
pub struct ChannelInfo {
    pub id: String,
    pub counterparty_endpoint: IbcEndpoint,
    pub connection_id: String,
}
