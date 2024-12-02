#![allow(non_snake_case)]
use std::mem::size_of_val;
use std::env;
use std::net::{ Ipv4Addr };//, IpAddr };

pub mod errors;
pub use crate::netlib::errors::*;

#[cfg(target_arch = "x86_64")]
pub mod syscalls_x86_64;
#[cfg(target_arch = "x86_64")]
use crate::netlib::syscalls_x86_64::*;

#[cfg(target_arch = "aarch64")]
pub mod syscalls_aarch64;
#[cfg(target_arch = "aarch64")]
use crate::netlib::syscalls_aarch64::*;


#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        if env::var("DEBUG_CUTTLEFISH").is_ok() {
            print!("DEBUG: ");
            println!($($arg)*);
        }
    }};
}
pub(crate) use debug; 


// ### Constants ###

// Source: https://github.com/torvalds/linux/blob/e8f897f4/include/uapi/asm-generic/poll.h#L11
// #define POLLIN          0x0001
// #define POLLPRI         0x0002
// #define POLLOUT         0x0004
// #define POLLERR         0x0008
// #define POLLHUP         0x0010
// #define POLLNVAL        0x0020

/*
Source
https://livegrep.com/
https://github.com/torvalds/linux/blob/3d7cb6b0/include/linux/socket.h#L183
https://github.com/torvalds/linux/blob/3d7cb6b0/include/linux/net.h#L49
https://github.com/torvalds/linux/blob/3d7cb6b0/include/uapi/linux/in.h#L31

Structs and consts
*/
pub const AF_INET:u16 = 2;
pub const SOCK_DGRAM:u16 = 2;
pub const SOCK_STREAM:u16 = 1;
// const SOCK_RAW:u16 = 3;

//pub const IPPROTO_ICMP:u16 = 1;
// const IP:u16 = 0;
pub const IPPROTO_UDP:u16 = 17;		/* User Datagram Protocol		*/
pub const IPPROTO_TCP:u16 = 6;		/* Transmission Control Protocol	*/
// const TCP:u16 = 6;		/* Transmission Control Protocol	*/
// const IPV6:u16 = 41;		/* IPv6-in-IPv4 tunnelling		*/
//

pub const RLIMIT_NOFILE:u16 = 7;
const SO_NONBLOCK:u64 = 2048;
/*
	SOCK_STREAM	= 1,
	SOCK_DGRAM	= 2,
	SOCK_RAW	= 3,
	SOCK_RDM	= 4,
	SOCK_SEQPACKET	= 5,
	SOCK_DCCP	= 6,
	SOCK_PACKET	= 10,
*/
// Legacy consts from Linux
// https://github.com/torvalds/linux/blob/3d7cb6b0/include/linux/socket.h#L334

pub const IP_TTL: u64 = 2;
pub const IP_RECVTTL: u64 = 11;
pub const IP_RECVERR: u64 = 12;

// const SOL_SOCKET: u64 = 1;
pub const SOL_IP: u64 = 0;

pub const MSG_ERRQUEUE: u64 = 8192;
//pub const POLLIN: u16 = 1;
//pub const POLLERR: u16 = 8;


/* TCP socket options */
pub const TCP_NODELAY:u64 = 1;	/* Turn off Nagle's algorithm. */
//pub const TCP_MAXSEG:u64 = 2;	/* Limit MSS */
//pub const TCP_CORK:u64 = 3;	/* Never send partially complete segments */
pub const TCP_KEEPIDLE:u64 = 4;	/* Start keeplives after this period */
pub const TCP_KEEPINTVL:u64 = 5;	/* Interval between keepalives */
//pub const TCP_KEEPCNT:u64 = 6;	/* Number of keepalives before death */
//pub const TCP_SYNCNT:u64 = 7;	/* Number of SYN retransmits */

pub const SOL_TCP:u64 = 6;
pub const SOL_SOCKET:u64 = 1;
pub const SO_KEEPALIVE:u64 = 9;

// Epoll things
pub const EPOLLIN: u32 = 1;
pub const EPOLLERR: u32 = 8;
pub const EPOLL_CTL_ADD: u32 = 1;

// Protocol consts
pub const ICMP: u16 = 1;
// const UDP: u8 = 17;
// const TCP: u8 = 6;

pub const DEFAULT_TIMEOUT: u64 = 5000;
pub const DEFAULT_ICMP_RATE: u64 = 1000;
// static mut ICMP_SEQUENCE: u16 = 1;


// ### Structures ###

