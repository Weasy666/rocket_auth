#![feature(proc_macro_hygiene, decl_macro)]
#![feature(crate_visibility_modifier)]
#![feature(label_break_value)]
#![feature(never_type)]
#![warn(clippy::all)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate log;

mod authenticator;
mod login;

// Reexport so that everything is in the crate namespace
pub use self::authenticator::{Authenticator, FromString};
pub use self::login::Login;
