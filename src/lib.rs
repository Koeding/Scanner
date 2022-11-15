extern crate reqwest;

mod api_calls;

use crate::api_calls::queries::{
    is_search_acc_int, is_search_acc_int_flt, is_search_acc_rng, is_search_acc_rng_flt,
    is_search_acc_tkn_rng, is_search_acc_tkn_rng_flt, is_search_rng, is_search_rng_flt,
};
use crate::api_calls::structs::{Api, Query};
pub use crate::api_calls::{searching, structs};
use crate::searching::{
    search_acc_int, search_acc_int_flt, search_acc_rng, search_acc_rng_flt, search_acc_tkn_rng,
    search_acc_tkn_rng_flt, search_rng, search_rng_flt,
};

use std::error::Error;

impl Api {
    pub fn generate_api(query: Query) -> Result<(), Box<dyn Error>> {
        if is_search_acc_tkn_rng_flt(&query) {
            // Search by Account & Token in Block Range with Value Threshhold
            let filtered_payload = search_acc_tkn_rng_flt(&query);
            println!("Results:{:#?}", filtered_payload);
        } else if is_search_acc_tkn_rng(&query) {
            // Search by Account & Token in Block Range (No value threshhold)
            let payload = search_acc_tkn_rng(&query);
            println!("Results:{:?}", payload);
        } else if is_search_acc_rng_flt(&query) {
            // Search by Account in Block Range with Value Threshhold
            let filtered_payload = search_acc_rng_flt(&query);
            println!("Results:{:#?}", filtered_payload);
        } else if is_search_acc_rng(&query) {
            // Search by Account in Block Range (No value threshhold)
            let payload = search_acc_rng(&query);
            println!("Results:{:?}", payload);
        } else if is_search_acc_int_flt(&query) {
            // Search by Account in Block Range with Value Threshhold
            let filtered_payload = search_acc_int_flt(&query);
            println!("Results:{:#?}", filtered_payload);
        } else if is_search_acc_int(&query) {
            // Search by Account in Block Range (No value threshhold)
            let payload = search_acc_int(&query);
            println!("Results:{:?}", payload);
        } else if is_search_rng_flt(&query) {
            // Search Block Range with Value Threshhold
            let filtered_payload = search_rng_flt(&query);
            println!("Results:{:#?}", filtered_payload);
        } else if is_search_rng(&query) {
            // Search by Block Range (no value threshhold)
            let payload = search_rng(&query);
            println!("Results:{:#?}", payload);
        }

        Ok(())
    }
}
