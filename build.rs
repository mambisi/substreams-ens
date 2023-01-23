
use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("registry", "ens-subgraph/abis/Registry.json")?
        .generate()?
        .write_to_file("src/abi/registry.rs")?;
    Abigen::new("publicresolver", "ens-subgraph/abis/PublicResolver.json")?
        .generate()?
        .write_to_file("src/abi/public_resolver.rs")?;
    Abigen::new("namewrapper", "ens-subgraph/abis/NameWrapper.json")?
        .generate()?
        .write_to_file("src/abi/namewrapper.rs")?;
    Abigen::new("ethregistrar", "ens-subgraph/abis/EthRegistrarControllerOld.json")?
        .generate()?
        .write_to_file("src/abi/ethregistrar.rs")?;
    Abigen::new("deed", "ens-subgraph/abis/Deed.json")?
        .generate()?
        .write_to_file("src/abi/deed.rs")?;
    Abigen::new("baseregistrar", "ens-subgraph/abis/BaseRegistrar.json")?
        .generate()?
        .write_to_file("src/abi/baseregistrar.rs")?;
    Abigen::new("auctionregistrar", "ens-subgraph/abis/AuctionRegistrar.json")?
        .generate()?
        .write_to_file("src/abi/auctionregistrar.rs")?;
    Ok(())
}