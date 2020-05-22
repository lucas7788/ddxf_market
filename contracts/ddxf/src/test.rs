use super::*;
use hexutil::{read_hex, to_hex};
use ostd::abi::{Decoder, Encoder};
use ostd::mock::build_runtime;
use ostd::mock::contract_mock::Command;
use ostd::prelude::String;

#[test]
fn test() {
    let data = read_hex("fdc8011364746f6b656e53656c6c65725075626c6973682435333963656464372d363363392d346231622d386165332d383764303236653331316532fd3c0100010000000020997a019b98e9847a0c3343bae2d7ad8d931bf784e11ad736539702b661b7f163002dadc839e0e46ade7c400c63704236c1567f09b76a68747470733a2f2f646576332e736167616d61726b65742e696f2f746f6b656e2d686173682f6170692f39393761303139623938653938343761306333333433626165326437616438643933316266373834653131616437333635333937303262363631623766313633010000000020997a019b98e9847a0c3343bae2d7ad8d931bf784e11ad736539702b661b7f1636a68747470733a2f2f646576332e736167616d61726b65742e696f2f746f6b656e2d686173682f6170692f39393761303139623938653938343761306333333433626165326437616438643933316266373834653131616437333635333937303262363631623766313633000000004f000000000000000000000000000000000000000201000000000000000000f2052a0100000000943577010000000020997a019b98e9847a0c3343bae2d7ad8d931bf784e11ad736539702b661b7f163").unwrap_or_default();
    let mut source = Source::new(&data);
    let method: &[u8] = source.read().unwrap();
    let (resource_id, ddo_bytes, item_bytes): (Vec<u8>, &[u8], &[u8]) = source.read().unwrap();
    println!("resource_id:{:?}", String::from_utf8(resource_id));
    let ddo = ResourceDDO::from_bytes(ddo_bytes);
    println!("ddo:{:?}", ddo.manager);
    let item = DTokenItem::from_bytes(item_bytes);
    println!("item:{:?}", item.expired_date);
}

#[test]
fn dtoken_test() {
    let addr = Address::repeat_byte(1);
    let item = DTokenItem {
        fee: Fee {
            contract_addr: addr,
            contract_type: TokenType::ONG,
            count: 1000000,
        },
        expired_date: 10000,
        stocks: 1000,
        templates: vec![TokenTemplate::new(None, vec![1u8; 32])],
    };

    let mut sink = Sink::new(16);
    sink.write(&item);

    let mut source = Source::new(sink.bytes());
    let item2: DTokenItem = source.read().unwrap();
    assert_eq!(item.stocks, item2.stocks);
}

#[test]
fn test2() {
    let data = read_hex("0001000000012a6469643a6f6e743a41626b35725255794a53636e6d5045645264567934693769666955377967433853682096cae35ce8a9b0244178bf28e4966c2ce1b8385723a96a6b838858cdd6ca0a1e00675478ea7368fd9579c00a8a749d29c2b82f2aef10687474703a2f2f64656d6f2e7465737401000000012a6469643a6f6e743a41626b35725255794a53636e6d5045645264567934693769666955377967433853682096cae35ce8a9b0244178bf28e4966c2ce1b8385723a96a6b838858cdd6ca0a1e10687474703a2f2f64656d6f2e7465737400012fee6d8699c9b8f992a6bd54753cf84cb3aae8740000").unwrap_or_default();
    let ddo = ResourceDDO::from_bytes(&data);
    println!("{}", ddo.manager);
}

#[test]
fn test3() {
    let data = read_hex("067265736f5f347f00010000000102313220000000000000000000000000000000000000000000000000000000000000000001fbe02b027e61a6d7602f26cfa9487fa58ef9ee7208656e64706f696e74010000000102313220000000000000000000000000000000000000000000000000000000000000000009656e64706f696e7432000000004f000000000000000000000000000000000000000001640000000000000005cac65e00000000010000000101023132200000000000000000000000000000000000000000000000000000000000000000").unwrap_or_default();
    let mut source = Source::new(&data);
    //    let mthod_name: &str = source.read().unwrap();
    //    println!("method_name:{}", mthod_name);
    let (resource_id, ddo, item): (&[u8], &[u8], &[u8]) = source.read().unwrap();
    //    println!("resource_id:{}", to_hex(resource_id));
    //    let ddo = ResourceDDO::from_bytes(ddo);
    //    println!("manager:{}", ddo.manager);
    //    let item = DTokenItem::from_bytes(item);
    //    println!("item:{}", item.stocks);

    let build = build_runtime();
    let addr = ostd::macros::base58!("Aejfo7ZX5PVpenRj23yChnyH64nf8T1zbu");
    build.witness(&[addr]);
    assert!(dtoken_seller_publish(resource_id, ddo, item));
}

