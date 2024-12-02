use core::ops::Index;
use core::str::FromStr;

use pictorus_traits::{BlockDataRead, BlockDataWrite, BlockDef, BlockParam};
use log::info;
use heapless::{String, Vec};

use orb_crate::*;

/// Struct representing a new block
pub struct ChannelRead<T, const D: usize> {
    name: &'static str,
    receiver: Option<embassy_sync::channel::Sender<'_, embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, T, D>>
}


/// Implement the BlockDef trait.
/// This is required to for a struct to be used as a block.
impl BlockDef for ChannelRead {
    /// Factory function for creating a new block instance
    ///
    /// # Arguments
    ///
    /// * `name` - The user-defined name of the block. Useful for debugging.
    /// * 'T' - The user-defined output type of the block.
    /// * `params` - A map of user-input block parameters by name.
    fn new(name: &'static str, params: &dyn Index<&str, Output = BlockParam>) -> Self {
        info!("Creating block {} with params: ", name);
        
        let me = Self {
            name,
            None,
        };

        me.accessor();

        me

    }

    fn accessor(&mut self){
        // This a bad placeholder, we need some way to scope &'static str name to be the identifier for importing a variable
        // declared in a shared crate between the main rust project and pictorus.
        // I'm sure somewhere in the block secret sauce you would have to have a map of available variables.
        // At compile time, we should know if this works and fail the build if the variable name is not present.
        // Ideally this would be something like......
        /*
        
        let variables: HashMap<&'static str, i32> = HashMap::from([
            ("foo", foo),
            ("bar", bar),
        ]);

        // Match a &'static str to a variable
        let input = "foo";
        match variables.get(input) {
            Some(value) => println!("The value of {} is {}", input, value),
            None => println!("Variable {} not found", input),
        }

        
        */

        //Of course the i32 is poor placeholder there. 

        // CALL THE ACCESSOR!!
        self.receiver = channel_receiver!(self.name)
    }

    /// Main run method for this block instance.
    /// This is called on every app iteration where the block is used.
    ///
    /// # Arguments
    ///
    /// * `inputs` - A slice of block inputs.
    ///                This contains the data of all upstream blocks connected
    ///                to this block's input ports.
    /// * `outputs` - A slice of block outputs.
    ///                 Data that gets set here will be passed to downstream blocks
    ///                 connected to the corresponding output ports.
    async fn run(&mut self, inputs: &[impl BlockDataRead], outputs: &mut [impl BlockDataWrite]) {

        self.reciever.receive().await
    }
}
