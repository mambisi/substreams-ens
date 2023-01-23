mod abi;
mod pb;
mod constants;


use substreams::errors::Error;
use substreams::hex;
use substreams::pb::substreams::Clock;
use substreams::prelude::*;
use substreams::scalar::{BigDecimal, BigInt};
use substreams::store;
use substreams_ethereum::pb::eth::v2::Block;
use crate::abi::baseregistrar::functions::Ens;
use crate::constants::ENS_REGISTRY;
use crate::pb::ens::Domain;


#[substreams::handlers::map]
pub fn map_domains_created(block: Block) -> Result<Domain, Error> {
}