// NOTE sockaddr is actually not usefull, search for sockaddr_in, since sockaddr is just a {
// sa_family, char: [] }
#[repr(C)]
// FIXME maybe I dont need to clone it
#[derive(PartialEq, Clone, Debug)]
pub struct sockaddr {
    pub sa_family: u16,   // address family, AF_xxx
    pub sin_port: u16,
    // Actually, there is no need to make in_addr struct for now, since it just makes the code less
    // readable
    //sin_addr: in_addr,
    pub sin_addr: u32,
    pub sin_zero: [u8; 8],
}

#[repr(C)]
pub struct rlimit {
    pub rlim_cur: usize,
    pub rlim_max: usize,
}

#[repr(C)]
#[derive(Debug)]
pub struct pollfd {
    pub fd: u32,
    pub events: u16,
    pub revents: u16,
}

#[repr(C)]
pub struct msghdr {
    pub msg_name: *const sockaddr,       /* optional address */
    pub msg_namelen: u32,    /* size of address */
    pub msg_iov: *mut iovec,        /* scatter/gather array */

    //msg_iovlen: u32,      /* # elements in msg_iov */
    pub msg_iovlen: usize,      /* # elements in msg_iov */
    pub msg_control: *mut [u8;165],    /* ancillary data, see below */
    pub msg_controllen: usize, /* ancillary data buffer len */
    pub msg_flags: i32,      /* flags on received message */
}

#[repr(C)]
pub struct iovec {                    /* Scatter/gather array items */
    pub iov_base: *mut u8,              /* Starting address */
    pub iov_len: usize,               /* Number of bytes to transfer */
}

// https://github.com/torvalds/linux/blob/3d7cb6b0/include/linux/socket.h#L96
// https://github.com/torvalds/linux/blob/3d7cb6b0/include/linux/socket.h#L334

// From https://github.com/rust-lang/libc
#[derive(Debug)]
#[derive(Clone)]
#[repr(C)]
// FIXME Maybe do this for arm too
#[cfg_attr(target_arch = "x86_64", repr(packed))]
//#[repr(packed)]
pub struct epoll_event {
    // Maybe change that to u32
    pub events: u32,
    pub data: u64,
    //pub pad: u64,
    //pub pad2: u128,
    //pub data: &epoll_data,
}

#[repr(C)]
#[derive(Debug)]
#[derive(Clone)]
pub struct epoll_data {
    pub u32: u32,
    pub u64: u64,
}


#[derive(Debug, Clone, PartialEq)]
pub struct Packet {
    pub protocol: u16,
    pub ip: Ipv4Addr,
    pub ttl: u8,
    pub port: u16,
//    pub latency: u8, float?
    pub data: Vec<u8>,
}

use std::fmt;
impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ip: {}, ttl: {}, port: {}", self.ip, self.ttl, self.port)
    }
}

#[derive(Debug, Clone)]
pub struct ErrorResponsePacket {
    pub Packet: Option<Packet>,
    pub Error: PacketError,
}

//#[derive(Debug, Clone, PartialEq)]
#[derive(Debug, Clone)]
pub enum PacketError {
    Timeout,
    NoRouteToHost,
    NetworkIsUnreachable,
    ExceededTTL,
}

// ### Functions ###

pub fn htons(u: u16) -> u16 { u.to_be() }
// fn ntohs(u: u16) -> u16 { Int::from_be(u) }
pub fn inet4_pton (ip: Ipv4Addr) -> u32 { u32::from_le_bytes(ip.octets()) }

// Opens a socket, doesnt really cares about anything except setsockopt, other errors handled in its (only) caller 'sendPacket'
fn createSocket (protocol: u16) -> i64 {

    let setsockoptValue: [u32;1] = [1;1];
    let setsockoptValue60: [u32;1] = [60;1];

    let fd = if protocol == IPPROTO_TCP {
        socket(AF_INET as u64, SOCK_STREAM as u64, protocol as u64)
    } else {
        socket(AF_INET as u64, SOCK_DGRAM as u64, protocol as u64)
    };

    if protocol == IPPROTO_TCP {
        setsockopt(fd as u64, SOL_TCP, TCP_NODELAY, &setsockoptValue, 4);
        setsockopt(fd as u64, SOL_SOCKET, SO_KEEPALIVE, &setsockoptValue, 4);
        setsockopt(fd as u64, SOL_TCP, TCP_KEEPIDLE, &setsockoptValue60, 4);
        setsockopt(fd as u64, SOL_TCP, TCP_KEEPINTVL, &setsockoptValue60, 4);
        setsockopt(fd as u64, SOL_IP, IP_RECVTTL, &setsockoptValue, 4);
    }

    let setsockoptRet1 = setsockopt(fd as u64, SOL_IP, IP_RECVTTL, &setsockoptValue, 4);
    let setsockoptRet2 = setsockopt(fd as u64, SOL_IP, IP_RECVERR, &setsockoptValue, 4);
    // Experimental for reciveing Time-to-life-exceeded msges
    let setsockoptRet3 = setsockopt(fd as u64, SOL_IP, 6, &setsockoptValue, 4);

    if setsockoptRet1 < 0 || setsockoptRet2 < 0 || setsockoptRet3 < 0 {
        println!("Error in setsockopt: {:?}, {:?}, {:?}", setsockoptRet1, setsockoptRet2, setsockoptRet3);
    }

    if fd == -13 {
        // Are we in container?
        // sudo sysctl -w net.ipv4.ping_group_range="0   2147483647"
        println!("Got permission denied for regular socket, are we in docker?");
        println!("Try this command: sudo sysctl -w net.ipv4.ping_group_range=\"0   2147483647\"");
        // TODO make a raw socket connection as a fallback (or maybe DRAM as a fallback) as
        // namespaces dont grant rights Going with raw socket
        unimplemented!();
    }

    fd
}


