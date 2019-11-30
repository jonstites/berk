#![deny(unsafe_code)]

pub mod schema;
pub mod models;
pub mod repo;
pub mod args;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

pub fn eat(_data: &[u8]) {
}
