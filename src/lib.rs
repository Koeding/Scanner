extern crate reqwest;

use reqwest::blocking;
use serde::{Deserialize, Serialize};
use std::error::Error;

// const THRESHOLD: String = "0".to_string();
pub struct Query {
    pub address_from: String,
    pub token_address: String,
    pub start_block: String,
    pub end_block: String,
    pub value_threshhold: String,
}

#[derive(Debug)]
pub struct Api {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
        println!("Making api call:{:?}", url);
        let api_call = blocking::get(url)?.text().unwrap();
        let api_call_to_json = serde_json::from_str::<Root>(&api_call).unwrap();
        Ok(api_call_to_json)
    }
}

// Filtered by value threshhold
impl Api {
    pub fn get_filtered_txs(
        query: &Query,
        responses: &Root,
    ) -> Result<Vec<Response>, &'static str> {
        let mut filtered_query: Vec<Response> = vec![];
        let conversion: u64 = 1000000;
        let threshhold_value: u64 = conversion * query.value_threshhold.parse::<u64>().unwrap();
        let n_elements = responses.result.len();
        for i in 1..n_elements {
            if responses.result[i].value.parse::<u64>().unwrap() >= threshhold_value {
                filtered_query.push(responses.result[i].clone());
            }
        }

        println!("filtered {:#?} @ {:#?}", filtered_query, threshhold_value);
        Ok(filtered_query)
    }
}
//write function to determine search
impl Api {
    pub fn generate_api(query: Query) -> Result<String, Box<dyn Error>> {
        let api = String::new();
        if !query.address_from.is_empty()
            && !query.start_block.is_empty()
            && !query.end_block.is_empty()
            && !query.token_address.is_empty()
        {
            // Search by account in Block Range
            let api = format!(
                    "https://api.etherscan.io/api?module=account&action=tokentx&contractaddress={}&address={}&page=1&offset=10000&startblock={}&endblock={}&sort=asc&apikey=8CSUXGCYX5P4JTIGP84VAW2H89APQYA3E8",
                    query.token_address, query.address_from, query.start_block, query.end_block);
        } else if !query.address_from.is_empty()
            && !query.start_block.is_empty()
            && !query.end_block.is_empty()
            && query.token_address.is_empty()
        {
            // Search by account in Block Range
            let api = format!(
                "https://api.etherscan.io/apt?module=account&action=txlist&address={}&startblock={}&endblock={}&page=t&offset=1000t&sort=ast&apikey=YourApiKeyToken",
                query.address_from, query.start_block, query.end_block);
        } else if query.address_from.is_empty() {
            if !query.start_block.is_empty() && !query.end_block.is_empty() {
                // Search by Block Range
                let api = format!("https://api.etherscan.io/api?module=account&action=txlistinternal&startblock={}&endblock={}&page=1&offset=10&sort=asc&apikey=YourApiKeyToken", query.start_block, query.end_block);
            }
        }
        Ok(api)
    }
}
