/*---- Exported Modules ----*/

pub mod address;
pub mod validation;
pub mod node;

use net::node::Peer;
use net::address;


/*---- Constants ----*/

const LISTEN: bool = true;


/*---- Functions ----*/


// pub fn advertise_local(peer: &Peer) {
//     if LISTEN && peer.successfullyConnected {
//         local_address_set = address::AddressSet("127.0.01", )
//     }
// }