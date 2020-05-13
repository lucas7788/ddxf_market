#![cfg_attr(not(feature = "mock"), no_std)]
#![feature(proc_macro_hygiene)]
extern crate alloc;
extern crate ontio_std as ostd;
use alloc::collections::btree_map::BTreeMap;
use ostd::abi::{Decoder, Encoder, Error, Sink, Source};
use ostd::database;
use ostd::prelude::H256;
use ostd::prelude::*;
use ostd::runtime::{address, check_witness, input, ret};
use ostd::types::{Address, U128};
mod utils;
use ostd::contract::{neo, ong, ont};
use utils::*;
mod basic;
use basic::*;

const ADMIN: Address = ostd::macros::base58!("AbtTQJYKfQxq4UdygDsbLVjE8uRrJ2H3tP");
const DECIMAL: U128 = 1000000000;
const MAX_PERCENTAGE: u16 = 10000;

fn set_mp(mp_account: &Address) -> bool {
    database::put(utils::KEY_MP, mp_account);
    true
}

fn set_fee_split_model(seller_acc: &Address, fee_split_model: FeeSplitModel) -> bool {
    assert!(fee_split_model.percentage <= MAX_PERCENTAGE);
    let mp = get_mp_account();
    assert!(check_witness(seller_acc) && check_witness(&mp));
    let mp = database::get::<_, Address>(KEY_MP).unwrap();
    assert!(check_witness(&mp) && check_witness(&seller_acc));
    database::put(
        utils::generate_fee_split_model_key(seller_acc),
        fee_split_model,
    );
    true
}

fn get_fee_split_model(seller_acc: &Address) -> FeeSplitModel {
    database::get::<_, FeeSplitModel>(utils::generate_fee_split_model_key(seller_acc))
        .unwrap_or(FeeSplitModel { percentage: 0 })
}

fn transfer_amount(buyer_acc: &Address, seller_acc: &Address, fee: Fee, n: U128) -> bool {
    assert!(check_witness(buyer_acc));
    let amt = n.checked_mul(fee.count as U128).unwrap();
    let self_addr = address();
    transfer(
        buyer_acc,
        &self_addr,
        amt,
        &fee.contract_type,
        Some(fee.contract_addr.clone()),
    );
    let mut balance = balance_of(seller_acc, &fee.contract_type);
    balance.balance += amt;
    balance.contract_address = Some(fee.contract_addr);
    database::put(
        utils::generate_balance_key(seller_acc, &fee.contract_type),
        balance,
    );
    true
}

fn balance_of(account: &Address, token_type: &TokenType) -> TokenBalance {
    database::get::<_, TokenBalance>(utils::generate_balance_key(account, token_type))
        .unwrap_or(TokenBalance::new(token_type.clone()))
}

fn settle(seller_acc: &Address) -> bool {
    assert!(check_witness(seller_acc));
    let self_addr = address();
    let mp = get_mp_account();
    let tokens = vec![TokenType::ONG, TokenType::ONT, TokenType::OEP4];
    for token in tokens.iter() {
        let balance = balance_of(seller_acc, &TokenType::ONG);
        if balance.balance != 0 {
            assert!(settle_inner(seller_acc, &self_addr, &mp, balance));
        }
    }
    true
}

fn settle_inner(
    seller_acc: &Address,
    self_addr: &Address,
    mp: &Address,
    balance: TokenBalance,
) -> bool {
    let fee_split = get_fee_split_model(seller_acc);
    let fee = balance
        .balance
        .checked_mul(fee_split.percentage as U128)
        .unwrap();
    assert!(transfer(
        &self_addr,
        &mp,
        fee,
        &balance.token_type,
        balance.contract_address
    ));
    true
}

fn transfer(
    from: &Address,
    to: &Address,
    amt: U128,
    contract_type: &TokenType,
    contract_addr: Option<Address>,
) -> bool {
    match contract_type {
        TokenType::ONG => {
            assert!(ong::transfer(from, to, amt));
        }
        TokenType::ONT => {
            assert!(ont::transfer(from, to, amt));
        }
        TokenType::OEP4 => {
            //TODO
            let contract_address = contract_addr.unwrap();
            let res = neo::call_contract(&contract_address, ("transfer", (from, to, amt))).unwrap();
        }
    }
    true
}

fn get_mp_account() -> Address {
    database::get::<_, Address>(utils::KEY_MP).unwrap()
}

#[no_mangle]
pub fn invoke() {
    let input = input();
    let mut source = Source::new(&input);
    let action: &[u8] = source.read().unwrap();
    let mut sink = Sink::new(12);
    match action {
        b"setFeeSplitModel" => {
            let (seller_acc, fee_split_model) = source.read().unwrap();
            sink.write(set_fee_split_model(seller_acc, fee_split_model));
        }
        b"get_fee_split_model" => {
            let seller_acc = source.read().unwrap();
            sink.write(get_fee_split_model(seller_acc));
        }
        b"transferAmount" => {
            let (buyer_acc, seller_acc, fee, n) = source.read().unwrap();
            sink.write(transfer_amount(buyer_acc, seller_acc, fee, n));
        }
        b"balance_of" => {
            let (account, token_type) = source.read().unwrap();
            sink.write(balance_of(account, &token_type));
        }
        b"settle" => {
            let seller_acc = source.read().unwrap();
            sink.write(settle(seller_acc));
        }
        b"get_mp_account" => {
            sink.write(get_mp_account());
        }
        _ => {
            let method = str::from_utf8(action).ok().unwrap();
            panic!("not support method:{}", method)
        }
    }
    ret(sink.bytes());
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}