// TODO Do I need to save the packets order?
pub fn recvPacket (fd: u64, sockaddrStruct: &sockaddr, revents: u16, requestPacket: &Packet) -> Result<Packet, ErrorResponsePacket> {

    // Recv data buffers    
    // In that 'if' I solve problem with huge TCP responses which I dont want to use for small ICMP and UDP packets
    let mut iov_buffer = if requestPacket.protocol == IPPROTO_TCP { vec![0; 3500] } else { vec![0; 200] };
    let mut cmsgArray = [0; 165];
    let mut iovecStructArray = iovec {
        iov_base: iov_buffer.as_mut_ptr(),
        iov_len: iov_buffer.capacity()
    };

    let recvmsgFlag = if revents > 1 || revents == 0 { MSG_ERRQUEUE } else { 0 };
    let recvMsghdr = msghdr {
        msg_name: sockaddrStruct, 
        msg_namelen: 128, 
        msg_iov: &mut iovecStructArray, 
        msg_iovlen: 1, 
        msg_control: &mut cmsgArray, 
        msg_controllen: cmsgArray.len(), 
        msg_flags: 6 as i32 
    };

    // Eventually, I might switch to https://linux.die.net/man/2/recvmmsg
    match recvmsg(fd, &recvMsghdr, recvmsgFlag) {
        0.. => {
            match revents {
                8 => {
                    // debug!("cmsgArray: {:?}", cmsgArray);
                    // debug!("iov_buffer: {:?}", iov_buffer);

                    // FIXME there is a problem with weird returns like this packets was ExceededTTL but actually
                    // thats just returns of No Destination. Check Wireshark for more.
                    //
                    // Bug is caused by revents == 0 for the intermediate packets which should return "error" with
                    // TTL exceeded and with IP. But even worse, it never goes in that function since there is no
                    // logging of revents value in above logging.

                    if cmsgArray[45] == 3 && cmsgArray[46] == 1 {
                        Err(ErrorResponsePacket { Packet: None, Error: PacketError::NoRouteToHost } )
                    } else {
                        Err(ErrorResponsePacket {

                            Packet: Some( Packet {
                                protocol: requestPacket.protocol.clone(),
                                ip: Ipv4Addr::from([cmsgArray[60], cmsgArray[61], cmsgArray[62], cmsgArray[63]]),
                                ttl: cmsgArray[16],
                                port: requestPacket.port.clone(),
                                data: iov_buffer,
                            }), Error: PacketError::ExceededTTL } )
                    }

                },
                1 => {

                    let tmp_src_ip = u32::to_le_bytes(sockaddrStruct.sin_addr);

                    Ok( Packet {
                        protocol: requestPacket.protocol.clone(),
                        ip: Ipv4Addr::new(tmp_src_ip[0], tmp_src_ip[1], tmp_src_ip[2], tmp_src_ip[3]),
                        ttl: cmsgArray[16],
                        port: requestPacket.port.clone(),
                        data: iov_buffer,
                    })
                },
                _ => {
                    println!("Unimplemented error in recvmsg: {:?}", LinuxCodeToLinuxError(revents.into()));
                    unimplemented!()
                },
            }
        },

        -11 => Err(ErrorResponsePacket { Packet: None, Error: PacketError::Timeout }),
        -101 => Err(ErrorResponsePacket { Packet: None, Error: PacketError::NetworkIsUnreachable }), 
        -113 => Err(ErrorResponsePacket { Packet: None, Error: PacketError::NoRouteToHost }),

        any_other_err => {
            print!("Erorr in recvmsg: {:?}\n", LinuxCodeToLinuxError(any_other_err));
            unimplemented!();
        }
    }

}

