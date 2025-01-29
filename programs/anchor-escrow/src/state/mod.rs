// Let's declare our sub-module where we will define the Escrow state.
pub mod escrow;

// We then expose the Escrow state to the rest of the program.
pub use escrow::*;