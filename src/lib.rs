extern crate reqwest;

use reqwest::blocking;
use serde::{Deserialize, Serialize};
use std::error::Error;

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
pub struct RootAddressTokenRange {
    status: String,
    message: String,
    pub result: Vec<ResponseAddressTokenRange>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResponseAddressTokenRange {
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
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RootAddressRange {
    pub status: String,
    pub message: String,
    pub result: Vec<ResponseAddressRange>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseAddressRange {
    pub block_number: String,
    pub time_stamp: String,
    pub hash: String,
    pub nonce: String,
    pub block_hash: String,
    pub transaction_index: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub gas: String,
    pub gas_price: String,
    pub is_error: String,
    #[serde(rename = "txreceipt_status")]
    pub txreceipt_status: String,
    pub input: String,
    pub contract_address: String,
    pub cumulative_gas_used: String,
    pub gas_used: String,
    pub confirmations: String,
    pub method_id: String,
    pub function_name: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RootBlockRange {
    pub status: String,
    pub message: String,
    pub result: Vec<ResponseBlockRange>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseBlockRange {
    pub block_number: String,
    pub time_stamp: String,
    pub hash: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub contract_address: String,
    pub input: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub gas: String,
    pub gas_used: String,
    pub trace_id: String,
    pub is_error: String,
    pub err_code: String,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RootAddressInternal {
    pub status: String,
    pub message: String,
    pub result: Vec<ResponseAddressInternal>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseAddressInternal {
    pub block_number: String,
    pub time_stamp: String,
    pub hash: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub contract_address: String,
    pub input: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub gas: String,
    pub gas_used: String,
    pub trace_id: String,
    pub is_error: String,
    pub err_code: String,
}
//TODO: use u128 to stop overflow of large value searches
//write function to determine search
impl Api {
    pub fn generate_api(query: Query) -> Result<(), Box<dyn Error>> {
        if !query.address_from.is_empty()
            && !query.start_block.is_empty()
            && !query.end_block.is_empty()
            && !query.token_address.is_empty()
            && !query.value_threshhold.is_empty()
        {
            // Search by Account & Token in Block Range with Value Threshhold
            let api = format!(
                    "https://api.etherscan.io/api?module=account&action=tokentx&contractaddress={}&address={}&page=1&offset=10000&startblock={}&endblock={}&sort=asc&apikey=8CSUXGCYX5P4JTIGP84VAW2H89APQYA3E8",
                    query.token_address, query.address_from, query.start_block, query.end_block).to_string();
            println!("Making api call:{:?}", api);
            let gen_api_call = blocking::get(api)?.text().unwrap();
            let gen_api_call_to_json =
                serde_json::from_str::<RootAddressTokenRange>(&gen_api_call).unwrap();
            let mut filtered_query: Vec<ResponseAddressTokenRange> = vec![];
            let threshhold: u64 = 1000000 * query.value_threshhold.parse::<u64>().unwrap();
            let n_elements = gen_api_call_to_json.result.len();
            for i in 1..n_elements {
                if gen_api_call_to_json.result[i].value.parse::<u64>().unwrap() >= threshhold {
                    filtered_query.push(gen_api_call_to_json.result[i].clone());
                }
            }
            println!("Results:{:#?}", filtered_query);
        } else if !query.address_from.is_empty()
            && !query.start_block.is_empty()
            && !query.end_block.is_empty()
            && !query.token_address.is_empty()
        {
            // Search by Account & Token in Block Range (No value threshhold)
            let api = format!(
                    "https://api.etherscan.io/api?module=account&action=tokentx&contractaddress={}&address={}&page=1&offset=10000&startblock={}&endblock={}&sort=asc&apikey=8CSUXGCYX5P4JTIGP84VAW2H89APQYA3E8",
                    query.token_address, query.address_from, query.start_block, query.end_block).to_string();
            println!("Making api call:{:?}", api);
            let gen_api_call = blocking::get(api)?.text().unwrap();
            let gen_api_call_to_json =
                serde_json::from_str::<RootAddressTokenRange>(&gen_api_call).unwrap();
            println!("Results:{:?}", gen_api_call_to_json);
        } else if !query.address_from.is_empty()
            && !query.start_block.is_empty()
            && !query.end_block.is_empty()
            && query.token_address.is_empty()
        {
            if !query.value_threshhold.is_empty() {
                // Search by Account in Block Range with Value Threshhold
                let api = format!(
                "https://api.etherscan.io/apt?module=account&action=txlist&address={}&startblock={}&endblock={}&page=t&offset=10000t&sort=ast&apikey=8CSUXGCYX5P4JTIGP84VAW2H89APQYA3E8",
                query.address_from, query.start_block, query.end_block).to_string();
                println!("Making api call:{:#?}", api);
                let gen_api_call = blocking::get(api)?.text().unwrap();
                let gen_api_call_to_json =
                    serde_json::from_str::<RootAddressRange>(&gen_api_call).unwrap();
                println!("Results:{:#?}", gen_api_call_to_json);
                let mut filtered_query: Vec<ResponseAddressRange> = vec![];
                let threshhold: u64 = 1000000 * query.value_threshhold.parse::<u64>().unwrap();
                let n_elements = gen_api_call_to_json.result.len();
                for i in 1..n_elements {
                    if gen_api_call_to_json.result[i].value.parse::<u64>().unwrap() >= threshhold {
                        filtered_query.push(gen_api_call_to_json.result[i].clone());
                    }
                }
                println!("Results:{:#?}", filtered_query);
            } else {
                // Search by Account in Block Range (No value threshhold)
                let api = format!(
                    "https://api.etherscan.io/apt?module=account&action=txlist&address={}&startblock={}&endblock={}&page=t&offset=10000t&sort=ast&apikey=8CSUXGCYX5P4JTIGP84VAW2H89APQYA3E8",
                    query.address_from, query.start_block, query.end_block).to_string();
                println!("Making api call:{:#?}", api);
                let gen_api_call = blocking::get(api)?.text().unwrap();
                let gen_api_call_to_json =
                    serde_json::from_str::<RootAddressRange>(&gen_api_call).unwrap();
                println!("Results:{:#?}", gen_api_call_to_json);
            }
        } else if query.address_from.is_empty() {
            if !query.start_block.is_empty() && !query.end_block.is_empty() {
                if !query.value_threshhold.is_empty() {
                    // Search Block Range with Value Threshhold
                    let api = format!("https://api.etherscan.io/api?module=account&action=txlistinternal&startblock={}&endblock={}&page=1&offset=10000&sort=asc&apikey=8CSUXGCYX5P4JTIGP84VAW2H89APQYA3E8", 
                    query.start_block, query.end_block).to_string();
                    println!("Making api call:{:?}", api);
                    let gen_api_call = blocking::get(api)?.text().unwrap();
                    let gen_api_call_to_json =
                        serde_json::from_str::<RootBlockRange>(&gen_api_call).unwrap();
                    let mut filtered_query: Vec<ResponseBlockRange> = vec![];
                    let threshhold: u64 = 1000000 * query.value_threshhold.parse::<u64>().unwrap();
                    let n_elements = gen_api_call_to_json.result.len();
                    for i in 1..n_elements {
                        if gen_api_call_to_json.result[i].value.parse::<u64>().unwrap()
                            >= threshhold
                        {
                            filtered_query.push(gen_api_call_to_json.result[i].clone());
                        }
                    }
                    println!("Results:{:#?}", filtered_query);
                } else {
                    // Search by Block Range (no value threshhold)
                    let api = format!("https://api.etherscan.io/api?module=account&action=txlistinternal&startblock={}&endblock={}&page=1&offset=10000&sort=asc&apikey=8CSUXGCYX5P4JTIGP84VAW2H89APQYA3E8", 
                    query.start_block, query.end_block).to_string();
                    println!("Making api call:{:?}", api);
                    let gen_api_call = blocking::get(api)?.text().unwrap();
                    let gen_api_call_to_json =
                        serde_json::from_str::<RootBlockRange>(&gen_api_call).unwrap();
                    println!("Results:{:#?}", gen_api_call_to_json);
                }
            }
        }

        Ok(())
    }
}
