//! Voice connection and gateway definitions.
#![deny(missing_docs)]

mod close_code;
mod opcode;
mod voice_region;
mod voice_state;

pub use self::{
    close_code::CloseCode, opcode::OpCode, voice_region::VoiceRegion, voice_state::VoiceState,
};
