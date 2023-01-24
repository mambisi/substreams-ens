pub(crate) mod abi;
pub(crate) mod constants;
mod keyer;
pub(crate) mod pb;
mod rpc;

use crate::abi::baseregistrar::functions::Ens;
use crate::abi::registry::events::NewOwner;
use crate::constants::{EMPTY_ADDRESS, ENS_REGISTRY, ROOT_NODE};
use crate::pb::ens::{Domain, Domains};
use ethabi::ethereum_types::H256;
use ethabi::{Address, Event};
use substreams::errors::Error;
use substreams::hex;
use substreams::pb::substreams::Clock;
use substreams::prelude::*;
use substreams::scalar::{BigDecimal, BigInt};
use substreams::store;
use substreams_ethereum::pb::eth::v2::Block;
use tiny_keccak::{Hasher, Keccak};

fn get_subnode(node: &[u8; 32], label: &[u8; 32]) -> [u8; 32] {
    let mut out = [0; 32];
    let mut keccak256 = Keccak::v256();
    keccak256.update(node);
    keccak256.update(label);
    keccak256.finalize(&mut out);
    out
}

#[substreams::handlers::map]
pub fn map_new_owner(block: Block) -> Result<Domains, Error> {
    use abi::registry::events::NewOwner;
    Ok(Domains {
        domains: block
            .events::<NewOwner>(&[&ENS_REGISTRY])
            .map(|(event, log)| {
                let subnode = H256::from(get_subnode(&event.node, &event.label));
                let mut domain = Domain::default();
                domain.id = hex::encode(subnode.as_bytes());
                domain.created_at_timestamp = block.timestamp_seconds();
                domain.created_at_block = block.number;
                domain.owner = event.owner;
                if event.node == ROOT_NODE {
                    domain.owner = EMPTY_ADDRESS.to_vec();
                    domain.is_migrated = true;
                }

                let resolver_address =
                    rpc::get_resolver_address_call(&Address::from(&ENS_REGISTRY), &subnode)
                        .unwrap();
                let parent_name = rpc::get_resolver_address_call(
                    &Address::from(&ENS_REGISTRY),
                    &H256::from(event.node),
                )
                .map(|addr| rpc::get_name_call(&addr, &H256::from(event.node)))
                .flatten();

                let mut label = match rpc::get_name_call(&resolver_address, &subnode) {
                    None => {
                        format!("[{}]", &hex::encode(event.label)[2..])
                    }
                    Some(label) => {
                        domain.label_name = label.clone();
                        label
                    }
                };
                if event.node == ROOT_NODE {
                    domain.name = label
                } else {
                    if let Some(parent_name) = parent_name {
                        domain.name = format!("{}.{}", label, parent_name)
                    }
                }
                domain
            })
            .collect(),
    })
}

#[substreams::handlers::store]
pub fn store_domains(domains: Domains, store: StoreSetProto<Domain>) {
    for domain in domains.domains {
        store.set(domain.log_ordinal, keyer::domain_key(&domain.id), &domain)
    }
}
