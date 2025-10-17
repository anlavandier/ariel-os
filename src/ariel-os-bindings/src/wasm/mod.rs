#[cfg(feature = "log")]
pub mod log;

#[cfg(feature = "rng")]
pub mod rng;

#[cfg(feature = "time")]
pub mod time;

#[cfg(feature = "udp")]
pub mod udp;

// #[cfg(feature = "coap-server")]
// mod coap_server;

pub struct ArielHost<'a> {
    #[cfg(all(feature = "rng", feature = "async"))]
    rng: ariel_os_random::FastRngSend,

    #[cfg(all(feature = "rng", not(feature = "async")))]
    rng: ariel_os_random::FastRng,

    #[cfg(all(feature = "udp", not(feature = "async")))]
    socket: Option<ariel_os_embassy::reexports::embassy_net::udp::UdpSocket<'a>>,

    #[cfg(all(feature = "udp", feature = "async"))]
    socket: compile_error!("UDP and async are incompatible as of right now"),

    #[cfg(feature = "udp")]
    buffer_size: usize,

    // #[cfg(feature = "coap-server")]
    // coap_server_host: coap_server::ArielCoapServerHost,

    // Used to mark the lifetime and type parameter(s) when the features don't use them
    _marker: core::marker::PhantomData<&'a ()>,


}

#[allow(clippy::derivable_impls, reason = "conditional compilation")]
impl Default for ArielHost<'_> {
    fn default() -> Self {
        ArielHost {
            #[cfg(all(feature = "rng", feature = "async"))]
            rng: ariel_os_random::fast_rng_send(),

            #[cfg(all(feature = "rng", not(feature = "async")))]
            rng: ariel_os_random::fast_rng(),

            #[cfg(feature = "udp")]
            socket: None,

            #[cfg(feature = "udp")]
            buffer_size: 0,

            // #[cfg(feature = "coap-server")]
            // coap_server_host: coap_server::ArielCoapServerHost::default(),

            _marker: core::marker::PhantomData,

        }
    }
}



#[cfg(feature = "rng")]
mod rng_impl{
    extern crate alloc;

    use alloc::vec::Vec;
    use rand_core::RngCore as _;

    use wasmtime::component::Resource;
    use super::rng;
    use super::ArielHost;

    impl rng::HostRNG for ArielHost<'_> {

        fn next_u32(&mut self,) -> u32 {
            self.rng.next_u32()
        }
        fn next_u64(&mut self,) -> u64 {
            self.rng.next_u64()
        }
        fn random_bytes(&mut self,len:u32,) -> Vec<u8> {
            let mut dest: Vec<u8> = core::iter::repeat_n(0, len as usize).collect();
            self.rng.fill_bytes(&mut dest);
            dest
        }
        fn drop(&mut self, _: Resource<rng::RNG>) -> wasmtime::Result<()> {
            unreachable!("Should never be dropped since it's never instantiated")
        }
    }

    impl rng::Host for ArielHost<'_> {}
}

#[cfg(feature = "time")]
mod time_impl {
    use super::time;
    use super::ArielHost;
    use ariel_os_embassy::api::time::{Timer, Instant};


    impl time::Host for ArielHost<'_> {
        #[cfg(feature = "async")]
        async fn sleep(&mut self, millis: u64) {
            Timer::after_millis(millis).await;
        }

        #[cfg(not(feature = "async"))]
        fn sleep(&mut self, millis: u64) {
            todo!("Locked until a fix for the Embassy Timer lands");
        }

        fn now_as_millis(&mut self) -> u64 {
            Instant::now().as_millis()
        }
    }
}

#[cfg(feature = "log")]
mod log_impl {
    extern crate alloc;
    use super::log;
    use super::ArielHost;
    use alloc::string::String;
    use ariel_os_debug::log::info;

    impl log::Host for ArielHost<'_> {
        fn info(&mut self, input: String) {
            info!("[WASM] {}", input.as_str());
        }
   }
}


#[cfg(feature = "udp")]
mod udp_impl {

    use super::udp;
    use super::ArielHost;

    use core::mem;

    extern crate alloc;
    use alloc::vec::Vec;

    use ariel_os_debug::log::info;
    use ariel_os_embassy::reexports::embassy_net::{self, IpAddress};
    use embassy_net::udp::{PacketMetadata, UdpMetadata, UdpSocket};

    use wasmtime::component::Resource;

    use embassy_futures::block_on;


    impl<'a> ArielHost<'a> {
        pub fn initialize_socket(&mut self,
            stack: ariel_os_embassy::NetworkStack,
            rx_meta: &'a mut [PacketMetadata],
            rx_buffer: &'a mut [u8],
            tx_meta: &'a mut [PacketMetadata],
            tx_buffer: &'a mut [u8],
        ) {
            let buff_size = tx_buffer.len();
            let rx_meta: &'static mut [PacketMetadata] = unsafe { mem::transmute(rx_meta) };
            let rx_buffer: &'static mut [u8] = unsafe { mem::transmute(rx_buffer) };
            let tx_meta: &'static mut [PacketMetadata] = unsafe { mem::transmute(tx_meta) };
            let tx_buffer: &'static mut [u8] = unsafe { mem::transmute(tx_buffer) };

            let socket = Some(UdpSocket::new(
                stack, rx_meta, rx_buffer,
                tx_meta, tx_buffer
            ));
            self.socket = socket;
            self.buffer_size = buff_size;
        }
    }