#[test]
fn test5() {
    let data = read_hex("0962757944746f6b656e03067265736f5f3501000000000000000000000000000000151dd3ecff3994999739bee170e6f490437248a7").unwrap_or_default();
    let mut source = Source::new(&data);

    let method_name: &str = source.read().unwrap();
    println!("method_name:{}", method_name);
    let (resource_id, n, buyer): (&[u8], U128, &Address) = source.read().unwrap();

    let build = build_runtime();
    let addr = ostd::macros::base58!("AHhXa11suUgVLX1ZDFErqBd3gskKqLfa5N");
    build.witness(&[buyer.clone()]);
    assert!(buy_dtoken(resource_id, n, buyer));
}

#[test]
fn test4() {
    let data = read_hex("067265736f5f31151dd3ecff3994999739bee170e6f490437248a704067265736f5f31151dd3ecff3994999739bee170e6f490437248a7014abc9c909098687710bd3510e3854045201ee06801000000000000000000000000000000").unwrap_or_default();
    let mut source = Source::new(&data);
    let method_name: &[u8] = source.read().unwrap();
    let (resource_id, account, agents, n) = source.read().unwrap();
    assert!(add_agents(resource_id, account, agents, n));
}

#[test]
fn serialize() {
    let mut bmap: BTreeMap<TokenTemplate, RT> = BTreeMap::new();
    let token_hash = read_hex("96cae35ce8a9b0244178bf28e4966c2ce1b8385723a96a6b838858cdd6ca0a1e")
        .unwrap_or_default();
    let token_template = TokenTemplate::new(
        Some(b"did:ont:Abk5rRUyJScnmPEdRdVy4i7ifiU7ygC8Sh".to_vec()),
        token_hash,
    );
    bmap.insert(token_template.clone(), RT::RTStaticFile);

    let mut token_endpoint = BTreeMap::new();
    token_endpoint.insert(token_template.clone(), "http://demo.test".to_string());
    let manager = ostd::macros::base58!("ARCESVnP8Lbf6S7FuTei3smA35EQYog4LR");

    let mp_contract_address = Address::repeat_byte(3);

    let dtoken_contract_hex =
        read_hex("2fee6d8699c9b8f992a6bd54753cf84cb3aae874").unwrap_or_default();
    let mut temp: [u8; 20] = [0; 20];
    for i in 0..20 {
        temp[i] = dtoken_contract_hex[i]
    }
    let dtoken_contract = Address::new(temp);
    let ddo = ResourceDDO {
        resource_type: RT::RTStaticFile,
        token_resource_type: bmap,
        manager: manager.clone(),
        endpoint: "endpoint".to_string(),
        token_endpoint,
        desc_hash: None,
        dtoken_contract_address: Some(dtoken_contract.clone()),
        mp_contract_address: None,
        split_policy_contract_address: None,
    };

    let mut sink = Sink::new(16);
    sink.write(ddo);
    println!("{}", to_hex(sink.bytes()));
}

