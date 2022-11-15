extern crate reqwest;

pub use crate::api_calls::searching;
use crate::api_calls::structs::{
    Query, ResponseAddressInternal, ResponseAddressRange, ResponseAddressTokenRange,
    ResponseBlockRange, RootAddressInternal, RootAddressRange, RootAddressTokenRange,
    RootBlockRange,
};
use reqwest::blocking;
use std::error::Error;

// Search by Account & Token in Block Range with Value Threshhold
pub fn search_acc_tkn_rng_flt(query: &Query) -> Vec<ResponseAddressTokenRange> {
    let api = format!(
            "https://api.etherscan.io/api?module=account&action=tokentx&contractaddress={}&address={}&page=1&offset=10000&startblock={}&endblock={}&sort=asc&apikey=8CSUXGCYX5P4JTIGP84VAW2H89APQYA3E8",
            query.token_address, query.address_from, query.start_block, query.end_block).to_string();
    println!("Making api call:{:?}", api);
    let gen_api_call = blocking::get(api).unwrap().text().unwrap();
    let gen_api_call_to_json =
        serde_json::from_str::<RootAddressTokenRange>(&gen_api_call).unwrap();
    let mut filtered_query: Vec<ResponseAddressTokenRange> = vec![];
    let threshhold: u128 = 1000000 * query.value_threshhold.parse::<u128>().unwrap();
    let n_elements = gen_api_call_to_json.result.len();
    for i in 1..n_elements {
        if gen_api_call_to_json.result[i]
            .value
            .parse::<u128>()
            .unwrap()
            >= threshhold
        {
            filtered_query.push(gen_api_call_to_json.result[i].clone());
        }
    }
    return filtered_query;
}

// Search by Account & Token in Block Range (No value threshhold)
pub fn search_acc_tkn_rng(query: &Query) -> Result<RootAddressTokenRange, Box<dyn Error>> {
    let api = format!(
            "https://api.etherscan.io/api?module=account&action=tokentx&contractaddress={}&address={}&page=1&offset=10000&startblock={}&endblock={}&sort=asc&apikey=8CSUXGCYX5P4JTIGP84VAW2H89APQYA3E8",
            query.token_address, query.address_from, query.start_block, query.end_block).to_string();
    println!("Making api call:{:?}", api);
    let gen_api_call = blocking::get(api)?.text().unwrap();
    let gen_api_call_to_json =
        serde_json::from_str::<RootAddressTokenRange>(&gen_api_call).unwrap();
    Ok(gen_api_call_to_json)
}

// Search by Account in Block Range with Value Threshhold
pub fn search_acc_rng_flt(query: &Query) -> Result<Vec<ResponseAddressRange>, Box<dyn Error>> {
    let api = format!(
            "https://api.etherscan.io/api?module=account&action=txlist&address={}&startblock={}&endblock={}&page=1&offset=10000t&sort=asc&apikey=8CSUXGCYX5P4JTIGP84VAW2H89APQYA3E8",
            query.address_from, query.start_block, query.end_block).to_string();
    println!("Making api call:{:#?}", api);
    let gen_api_call = blocking::get(api)?.text().unwrap();
    let gen_api_call_to_json = serde_json::from_str::<RootAddressRange>(&gen_api_call).unwrap();
    println!("Results:{:#?}", gen_api_call_to_json);
    let mut filtered_query: Vec<ResponseAddressRange> = vec![];
    let threshhold: u128 = 1000000 * query.value_threshhold.parse::<u128>().unwrap();
    let n_elements = gen_api_call_to_json.result.len();
    for i in 1..n_elements {
        if gen_api_call_to_json.result[i]
            .value
            .parse::<u128>()
            .unwrap()
            >= threshhold
        {
            filtered_query.push(gen_api_call_to_json.result[i].clone());
        }
    }
    Ok(filtered_query)
}

// Search by Account in Block Range (No value threshhold)
pub fn search_acc_rng(query: &Query) -> Result<RootAddressRange, Box<dyn Error>> {
    let api = format!(
            "https://api.etherscan.io/api?module=account&action=txlist&address={}&startblock={}&endblock={}&page=1&offset=10000&sort=asc&apikey=8CSUXGCYX5P4JTIGP84VAW2H89APQYA3E8",
            query.address_from, query.start_block, query.end_block).to_string();
    println!("Making api call:{:#?}", api);
    let gen_api_call = blocking::get(api)?.text().unwrap();
    println!("Results:{:#?}", gen_api_call);

    let gen_api_call_to_json = serde_json::from_str::<RootAddressRange>(&gen_api_call).unwrap();
    Ok(gen_api_call_to_json)
}