pub fn sendPacket (packet: &Packet) -> Result<(u64, sockaddr), LinuxError> {
    debug!("sendPacket entry: Packet: {}", packet);

    let fd = createSocket(packet.protocol);
    if fd < 0 {
        debug!("createSocket: {:?}, error: {:?}", fd, LinuxCodeToLinuxError(fd));
        return Err(LinuxCodeToLinuxError(fd));
    }

    // I suppose it is better to set TTL here than in createSocket if custom TTL
    let setsockoptValue: [u32;1] = [packet.ttl as u32;1];
    let setsockRet = setsockopt(fd  as u64, 0, IP_TTL as u64, &setsockoptValue, 4);
    if setsockRet < 0 { return Err(LinuxCodeToLinuxError(setsockRet)); }
    
    fcntl(fd as u64, 4 as u64, SO_NONBLOCK);

    // Change ICMP sequence
    // TODO make this nice and without unsafe 
    // unsafe {
    // ICMP_DATA[7] = ICMP_sequence as u8;
    // ICMP_sequence = ICMP_sequence + 1;

    // sin_port is just a port to which you would like to connect. 0 in case of ICMP
    let sockaddrStruct = sockaddr { 
        sa_family:AF_INET, 
        sin_port: htons(packet.port), 
        sin_addr: inet4_pton(packet.ip), 
        sin_zero: [0; 8]
    };

    // FIXME Sometimes connect hangs forever.
    connect(fd as u64, &sockaddrStruct, size_of_val(&sockaddrStruct) as u64);

    let sendtoRes = sendto(
        fd as u64,
        packet.data.as_ptr(),
        packet.data.len() as u64,
        0 as u64,
        &sockaddrStruct,
        size_of_val(&sockaddrStruct) as u64);

    // FIXME
    // For some reason sometimes there is a EINVAL error if I am too fast with sending packets.
    // I have no idea how to fix this for now and here is the ditry hack
    debug!("sendPacket: sendtoRes: {:?}", sendtoRes);
    if sendtoRes == -22 {
        std::thread::sleep(std::time::Duration::from_millis(300));
        return sendPacket(packet);
    }
    if sendtoRes < 0 { return Err(LinuxCodeToLinuxError(sendtoRes)); }
    return Ok((fd as u64, sockaddrStruct));
}


// This function does NOT care for reciveing, it only sends packets in the order and thats it
pub fn sendTCPSequence (pkt: Packet, data: Vec<&str>) -> Result<(u64, sockaddr, Packet), LinuxError> {
    match sendPacket(&pkt) {
        Ok((fd, sockaddr)) => { 
            for packet_data in data {
                sendto(fd as u64, from_hex(packet_data).as_ptr(), from_hex(packet_data).len() as u64, 0 as u64, &sockaddr, size_of_val(&sockaddr) as u64);
            }
            Ok((fd, sockaddr, pkt))
        },
        Err(LinuxError) => Err(LinuxError),
    }
}

//https://docs.rs/hex/latest/src/hex/lib.rs.html#9-525
pub fn from_hex<T: AsRef<[u8]>>(hex: T) -> Vec<u8> {
    let hex = hex.as_ref();
    if hex.len() % 2 != 0 { return vec![0]; }
    hex.chunks(2)
        .enumerate()
        .map(|(_i, pair)| val(pair[0]) << 4 | val(pair[1]))
        .collect()
}

fn val(c: u8) -> u8 {
    match c {
        b'A'..=b'F' => c - b'A' + 10,
        b'a'..=b'f' => c - b'a' + 10,
        b'0'..=b'9' => c - b'0',
        _ => 0,
    }
}


// ### Syscall wrappers ###

// TODO: Make checks for root and make checks and etc for hard limit
/*
pub fn getSocketLimit () -> rlimit {
    let rlim = rlimit { rlim_cur: 0, rlim_max: 0 };
    getrlimit(RLIMIT_NOFILE as u64, &rlim);
    rlim
}
*/

pub fn setSocketLimit (newLimit: usize) -> i64 {
    setrlimit(RLIMIT_NOFILE as u64, &rlimit { rlim_cur: newLimit, rlim_max: 4096 })
}

pub fn epollWait(epollfd: u64, events: &mut Vec<epoll_event>) -> i64 {
    epoll_pwait(epollfd, events.as_mut_ptr(), events.len() as u64, DEFAULT_TIMEOUT)
}

pub fn epollCtl(epollfd: u64, epoll_event_code: u64, fd: u64, epoll_event: &epoll_event) -> i64 {
    epoll_ctl(epollfd, epoll_event_code, fd, epoll_event)
}

pub fn closeSocket (fd: u64) -> i64 { close(fd) }
