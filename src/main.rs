/* toy blockchain based on the code from  
   https://dev.to/ecj222/how-to-build-a-blockchain-from-scratch-in-rust-46
 */
#[allow(unused_imports)]
mod blockch;
use crate::blockch::blockchain;

fn main() {
    let difficulty = 1;
    let mut blockchain = blockchain::Blockchain::new(difficulty);
    blockchain::Blockchain::add_block(&mut blockchain);
    blockchain::Blockchain::add_block(&mut blockchain);
    blockchain::Blockchain::add_block(&mut blockchain);
}
