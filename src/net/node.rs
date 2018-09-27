use net::address::{ AddressAsPeer, ServiceFlag };

/* Equivalent to Bitcon's CNode  */
#[derive(Clone, Debug)]
pub struct Peer {
    id: u64,
    services: ServiceFlag,
    startingHeight: u8,
    address: AddressAsPeer,
    successfullyConnected: bool
    // keyedNetGroupIn
    // localHostNonceIn
    // socket
}

