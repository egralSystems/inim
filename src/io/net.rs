use std::prelude::v1::*;

use rhai::Engine;

pub fn register_socket<S: Net>(engine: &mut Engine) {
    engine
        .register_type_with_name::<S>("Socket")
        .register_fn("tcp", S::tcp)
        .register_fn("udp", S::udp)
        .register_fn("bind", S::bind)
        .register_fn("connect", S::connect)
        .register_fn("set_timeout", S::set_timeout)
        .register_fn("accept", S::accept)
        .register_fn("send_string", S::send_string)
        .register_fn("send_blob", S::send_blob)
        .register_fn("recv_string", S::recv_string)
        .register_fn("recv_line", S::recv_line)
        .register_fn("recv_blob", S::recv_blob)
        .register_fn("close", S::close);
}

pub trait Net: Clone + 'static {
    fn tcp() -> Self; // Create TCP socket
    fn udp() -> Self; // Create UDP socket

    fn bind(&mut self, addr: &str, port: u16) -> String; // Start server
    fn connect(&mut self, addr: &str, port: u16) -> String; // Connect to server

    fn set_timeout(&mut self, timeout: i64); // Set recv timeout

    fn accept(&mut self) -> Self; // Wait for connections

    fn send_string(&mut self, msg: &str) -> String; // Send string
    fn recv_string(&mut self, char_count: i64) -> String; // Receive a string
    fn recv_line(&mut self) -> String;

    fn send_blob(&mut self, msg: Vec<u8>) -> String; // Send bytes
    fn recv_blob(&mut self, byte_count: i64) -> Vec<u8>; // Receive a blob

    fn close(&mut self); // Close socket
}