    impl udp::Ipv4Addr {
        fn from_octets(octets: [u8; 4]) -> Self {
            Self {
                a: octets[0],
                b: octets[1],
                c: octets[2],
                d: octets[3],
            }
        }
    }

    #[cfg(feature = "ipv6")]
    impl udp::Ipv6Addr {
        fn from_segments(segments: [u16; 8]) -> Self {
            Self {
                a: segments[0],
                b: segments[1],
                c: segments[2],
                d: segments[3],
                e: segments[4],
                f: segments[5],
                g: segments[6],
                h: segments[7],
            }
        }
    }


    impl From<IpAddress> for udp::IpAddr {
        fn from(t: IpAddress) -> Self {
            match t {
                IpAddress::Ipv4(ipaddr) => {
                    let octs = ipaddr.octets();
                    udp::IpAddr::V4(
                        udp::Ipv4Addr::from_octets(octs)
                    )
                },
                #[cfg(feature = "ipv6")]
                IpAddress::Ipv6(ipaddr) => {
                    let segments = ipaddr.segments();
                    udp::IpAddr::V6(
                        udp::Ipv6Addr::from_segments(segments)
                    )
                },
                #[allow(unreachable_patterns, reason = "Conditional compilation")]
                _ => unreachable!()
            }
        }
    }

    impl From<udp::IpAddr> for IpAddress {
        fn from(t: udp::IpAddr) -> Self {
            match t {
                udp::IpAddr::V4(ipaddr) => {
                    let udp::Ipv4Addr { a, b, c, d } = ipaddr;
                    Self::v4(a, b, c, d)
                },
                #[cfg(feature = "ipv6")]
                udp::IpAddr::V6(ipaddr) => {
                    let udp::Ipv6Addr { a, b, c, d, e, f, g, h } = ipaddr;
                    Self::v6(a, b, c, d, e, f, g, h)
                },
                #[allow(unreachable_patterns, reason = "Conditional compilation")]
                _ => unreachable!()
            }
        }
    }

    impl From<UdpMetadata> for udp::UdpMetadata {
        fn from(t: UdpMetadata) -> Self {
            let UdpMetadata { endpoint, local_address, meta: _ } = t;
            let e_addr = endpoint.addr.into();
            let e_port = endpoint.port;

            Self {
                endpoint: udp::Endpoint { addr: e_addr, port: e_port },
                local_addr: local_address.map(udp::IpAddr::from),
            }
        }
    }

    impl From<udp::UdpMetadata> for UdpMetadata {
        fn from(t: udp::UdpMetadata) -> Self {
            let udp::UdpMetadata { endpoint, local_addr: _ } = t;
            let e_addr: IpAddress = endpoint.addr.into();
            let e_port = endpoint.port;

            (e_addr, e_port).into()
        }
    }

    impl<'a> udp::HostUdpSocket for ArielHost<'a> {
        fn bind(&mut self, port: u16) -> Result<(), ()> {
            self.socket.as_mut().map(
                |socket| { socket.bind(port).map_err(|_| ()) }
            ).unwrap_or(Err(()))
        }

        // This function is blocking the host
        fn send(&mut self, data: Vec<u8>, endpoint: udp::UdpMetadata) -> Result<(), ()> {
            info!("Sending some data");
            self.socket.as_mut().map(
                |socket| {
                    let endpoint = UdpMetadata::from(endpoint);
                    info!("Sending some data to {:?}", endpoint);
                    block_on(socket.send_to(&data, endpoint)).map_err(|_| ())
                }
            ).unwrap_or(Err(()))
        }

        fn try_recv(&mut self) -> Result<Option<(Vec<u8>, udp::UdpMetadata)>, ()> {
            self.socket.as_mut().map(
                |socket| {
                    if socket.may_recv() {
                        let mut buf: Vec<u8> = core::iter::repeat_n(0, self.buffer_size).collect();
                        match block_on(socket.recv_from(&mut buf)) {
                            Err(_) => Err(()),
                            Ok((0, _)) => Ok(None),
                            Ok((n, endpoint)) => {
                                info!("Received some data from {:?}", endpoint);
                                buf.truncate(n);
                                Ok(Some((buf, endpoint.into())))
                            }
                        }
                    }
                    else {
                        Ok(None)
                    }
                }
            ).unwrap_or(Err(()))
        }

        fn drop(&mut self, _: Resource<udp::UdpSocket>) -> wasmtime::Result<()> {
            unreachable!()
        }
    }

    impl udp::Host for ArielHost<'_> {}
}


// #[cfg(feature = "coap-server")]
// mod coap_server_impl {
//     use super::coap_server;
//     use super::ArielHost;
//     impl coap_server::Host for ArielHost<'_> {
//         fn register(&mut self, at: &str, response: coap_server::CoapReply) {
//             todo!()
//         }

//         async  fn run(&mut self,) {
//             todo!()
//         }

//     }
// }