// use core::marker::PhantomData;

use alloc::string::String;

#[derive(Clone)]
pub enum NetError {
    Timeout,
}

pub trait Socket: Clone {
    fn tcp() -> Self; // Create TCP socket
    fn udp() -> Self; // Create UDP socket

    fn bind(&mut self, addr: &str, port: u16) -> Result<(), NetError>; // Start server
    fn connect(&mut self, addr: &str, port: u16) -> Result<(), NetError>;

    fn set_timeout(&mut self, timeout: i64); // Set recv timeout

    fn accept(&mut self) -> Result<Self, NetError>; // Wait for connections

    fn send(&mut self, msg: &str) -> Result<(), NetError>; // Send string
    fn recv(&mut self) -> Result<String, NetError>; // Recive a line

    fn close(&mut self); // Close socket
}

pub trait Request: Clone {} // TODO

// pub trait Net: Clone + Socket {}
// #[derive(Clone)]
// pub struct Net<S, R>
// where
//     S: Socket + 'static,
//     R: Request + 'static,
// {
//     _s_phantom: PhantomData<S>,
//     _r_phantom: PhantomData<R>,
// }

// impl<S, R> Net<S, R>
// where
//     S: Socket + 'static,
//     R: Request + 'static,
// {
//     pub fn new() -> Self {
//         Self {
//             _s_phantom: PhantomData,
//             _r_phantom: PhantomData,
//         }
//     }
// }
