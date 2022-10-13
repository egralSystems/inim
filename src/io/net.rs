use std::prelude::v1::*;

use rhai::{Engine, EvalAltResult};

pub fn register_socket<S: Socket>(engine: &mut Engine) {
    engine
        .register_type_with_name::<S>("Socket")
        .register_fn("tcp", S::tcp)
        .register_fn("udp", S::udp)
        .register_fn("bind", S::bind)
        .register_fn("connect", S::connect)
        .register_fn("set_timeout", S::set_timeout)
        .register_fn("accept", S::accept)
        .register_fn("send", S::send)
        .register_fn("recv", S::recv)
        .register_fn("close", S::close);
}

pub trait Socket: Clone + 'static {
    fn tcp() -> Self; // Create TCP socket
    fn udp() -> Self; // Create UDP socket

    fn bind(&mut self, addr: &str, port: u16) -> Result<(), Box<EvalAltResult>>; // Start server
    fn connect(&mut self, addr: &str, port: u16) -> Result<(), Box<EvalAltResult>>;

    fn set_timeout(&mut self, timeout: i64); // Set recv timeout

    fn accept(&mut self) -> Result<Self, Box<EvalAltResult>>; // Wait for connections

    fn send(&mut self, msg: &str) -> Result<(), Box<EvalAltResult>>; // Send string
    fn recv(&mut self) -> Result<String, Box<EvalAltResult>>; // Recive a line

    fn close(&mut self); // Close socket
}
