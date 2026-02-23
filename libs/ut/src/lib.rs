//! Utility macros
//! ### Available macros
//! + [`ut_note`]
//! + [`ut_time`]
//! + [`ut_view`]
//! + [`ut_out`]
//! + [`ut_outln`]
//! 
//! 
//! ### Available features
//! + ut_on -> all_on
//! + ut_off -> all_off
//! 
//! + dbg_on -> view, time, note on
//! + dbg_off -> view, time, note off
//! 
//! + note_on -> note on
//! + note_off -> note off
//! 
//! + time_on -> time on
//! + time_off -> time off
//! 
//! + view_on -> view on
//! + view_off -> view off
//! 
//! + out_on -> out and outln on
//! + out_off -> out and outln off
//! 
//! ### Setup
//! ```toml
//! [features]
//! 
//! default = []
//! all_off = ["ut/note_off", "ut/time_off", "ut/view_off", "ut/out_off"]
//! all_on = ["ut/note_on", "ut/time_on", "ut/view_on", "ut/out_on"]
//! ```

pub mod time;
pub mod note;
pub mod view;
pub mod out;

