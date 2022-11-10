extern crate clap;
extern crate reqwest;

use clap::{App, Arg};
use etherscanner::{Api, Query};

const THRESHOLD: u64 = 0;

fn main() {
    let matches = App::new("Searcher")
        .about("Finds transactions on Etherscan for multiple given parameters")
        .version("1.3.3.7")
        .author("Ben Ruggles")
        .arg(
            Arg::with_name("Sender")
                .help("Address of villain")
                .index(1)
                .required(true)
                .value_name("ADDRESS")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("Start Block")
                .help("Lower-bound block")
                .value_name("BLOCKNUMBER")
                .index(2)
                .required(true)
                .default_value("0")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("End Block")
                .help("Upper-bound block")
                .value_name("BLOCKNUMBER")
                .index(3)
                .required(true)
                .default_value("99999999")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("Token")
                .help("Specific ERC-20 Token address")
                .index(4)
                .required(true)
                .value_name("CONTRACT ADDRESS")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("Value Threshhold")
                .help("Specific ERC-20 Token address")
                .index(5)
                .required(false)
                .value_name("MINIMUM USD VALUE")
                .default_value("0")
                .takes_value(true),
        )
        .get_matches();

    let address_from = matches.value_of("Sender").unwrap();
    let token_address = matches.value_of("Token").unwrap();
    let start_block = matches.value_of("Start Block").unwrap();
    let end_block = matches.value_of("End Block").unwrap();
    let value_threshhold: u64 = matches
        .value_of("End Block")
        .unwrap()
        .parse::<u64>()
        .unwrap();

    let query: Query = Query {
        address_from: address_from.to_string(),
        token_address: token_address.to_string(),
        start_block: start_block.to_string(),
        end_block: end_block.to_string(),
        value_threshhold: value_threshhold,
    };

    println!(
        "   
    Scanning for the following parameters:\n
        Sender: {}
        Token Address: {}
        Start Block: {}
        End Block: {}\n",
        query.address_from, query.token_address, query.start_block, query.end_block
    );
    let responses = Api::get_txs(&query).unwrap();

    // Filtered call
    if query.value_threshhold != THRESHOLD {
        Api::get_filtered_txs(query, &responses).unwrap();
    } else {
        println!("{:#?}", responses);
    }
}
