extern crate clap;
extern crate reqwest;

use clap::{App, Arg};
use etherscanner::structs::{Api, Query};

fn main() {
    let matches = App::new("Searcher")
        .about("Finds transactions on Etherscan for multiple given parameters
                In order to ultilise this app fully unnix commands are necessary
                To save a given search use the > symbol after your search terms. 
                i.e. <ADDRESS> <BLOCKNUMBER> <BLOCKNUMBER> <CONTRACT ADDRESS> [MINIMUM USD VALUE] > filenamehere.json
                this will save your search in a file name output.json in the present working directory..")
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
                .empty_values(true)
                // .default_value("0")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("End Block")
                .help("Upper-bound block")
                .value_name("BLOCKNUMBER")
                .index(3)
                .required(true)
                .empty_values(true)
                // .default_value("99999999")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("Token")
                .help("Specific ERC-20 Token address")
                .index(4)
                .required(true)
                .empty_values(true)
                .value_name("CONTRACT ADDRESS")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("Value Threshhold")
                .help("Minimum value threshhold of transactions")
                .index(5)
                .required(true)
                .value_name("MINIMUM USD VALUE")
                .empty_values(true)
                // .default_value("0")
                .takes_value(true),
        )
        .get_matches();

    let address_from = matches.value_of("Sender").unwrap();
    let token_address = matches.value_of("Token").unwrap();
    let start_block = matches.value_of("Start Block").unwrap();
    let end_block = matches.value_of("End Block").unwrap();
    let value_threshhold = matches.value_of("Value Threshhold").unwrap();

    let query: Query = Query {
        address_from: address_from.to_string(),
        token_address: token_address.to_string(),
        start_block: start_block.to_string(),
        end_block: end_block.to_string(),
        value_threshhold: value_threshhold.to_string(),
    };

    println!(
        " 
        ███████╗ ██████╗ █████╗ ███╗   ██╗███╗   ██╗███████╗██████╗ 
        ██╔════╝██╔════╝██╔══██╗████╗  ██║████╗  ██║██╔════╝██╔══██╗
        ███████╗██║     ███████║██╔██╗ ██║██╔██╗ ██║█████╗  ██████╔╝
        ╚════██║██║     ██╔══██║██║╚██╗██║██║╚██╗██║██╔══╝  ██╔══██╗
        ███████║╚██████╗██║  ██║██║ ╚████║██║ ╚████║███████╗██║  ██║
        ╚══════╝ ╚═════╝╚═╝  ╚═╝╚═╝  ╚═══╝╚═╝  ╚═══╝╚══════╝╚═╝  ╚═╝
                                                                    
          

            Scanning for the following parameters:\n
                Sender: {}
                Token Address: {}
                Start Block: {}
                End Block: {}\n",
        query.address_from, query.token_address, query.start_block, query.end_block
    );

    let response = Api::generate_api(query);
    println!("{:?}", response);
}
