use super::alloc::vec::Vec;
use super::*;
const KEY_FEE_SPLIT_MODEL: &[u8] = b"01";
const KEY_BALANCE: &[u8] = b"02";
pub const KEY_MP: &[u8] = b"03";
const KEY_RESOURCE_ID: &[u8] = b"04";

pub fn generate_fee_split_model_key(account: &Address) -> Vec<u8> {
    [KEY_FEE_SPLIT_MODEL, account.as_ref()].concat()
}
pub fn generate_balance_key(order_id: &[u8]) -> Vec<u8> {
    [KEY_BALANCE, order_id].concat()
}

pub fn generate_resource_id_key(addr: &Address) -> Vec<u8> {
    [KEY_RESOURCE_ID, addr.as_ref()].concat()
}
