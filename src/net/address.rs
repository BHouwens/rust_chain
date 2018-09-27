pub enum ServiceFlag {
    None,           // Nothing
    Network,        // NETWORK means that the node is capable of serving the complete block chain (no pruning)
    GetUTXO,        // GETUTXO means the node is capable of responding to the getutxo protocol request.
    Bloom,          // BLOOM means the node is capable and willing to handle bloom-filtered connections.
    Witness,        // WITNESS indicates that a node can be asked for blocks and transactions including witness data.
    XThin,          // XTHIN means the node supports Xtreme Thinblocks
    NetworkLimited  // NETWORK_LIMITED means the same as NETWORK with the limitation of only serving the last 288 (2 day) blocks
}

/* Equivalent to Bitcon's CService  */
#[derive(Clone, Debug)]
pub struct AddressSet {
    address: String,
    port: u64
}

impl AddressSet {
    fn new(address: String, port: u64) -> AddressSet {
        AddressSet {
            address: address,
            port: port
        }
    }
}

/* Equivalent to Bitcon's CAddress  */
#[derive(Clone, Debug)]
pub struct AddressAsPeer {
    address: AddressSet,
    time: u64,
    services: ServiceFlag
}

impl AddressAsPeer {
    fn new(address_set: AddressSet, time: u64, services: ServiceFlag) -> AddressAsPeer {
        AddressAsPeer {
            address: address_set,
            time: time,
            services: services
        }
    }
}