use crate::abi;
use crate::constants::ROOT_NODE;
use crate::pb::ens::Resolver;
use ethabi::ethereum_types::H256;
use ethabi::Address;
use substreams::Hex;
use substreams_ethereum::pb::eth::rpc::RpcCall;
use substreams_ethereum::rpc::RpcBatch;

pub fn get_name_call(resolver_address: &Address, node: &H256) -> Option<String> {
    let responses = RpcBatch::new()
        .add(
            abi::public_resolver::functions::Name {
                node: *node.as_fixed_bytes(),
            },
            resolver_address.as_fixed_bytes().to_vec(),
        )
        .execute()
        .unwrap()
        .responses;

    RpcBatch::decode::<_, abi::public_resolver::functions::Name>(&responses[0])
}

pub fn get_resolver_address_call(registry_address: &Address, node: &H256) -> Option<Address> {
    let responses = RpcBatch::new()
        .add(
            abi::registry::functions::Resolver {
                node: *node.as_fixed_bytes(),
            },
            registry_address.as_fixed_bytes().to_vec(),
        )
        .execute()
        .unwrap()
        .responses;

    RpcBatch::decode::<_, abi::registry::functions::Resolver>(&responses[0])
        .map(|resolver| Address::from_slice(resolver.as_slice()))
}
