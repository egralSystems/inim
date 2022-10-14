use std::prelude::v1::*;

use rhai::Engine;

pub fn register_net<N: Net>(engine: &mut Engine) {
    engine
        .register_type_with_name::<N>("Net")
        .register_fn("net", N::tcp)
        .register_get("addr", N::addr)
        .register_fn("bind", N::bind)
        .register_fn("connect", N::connect)
        .register_fn("set_timeout", N::set_timeout)
        .register_fn("accept", N::accept)
        .register_fn("write", N::send_string)
        .register_fn("write", N::send_blob)
        .register_fn("read_str", N::recv_string)
        .register_fn("read_line", N::recv_line)
        .register_fn("read_blob", N::recv_blob_amount)
        .register_fn("read_blob", N::recv_blob)
        .register_fn("close", N::close);
}

pub trait Net: Clone + 'static {
    fn tcp() -> Self; // Create TCP socket

    fn addr(&mut self) -> String;

    fn bind(&mut self, addr: &str) -> String; // Start server
    fn connect(&mut self, addr: &str) -> String; // Connect to server

    fn set_timeout(&mut self, timeout: i64); // Set recv timeout

    fn accept(&mut self) -> Self; // Wait for connections

    fn send_string(&mut self, msg: &str) -> String; // Send string
    fn recv_string(&mut self, char_count: i64) -> String; // Receive a string
    fn recv_line(&mut self) -> String;

    fn send_blob(&mut self, msg: Vec<u8>) -> String; // Send bytes
    fn recv_blob_amount(&mut self, byte_count: i64) -> Vec<u8>; // Receive a blob
    fn recv_blob(&mut self) -> Vec<u8>;

    fn close(&mut self); // Close socket
}
