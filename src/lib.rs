extern crate reqwest;

use reqwest::blocking;
use serde::{Deserialize, Serialize};
use std::error::Error;
pub struct Query {
    pub address_from: String,
    pub token_address: String,
    pub start_block: String,
    pub end_block: String,
    pub value_threshhold: u64,
}
#[derive(Debug)]
pub struct Api {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Root {
    status: String,
    message: String,
    pub result: Vec<Response>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub block_number: String,
    pub time_stamp: String,
    pub hash: String,
    pub nonce: String,
    pub block_hash: String,
    pub from: String,
    pub contract_address: String,
    pub to: String,
    pub value: String,
    pub token_name: String,
    pub token_symbol: String,
    pub token_decimal: String,
    pub transaction_index: String,
    pub gas: String,
    pub gas_price: String,
    pub gas_used: String,
    pub cumulative_gas_used: String,
    pub input: String,
    pub confirmations: String,
}

// Non filtered impl
impl Api {
    pub fn get_txs(query: &Query) -> Result<Root, Box<dyn Error>> {
        let url = format!(
            "https://api.etherscan.io/api?module=account&action=tokentx&contractaddress={}&address={}&page=1&offset=1000&startblock={}&endblock={}&sort=asc&apikey=8CSUXGCYX5P4JTIGP84VAW2H89APQYA3E8",
            query.token_address, query.address_from, query.start_block, query.end_block
        )
        .to_string();
        let api_call = blocking::get(url)?.text().unwrap();
        let api_call_to_json = serde_json::from_str::<Root>(&api_call).unwrap();
        Ok(api_call_to_json)
    }
}

// Filtered by value threshhold
impl Api {
    pub fn get_filtered_txs(query: Query, responses: &Root) -> Result<Vec<Response>, &'static str> {
        let mut filtered_query: Vec<Response> = vec![];
        let n_elements = responses.result.len();
        for i in 1..n_elements {
            if &responses.result[i].value.parse::<u64>().unwrap() >= &query.value_threshhold {
                filtered_query.push(responses.result[i].clone());
            } else {
                return Err("whoops");
            }
        }

        println!("{:#?}", filtered_query);
        Ok(filtered_query)
    }
}
