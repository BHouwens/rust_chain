/*---- Exported Modules ----*/

pub mod address;
pub mod validation;
pub mod node;

use node::Peer;
use address;


/*---- Constants ----*/

const LISTEN: bool = true;



/*---- Functions ----*/

/**
 * Pushes our address to a peer
 * 
 * `peer` - Peer to send to
 */

pub fn advertise_local(peer: &Peer) {
    if LISTEN && peer.successfullyConnected {
        local_address_set = address::AddressSet("127.0.01", )
    }
}