#[test]
fn publish() {
    let resource_id = b"resource_id";
    let mut bmap: BTreeMap<TokenTemplate, RT> = BTreeMap::new();
    let temp = vec![0u8; 36];
    let token_template = TokenTemplate::new(None, temp);
    bmap.insert(token_template.clone(), RT::RTStaticFile);

    let manager = Address::repeat_byte(1);
    let dtoken_contract_address = Address::repeat_byte(2);
    let mp_contract_address = Address::repeat_byte(3);

    let ddo = ResourceDDO {
        resource_type: RT::RTStaticFile,
        token_resource_type: bmap,
        manager: manager.clone(),
        endpoint: "endpoint".to_string(),
        token_endpoint: BTreeMap::new(),
        desc_hash: None,
        dtoken_contract_address: Some(dtoken_contract_address.clone()),
        mp_contract_address: None,
        split_policy_contract_address: None,
    };

    let mut sink_temp = Sink::new(64);
    sink_temp.write(&ddo);
    let mut source = Source::new(sink_temp.bytes());
    let ddo_temp: ResourceDDO = source.read().unwrap();
    assert_eq!(&ddo.manager, &ddo_temp.manager);

    let contract_addr = Address::repeat_byte(4);
    let fee = Fee {
        contract_addr,
        contract_type: TokenType::ONG,
        count: 0,
    };
    let mut templates = vec![];
    templates.push(token_template.clone());
    let dtoken_item = DTokenItem {
        fee,
        expired_date: 1,
        stocks: 1,
        templates,
    };

    let handle = build_runtime();
    handle.witness(&[manager.clone(), ADMIN.clone()]);
    assert!(dtoken_seller_publish(
        resource_id,
        &ddo.to_bytes(),
        &dtoken_item.to_bytes()
    ));

    assert!(set_dtoken_contract(&dtoken_contract_address));

    let buyer = Address::repeat_byte(4);

    let mut ong_balance_map = BTreeMap::<Address, U128>::new();
    ong_balance_map.insert(buyer.clone(), 10000);
    ong_balance_map.insert(manager.clone(), 10000);
    let buyer2 = Address::repeat_byte(5);
    ong_balance_map.insert(buyer2.clone(), 10000);

    let call_contract = move |_addr: &Address, _data: &[u8]| -> Option<Vec<u8>> {
        if _addr == &dtoken_contract_address {
            mock_dtoken_contract(_data, &mut ong_balance_map)
        } else {
            mock_mp_contract(_data, &mut ong_balance_map)
        }
    };
    handle.on_contract_call(call_contract);

    handle.witness(&[buyer.clone()]);
    assert!(buy_dtoken(resource_id, 1, &buyer));

    handle.witness(&[buyer.clone(), buyer2.clone()]);
    assert!(buy_dtoken_from_reseller(resource_id, 1, &buyer2, &buyer));
    let token_template_bytes = token_template.to_bytes();

    assert!(use_token(resource_id, &buyer2, &token_template_bytes, 1));
}

fn mock_mp_contract(
    _data: &[u8],
    ong_balance_map: &mut BTreeMap<Address, U128>,
) -> Option<Vec<u8>> {
    let mut sink = Sink::new(12);
    let mut source = Source::new(_data);
    let command = Command::decode(&mut source).ok().unwrap();
    match command {
        Command::Transfer { from, to, value } => {
            let mut from_ba = ong_balance_map.get(from).map(|val| val.clone()).unwrap();
            let mut to_ba = ong_balance_map
                .get(to)
                .map(|val| val.clone())
                .unwrap_or_default();
            from_ba -= value;
            to_ba += value;
            ong_balance_map.insert(from.clone(), from_ba);
            ong_balance_map.insert(to.clone(), to_ba);
            sink.write(true);
        }
        Command::BalanceOf { addr } => {
            let ba = ong_balance_map.get(addr).map(|val| val.clone()).unwrap();
            sink.write(ba);
        }
        _ => {}
    }
    return Some(sink.bytes().to_vec());
}

#[derive(Encoder, Decoder)]
enum DtokenCommand {
    GenerateDToken(GenerateDToken),
    TransferDToken(TransferDToken),
    UseToken(UseToken),
}

#[derive(Encoder, Decoder)]
struct UseToken {
    account: Address,
    resource_id: Vec<u8>,
    token_template: Vec<u8>,
    n: U128,
}

#[derive(Encoder, Decoder)]
struct GenerateDToken {
    account: Address,
    resource_id: Vec<u8>,
    templates: Vec<u8>,
    n: U128,
}

#[derive(Encoder, Decoder)]
struct TransferDToken {
    from_account: Address,
    to_account: Address,
    resource_id: Vec<u8>,
    templates: Vec<u8>,
    n: U128,
}

fn mock_dtoken_contract(
    _data: &[u8],
    ong_balance_map: &mut BTreeMap<Address, U128>,
) -> Option<Vec<u8>> {
    let mut sink = Sink::new(12);
    let mut source = Source::new(_data);
    let command = DtokenCommand::decode(&mut source).ok().unwrap();
    match command {
        DtokenCommand::GenerateDToken(generate) => {
            sink.write(true);
        }
        DtokenCommand::TransferDToken(transfer) => {
            sink.write(true);
        }
        DtokenCommand::UseToken(useToken) => {
            sink.write(true);
        }
    }
    return Some(sink.bytes().to_vec());
}