// Search Block Range with Value Threshhold
pub fn search_rng_flt(query: &Query) -> Result<Vec<ResponseBlockRange>, Box<dyn Error>> {
    let api = format!("https://api.etherscan.io/api?module=account&action=txlistinternal&startblock={}&endblock={}&page=1&offset=10000&sort=asc&apikey=8CSUXGCYX5P4JTIGP84VAW2H89APQYA3E8", 
        query.start_block, query.end_block).to_string();
    println!("Making api call:{:?}", api);
    let gen_api_call = blocking::get(api)?.text().unwrap();
    let gen_api_call_to_json = serde_json::from_str::<RootBlockRange>(&gen_api_call).unwrap();
    let mut filtered_query: Vec<ResponseBlockRange> = vec![];
    let threshhold: u128 = 1000000 * query.value_threshhold.parse::<u128>().unwrap();
    let n_elements = gen_api_call_to_json.result.len();
    for i in 1..n_elements {
        if gen_api_call_to_json.result[i]
            .value
            .parse::<u128>()
            .unwrap()
            >= threshhold
        {
            filtered_query.push(gen_api_call_to_json.result[i].clone());
        }
    }
    Ok(filtered_query)
}

pub fn search_rng(query: &Query) -> Result<RootBlockRange, Box<dyn Error>> {
    // Search by Block Range (no value threshhold)
    let api = format!("https://api.etherscan.io/api?module=account&action=txlistinternal&startblock={}&endblock={}&page=1&offset=10000&sort=asc&apikey=8CSUXGCYX5P4JTIGP84VAW2H89APQYA3E8", 
            query.start_block, query.end_block).to_string();
    println!("Making api call:{:?}", api);
    let gen_api_call = blocking::get(api)?.text().unwrap();
    let gen_api_call_to_json = serde_json::from_str::<RootBlockRange>(&gen_api_call).unwrap();
    Ok(gen_api_call_to_json)
}
// Search by Account & Token in Block Range with Value Threshhold
pub fn search_acc_int_flt(query: &Query) -> Result<Vec<ResponseAddressInternal>, Box<dyn Error>> {
    let api = format!(
            "https://api.etherscan.io/api?module=account&action=tokentx&contractaddress={}&address={}&page=1&offset=10000&startblock={}&endblock={}&sort=asc&apikey=8CSUXGCYX5P4JTIGP84VAW2H89APQYA3E8",
            query.token_address, query.address_from, query.start_block, query.end_block).to_string();
    println!("Making api call:{:?}", api);
    let gen_api_call = blocking::get(api)?.text().unwrap();
    let gen_api_call_to_json = serde_json::from_str::<RootAddressInternal>(&gen_api_call).unwrap();
    let mut filtered_query: Vec<ResponseAddressInternal> = vec![];
    let threshhold: u128 = 1000000 * query.value_threshhold.parse::<u128>().unwrap();
    let n_elements = gen_api_call_to_json.result.len();
    for i in 1..n_elements {
        if gen_api_call_to_json.result[i]
            .value
            .parse::<u128>()
            .unwrap()
            >= threshhold
        {
            filtered_query.push(gen_api_call_to_json.result[i].clone());
        }
    }
    Ok(filtered_query)
}

// Search by Account & Token in Block Range (No value threshhold)
pub fn search_acc_int(query: &Query) -> Result<RootAddressInternal, Box<dyn Error>> {
    let api = format!(
            "https://api.etherscan.io/api?module=account&action=tokentx&contractaddress={}&address={}&page=1&offset=10000&startblock={}&endblock={}&sort=asc&apikey=8CSUXGCYX5P4JTIGP84VAW2H89APQYA3E8",
            query.token_address, query.address_from, query.start_block, query.end_block).to_string();
    println!("Making api call:{:?}", api);
    let gen_api_call = blocking::get(api)?.text().unwrap();
    let gen_api_call_to_json = serde_json::from_str::<RootAddressInternal>(&gen_api_call).unwrap();
    Ok(gen_api_call_to_json)
}
