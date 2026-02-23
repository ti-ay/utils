//! Utility macros
//! 
//! ### Setup
//! ```toml
//! [features]
//! 
//! default = []
//! all_off = ["ut/note_off", "ut/time_off", "ut/view_off"]
//! all_on = ["ut/note_on", "ut/time_on", "ut/view_on"]
//! ```

pub mod time;
pub mod note;
pub mod view;

