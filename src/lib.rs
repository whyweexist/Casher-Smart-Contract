
#![no_std]
use gstd::{msg,async_main, collections::HashMap , prelude::*,ActorId};
use io::*;

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));


// 1. Create the main state as a static variable.
static mut STATE:Option<WhisperCashState> = None;



// 2. Create the mutability function for your state.
fn state_mut() -> &'static mut WhisperCashState {

    let state = unsafe {  STATE.as_mut()};

    unsafe { state.unwrap_unchecked() }

}

// Create a Main State
#[derive(Clone, Default)]
pub struct WhisperCashState {
    pub current_state: String,
    pub all_users: HashMap<ActorId, String>,
}

// Create a implementation on State
impl WhisperCashState {
    #[allow(dead_code)]
    async fn firstmethod(&mut self) {}
    #[allow(dead_code)]
    async fn secondmethod(&mut self) { }
    #[allow(dead_code)]
    async fn thirdmethod(&mut self) {}
}


// 3. Create the init() function of your contract.
#[no_mangle]
extern "C" fn init () {


    let state = WhisperCashState {
        ..Default::default()
    };

    unsafe { STATE = Some(state) };


}


// 4.Create the main() or Async function for your contract.
#[async_main]
async fn main(){

        // We load the input message
        let action: WhisperCashAction = msg::load().expect("Could not load Action");

        // We receive an action from the user and update the state. Example:
        match action {
            WhisperCashAction::On => {

                // Create a variable with mutable state.
                let main_state = state_mut();


                main_state.current_state = "On".to_string();

                // Update your second field in state.
                main_state.all_users.insert(msg::source(), "On".to_string());


                 // Generate your event.
                 let _ =msg::reply(WhisperCashEvent::On,0);


            }
            WhisperCashAction::Off => {


                 // Create a variable with mutable state.
                let main_state = state_mut();

                
                // Update your first field in state.       
                main_state.current_state = "Off".to_string();

                // Update your second field in state.
                main_state.all_users.insert(msg::source(), "Off".to_string());


                 // Generate your event.
                 let _ =msg::reply(WhisperCashEvent::Off,0);

               

            }

        };
    }

        


// 5. Create the state() function of your contract.
#[no_mangle]
extern "C" fn state() {
   
    let state = unsafe { STATE.take().expect("Unexpected error in taking state") };

    msg::reply::<IoWhisperCashState>(state.into(), 0)
    .expect("Failed to encode or reply with `<ContractMetadata as Metadata>::State` from `state()`");
    
}


// Implementation of the From trait for converting CustomStruct to IoCustomStruct
impl From<WhisperCashState> for IoWhisperCashState {

    // Conversion method
    fn from(value: WhisperCashState) -> Self {
        // Destructure the CustomStruct object into its individual fields
        let WhisperCashState {
            current_state,
            all_users,
        } = value;

        // Perform some transformation on second field, cloning its elements (Warning: Just for HashMaps!!)
        let all_users = all_users.iter().map(|(k, v)| (*k, v.clone())).collect();
   
        // Create a new IoCustomStruct object using the destructured fields
        Self {
            current_state,
            all_users,
        }
    }
}