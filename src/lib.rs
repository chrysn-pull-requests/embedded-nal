//! # embedded-nal - A Network Abstraction Layer for Embedded Systems

#![doc(html_root_url = "https://docs.rs/embedded-nal/0.2.0")]
#![no_std]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod dns;
mod stack;

pub use nb;
pub use no_std_net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

pub use dns::{AddrType, Dns};
pub use stack::{TcpClientStack, TcpFullStack, UdpClientStack, UdpFullStack};
