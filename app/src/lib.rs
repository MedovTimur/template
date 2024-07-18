#![no_std]
use sails::prelude::*;
mod service;
use service::HelloWorld;

#[derive(Default)]
pub struct Program;

#[gprogram]
impl Program {
    pub fn new() -> Self {
        Self
    }
    pub fn hello_world(&self) -> HelloWorld {
        HelloWorld::default()
    }
}
