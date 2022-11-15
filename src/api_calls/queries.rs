use crate::api_calls::structs::Query;

pub fn is_search_acc_int(query: &Query) -> bool {
    !query.address_from.is_empty()
        && !query.start_block.is_empty()
        && !query.end_block.is_empty()
        && !query.token_address.is_empty()
        && !query.value_threshhold.is_empty()
}

pub fn is_search_acc_int_flt(query: &Query) -> bool {
    !query.address_from.is_empty()
        && !query.start_block.is_empty()
        && !query.end_block.is_empty()
        && !query.token_address.is_empty()
        && !query.value_threshhold.is_empty()
}

pub fn is_search_acc_rng(query: &Query) -> bool {
    !query.address_from.is_empty()
        && !query.start_block.is_empty()
        && !query.end_block.is_empty()
        && query.token_address.is_empty()
        && query.value_threshhold.is_empty()
}

pub fn is_search_acc_rng_flt(query: &Query) -> bool {
    !query.address_from.is_empty()
        && !query.start_block.is_empty()
        && !query.end_block.is_empty()
        && query.token_address.is_empty()
        && !query.value_threshhold.is_empty()
}

pub fn is_search_acc_tkn_rng(query: &Query) -> bool {
    !query.address_from.is_empty()
        && !query.start_block.is_empty()
        && !query.end_block.is_empty()
        && !query.token_address.is_empty()
        && query.value_threshhold.is_empty()
}

pub fn is_search_acc_tkn_rng_flt(query: &Query) -> bool {
    !query.address_from.is_empty()
        && !query.start_block.is_empty()
        && !query.end_block.is_empty()
        && !query.token_address.is_empty()
        && !query.value_threshhold.is_empty()
}

pub fn is_search_rng(query: &Query) -> bool {
    query.address_from.is_empty()
        && !query.start_block.is_empty()
        && !query.end_block.is_empty()
        && query.token_address.is_empty()
        && query.value_threshhold.is_empty()
}

pub fn is_search_rng_flt(query: &Query) -> bool {
    query.address_from.is_empty()
        && !query.start_block.is_empty()
        && !query.end_block.is_empty()
        && query.token_address.is_empty()
        && !query.value_threshhold.is_empty()
}
