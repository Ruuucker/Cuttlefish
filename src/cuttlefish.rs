#![allow(non_snake_case)]
use std::time::SystemTime;
use crate::cuttlefish::graph::Graph;
use std::collections::HashMap;
use std::net::Ipv4Addr;

//mod ssh;
mod host_fingerprinting;
pub use self::host_fingerprinting::*;
//pub use host_fingerprinting::snmp::*;

pub mod netlib;
pub use netlib::*;

mod graph;
use std::env;

// 0, 3 here is the ICMP sequence number, need to keep track of it
// static ICMP_DATA: [u8; 48] = [8, 0, 252, 153, 255, 255, 0, 3, 234,37,100,0,0,0,0,0,
//     40, 41, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
//     40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55];

const ICMP_DATA: &str = "080042a505280001ee70186600000000dd870d0000000000101112131415161718191a1b1c1d1e1f202122232425262728292a2b2c2d2e2f3031323334353637";

// This feels a little wrong, but at the time Rust cannot allocate memory at compile time and I
// dont want this allocation at runtime.

// AFAIK admins tend to give servers a pretty IP, and also they use them more often, so for the
// basic discovery, we will ping every pretty IP in each subnet.
const SUBNETS_DISCOVERY: [Packet; 40] = [
    Packet { protocol: ICMP, ip: Ipv4Addr::new(10, 0, 0, 1), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(10, 0, 1, 1), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(10, 1, 0, 1), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(10, 1, 1, 1), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(10, 10, 10, 1), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(10, 10, 10, 10), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(10, 10, 0, 1), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(10, 10, 1, 1), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(10, 0, 10, 1), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(10, 0, 10, 10), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(10, 100, 100, 1), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(10, 100, 100, 100), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(10, 200, 100, 100), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(10, 200, 200, 200), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(10, 200, 200, 1), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(10, 11, 11, 11), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(10, 11, 11, 1), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(10, 0, 11, 1), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(10, 20, 0, 1), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(10, 20, 20, 1), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(10, 20, 20, 20), ttl: 64, data: Vec::new(), port: 0 }, 

    Packet { protocol: ICMP, ip: Ipv4Addr::new(172, 16, 0, 1), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(172, 16, 1, 1), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(172, 32, 0, 1), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(172, 32, 1, 1), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(172, 16, 16, 16), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(172, 10, 10, 10), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(172, 10, 10, 1), ttl: 64, data: Vec::new(), port: 0 }, 

    Packet { protocol: ICMP, ip: Ipv4Addr::new(192, 168, 0, 1), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(192, 168, 1, 1), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(192, 168, 10, 1), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(192, 168, 1, 10), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(192, 168, 10, 10), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(192, 168, 20, 1), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(192, 168, 20, 10), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(192, 168, 168, 1), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(192, 168, 168, 168), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(192, 192, 1, 1), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(192, 192, 192, 1), ttl: 64, data: Vec::new(), port: 0 }, 
    Packet { protocol: ICMP, ip: Ipv4Addr::new(192, 192, 192, 192), ttl: 64, data: Vec::new(), port: 0 }, 
];


// Gets one ip, pings every ip in 0/24 range
// For now its just 255 ips, maybe later I will need to ping bigger subnets. There was a function for sunbet creation which I deleted, see in git history.
fn pingSubnet (ip: Ipv4Addr, octetIndex: usize) -> Vec<(u64, sockaddr, Packet)> {

    let mut octets = ip.octets();
    let mut retvec = Vec::new();
    let firstNumber = if octetIndex == 3 { 1 } else { 0 };

    for newOctet in firstNumber..=254 {
        octets[octetIndex] = newOctet;
        let ipAddress = Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]);
        let pkt_to_send = Packet {
                    protocol: ICMP,
                    ip: ipAddress,
                    ttl: 64,
                    port: 0,
                    data: from_hex(ICMP_DATA)
                    };

        match sendPacket(&pkt_to_send) {
            Ok((fd, sockaddr)) => retvec.push((fd, sockaddr, pkt_to_send)),

            Err(LinuxError) => {
                match LinuxError {
                    // TODO implement return of the errors at the event loop
                    //LinuxError::TooManyOpenFiles => continue, // Is this continue will be for main loop of for the closures?
                    _ => unimplemented!(),
                }
            }
        }
    }

    retvec
}

fn sendFirstDiscoveryPings() -> Vec<(u64, sockaddr, Packet)> {
    let mut discovery_ret_vec: Vec<(u64, sockaddr, Packet)> = Vec::new();

    for mut packet in SUBNETS_DISCOVERY {
        packet.data = from_hex(ICMP_DATA);
        match sendPacket(&packet) {
            Ok((fd, sockaddr)) => discovery_ret_vec.push((fd, sockaddr, packet)),
            Err(LinuxError) => {
                println!("Unimplemented error from sendPacket: {:?}", LinuxError);
                // TODO implement return of the errors at the event loop
                unimplemented!()
            },
        }
    }
    discovery_ret_vec
}

fn tracerouteToIP(ip: Ipv4Addr, ttl: u8) -> Result<Vec<(u64, sockaddr, Packet)>, LinuxError> {

    let mut traceroutes_fds: Vec<(u64, sockaddr, Packet)> = Vec::new();
    let maxTTL: u8 = match ttl {
        // Linux case
        // Sometimes, servers like 1.1.1.1 put weird TTL like
        // 68, so we add numbers for a good measure
        // 0..=64 => 64 - ttl,
        0..=64 => 64 - ttl + 3,
        // Windows case
        65..=127 => 127 - ttl,
        // Linux BGP and other big net case
        128..=255 => 255 - ttl,
    };

    for iteractive_ttl in 1..=maxTTL {
        let pkt = Packet {
             protocol: ICMP,
            //protocol: IPPROTO_TCP,
            ip: ip,
            ttl: iteractive_ttl,
            port: 0,
            data: from_hex(ICMP_DATA)
        };

        match sendPacket(&pkt) {
            Ok((fd, sockaddr)) => traceroutes_fds.push((fd, sockaddr, pkt)),
            // TODO better to check for errors on every packet separatly
            Err(LinuxError) => { return Err(LinuxError); }, 
        }
    }
    Ok(traceroutes_fds)
}

fn isLinuxTTL(ttl: u8) -> bool {
    return match ttl {
        // Linux case
        // Sometimes, servers like 1.1.1.1 put weird TTL like
        // 68, so we add numbers for a good measure
        // 0..=64 => 64 - ttl,
        0..=64 => true,
        // Windows case
        65..=127 => false,
        // Linux BGP and other big net case
        128..=255 => true,
    };
}

fn unwrapResult (result: Result<Packet, ErrorResponsePacket>) -> Option<Packet> {

    match result {
        Err(EPacket) => {
            match EPacket.Error {
                PacketError::ExceededTTL => {
                    // If we dont have packet from ExceededTTL than we doing something wrong
                    // EPacket.Packet.unwrap()
                    EPacket.Packet
                },
                PacketError::Timeout => None,
                PacketError::NoRouteToHost => None,
                PacketError::NetworkIsUnreachable => None,
            }
        },
        Ok(Packet) => Some(Packet)
    }

}


fn calculateNeibors(ip: Ipv4Addr) -> Vec<Ipv4Addr> {
    let octets = ip.octets();
    let mut return_vector = vec![];

    if octets[2] != 0 {
        return_vector.push(Ipv4Addr::new(octets[0], octets[1], octets[2] - 1, 1));
    }
    if octets[2] < 254 {
        return_vector.push(Ipv4Addr::new(octets[0], octets[1], octets[2] + 1, 1));
    }
    return_vector

}


#[derive(PartialEq, Clone, Debug)]
pub struct FdUtilityInfo {
    //fd: u64,
    sockaddr: sockaddr,
    sendTime: SystemTime,
    requestPacket: Packet,
    callback: PacketCallbackChoose,
    graphId: u32,
    index: usize,
    // At the moment it seems the best solution to mark the packet by its supposed graph place
    // graphPath: Vec<u32>,
}

#[derive(Debug, PartialEq, Clone)]
enum PacketCallbackChoose {
    Traceroute,
    PingSubnet,
    PostDiscovery,
    Fingerprint,
}


fn waitForPackets (epollfd: u64, epoll_waitVec: &mut Vec<epoll_event>, fd_hashmap: &mut HashMap<u64, FdUtilityInfo>) -> Vec<(Result<Packet, ErrorResponsePacket>, FdUtilityInfo)> {

    debug!("Starting to wait for epollWait");
    let mut return_vec = Vec::new();
    let return_packets_count = epollWait(epollfd, epoll_waitVec);
    debug!("Returned from epollWait: {:?}", return_packets_count);
    if return_packets_count < 0 {
        println!("Error in epollWait, waitForPackets function: {:?}", LinuxCodeToLinuxError(return_packets_count));
        panic!("");
    }

    for count in 0..return_packets_count as usize {
        let event = &epoll_waitVec[count];
        let fd = event.data;
        let fd_utility_info = fd_hashmap.get(&fd).unwrap();

        let resp_pkt = recvPacket(
            event.data, 
            &fd_utility_info.sockaddr, 
            event.events as u16, 
            &fd_utility_info.requestPacket);


        // FIXME this is a horrible fix for fragmented TCP packets
        if fd_utility_info.requestPacket.port == 22 && resp_pkt.is_ok() {
            let mut data = resp_pkt.clone().unwrap().data;
            data.dedup(); // Get rid of 0 in the buffer

            // We are not interested in partial SSH packets so just skip it until the next check
            if data.len() < 2500 { continue }
        }

        // Close the socket and remove it from the epoll_waitVec and from fd_hashmap
        let fd_utility_info = fd_hashmap.remove(&fd).unwrap();
        closeSocket(fd as u64);

        let epoll_event_index = epoll_waitVec.iter().position(|e| e.data == fd).unwrap();
        epoll_waitVec.swap_remove(epoll_event_index);

        return_vec.push((resp_pkt, fd_utility_info));
    }
    clearTimeoutedPackets(epoll_waitVec, fd_hashmap);

    return_vec
}

fn clearTimeoutedPackets(epoll_waitVec: &mut Vec<epoll_event>, fd_hashmap: &mut HashMap<u64, FdUtilityInfo>) {

    epoll_waitVec.retain(|event| {
        let fd = event.data;
        let fd_utility_info = fd_hashmap.get(&fd);

        if fd_utility_info.is_some() {
            let fd_utility_info = fd_utility_info.unwrap();

            if SystemTime::now().duration_since(fd_utility_info.sendTime).unwrap().as_millis() >= DEFAULT_TIMEOUT.into() {
                debug!("clearTimeoutedPackets, packet has timeouted, requestPacket: {}", fd_utility_info.requestPacket);
                fd_hashmap.remove(&fd).unwrap();
                closeSocket(fd as u64);
                return false;
            }
        }
        return true;
    });
}

fn addToEpoll (fd: u64, epollfd: u64, fd_utility_info: FdUtilityInfo, FdHashmap: &mut HashMap<u64, FdUtilityInfo>, epoll_waitVec: &mut Vec<epoll_event>) {
    // TODO make thrugh .map()
    FdHashmap.insert(fd, fd_utility_info);
    let event = epoll_event { events: EPOLLIN | EPOLLERR, data: fd };

    epollCtl(epollfd as u64, EPOLL_CTL_ADD as u64, fd, &event);

    epoll_waitVec.push(event);
}

pub fn loopStart() {

    /*
     * For offline testing of the rendering functionality
    let json = std::fs::read("graph.json").unwrap();
    let json2 = String::from_utf8_lossy(&json);
    let GraphRight: Graph = serde_json::from_str(&json2).unwrap(); // Deserialize from a json string
    GraphRight.renderInTerminal();
    //return;
    */

    let mut FdHashmap: HashMap<u64, FdUtilityInfo> = HashMap::new();
    let mut TraceroutesHashmap = HashMap::new();
    let mut IPsAlreadySubnetPinged: Vec<Ipv4Addr> = Vec::new();
    let mut epoll_waitVec: Vec<epoll_event> = Vec::new();

    let localhost = "127.0.0.1".parse::<Ipv4Addr>().unwrap();// todo make a local ip getting function
    let mut Graph = Graph::new(localhost);

    #[cfg(target_arch = "x86_64")]
    let epollfd = syscalls_x86_64::epoll_create1(0);
    #[cfg(target_arch = "aarch64")]
    let epollfd = syscalls_aarch64::epoll_create1(0);

    // Max of the non root processes
    setSocketLimit(4096);
    //setSocketLimit(10096);

    for (fd, sockaddr, pkt) in sendFirstDiscoveryPings() {
        let fd_utility_info = FdUtilityInfo { sockaddr: sockaddr, requestPacket: pkt, callback: PacketCallbackChoose::Traceroute, graphId: 0, index: 0, sendTime: SystemTime::now() };
        addToEpoll(fd, epollfd as u64, fd_utility_info, &mut FdHashmap, &mut epoll_waitVec);
    }

    let mut return_packets = vec![];
    let mut last_successfull_index = 0;

    debug!("Event loop started");
    loop {
        return_packets.append(&mut waitForPackets(epollfd as u64, &mut epoll_waitVec, &mut FdHashmap));
        if return_packets.len() == 0 { break };
        
        'packets_loop: for (i, (packet, fd_utility_info)) in return_packets.iter().enumerate() {
            match fd_utility_info.callback {
                // TODO I should split these callbacks in separate functions
                PacketCallbackChoose::Traceroute => {

                    let pkt = unwrapResult(packet.clone());

                    // AFAIN there should be only Ok responses, no ExceededTTL or something else
                    // but will leave the check for now.

                    if pkt.is_some() {
                        let pkt = pkt.unwrap();
                        debug!("Received packet in Traceroute: {}", pkt);

                        // Check if we already tracerouted this IP
                        if TraceroutesHashmap.get(&inet4_pton(pkt.ip)).is_none() {

                            let traceroutes;
                            match tracerouteToIP(pkt.ip, pkt.ttl) {
                                Ok(trace) => traceroutes = trace,
                                Err(LinuxError) => {
                                    println!("tracerouteToIP error: {:?}", LinuxError);
                                    break 'packets_loop;
                                }
                            }
                            // The reason is defautl icmp_rate for linux
                            std::thread::sleep(std::time::Duration::from_millis(DEFAULT_ICMP_RATE + 1000));
                            //debug!("Got this ktop_traceroute_vec, {:?}", ktop_tracerote_vec);

                            // Seems like it would be better to use only ip as a key in hashmap since
                            // we will be able to use the hashmap as a check for already scanned ips
                            // and there is no downsides seems like since IP must be unique.
                            let graphId = inet4_pton(pkt.ip);
                            let mut vector_to_hashmap = vec![Some(localhost)];

                            // For now let the ID by just IP + ttl and incrementation
                            for (i, (fd, sockaddr, pkt)) in traceroutes.iter().enumerate() {
                                let fd_utility_info = FdUtilityInfo { sockaddr: sockaddr.clone(), requestPacket: pkt.clone(), callback: PacketCallbackChoose::PingSubnet, graphId: graphId,
                                        index: i + 1, // +1 since we already have starting node as localhost
                                        sendTime: SystemTime::now()
                                };

                                addToEpoll(*fd, epollfd as u64, fd_utility_info, &mut FdHashmap, &mut epoll_waitVec);
                                // Placeholder for future packets which will be added at PingSubnet
                                vector_to_hashmap.push(None);
                            }

                            debug!("Created vector_to_hashmap: {:?}, graphId: {}, for packet, {}", vector_to_hashmap, graphId, pkt);
                            TraceroutesHashmap.insert(graphId, vector_to_hashmap);
                        }

                    }
                },
                PacketCallbackChoose::PingSubnet => {
                    // TODO make a kind of cache where I can remember which IPs I have subnetPinged
                    // already.

                    // If there is an index greater than the one we have, then it is a dup ping
                    // which happens bc I ping with a bit greater TTL than I need

                    if TraceroutesHashmap.get(&fd_utility_info.graphId).unwrap().len() > fd_utility_info.index {

                        if packet.is_ok() {
                            // We got the Ok packet hence this is the end of traceroute
                            debug!("Got Ok packet, truncating hashmap: {:?}, index: {:?}, graphId: {}", TraceroutesHashmap.get(&fd_utility_info.graphId).unwrap(), fd_utility_info.index, fd_utility_info.graphId);
                            TraceroutesHashmap.get_mut(&fd_utility_info.graphId).unwrap().truncate(fd_utility_info.index);// = Some(pkt.ip);
                            debug!("Got Ok packet, new hashmap: {:?}, index: {:?}, graphId: {}", TraceroutesHashmap.get(&fd_utility_info.graphId).unwrap(), fd_utility_info.index, fd_utility_info.graphId);
                        } 


                        // Adding packet to hashmap
                        let pkt = unwrapResult(packet.clone());

                        if pkt.is_some() {
                            let pkt = pkt.unwrap();
                            debug!("Received packet in PingSubnet: {}, its graphPath: {:?}, index {:?}, graphId: {}", pkt, TraceroutesHashmap.get(&fd_utility_info.graphId), &fd_utility_info.index, &fd_utility_info.graphId);

                            // We dont want to deal with external IP ranges
                            if pkt.ip.is_private() {

                                // First calculate neibors
                                let neibors = calculateNeibors(pkt.ip);
                                debug!("Got neibors: {:?}, from ip: {:?}", neibors, pkt.ip);
                                for ip in neibors {
                                    // Check for already tracerouted IPs and traceroute heibors if they are not
                                    // tracerouted yet
                                    if TraceroutesHashmap.get(&inet4_pton(ip)).is_none() {
                                        //debug!("Sending this packet to this IP for it is a neibor and wasnt tracerouted yet: {:?}", ip);

                                        // Send ping and put to event loop
                                        let simple_ping_pkt = Packet { protocol: ICMP, ip: ip, ttl: 64, port: 0, data: from_hex(ICMP_DATA) };
                                        match sendPacket(&simple_ping_pkt) {
                                            Ok((fd, sockaddr)) => {
                                                let fd_utility_info = FdUtilityInfo { sockaddr: sockaddr, requestPacket: simple_ping_pkt, callback: PacketCallbackChoose::Traceroute, graphId: 0, index: 0, sendTime: SystemTime::now() };
                                                addToEpoll(fd, epollfd as u64, fd_utility_info, &mut FdHashmap, &mut epoll_waitVec);
                                            },
                                            Err(LinuxError) => {
                                                println!("LinuxError at sending ping to neibor: {:?}", LinuxError);
                                                unimplemented!()
                                            }, 
                                        }
                                    }
                                }


                                // Making subnet ping itself
                                if !IPsAlreadySubnetPinged.contains(&pkt.ip) {
                                    debug!("Making subnet ping for ip: {:?}", pkt.ip);
                                    IPsAlreadySubnetPinged.push(pkt.ip);

                                    for (fd, sockaddr, subnetPing_pkt) in pingSubnet(pkt.clone().ip, 3) {

                                        let fd_utility_info_tmp = FdUtilityInfo { sockaddr: sockaddr, 
                                            requestPacket: subnetPing_pkt, 
                                            callback: PacketCallbackChoose::PostDiscovery, 
                                            graphId: fd_utility_info.graphId, 
                                            index: fd_utility_info.index,
                                            sendTime: SystemTime::now()
                                        };

                                        addToEpoll(fd, epollfd as u64, fd_utility_info_tmp, &mut FdHashmap, &mut epoll_waitVec);
                                    }
                                }

                                // First things first, If this is an error from Traceroute, than we should
                                // put this IP in traceroute path
                                // FIXME I belive I need to place if statesments better in this function
                                // Dont allow destination go to graphPath, destination is added at
                                // fingerprint stage

                                // This check here since I truncated the graphPath above and dont want the
                                // destination to be in graphPath, so I allow in only non ok (yeah, I need
                                // to rethink this if switches here)
                                if TraceroutesHashmap.get(&fd_utility_info.graphId).unwrap().len() > fd_utility_info.index {
                                    // Finaly, update the TraceroutesHashmap with a received packet
                                    TraceroutesHashmap.get_mut(&fd_utility_info.graphId).unwrap()[fd_utility_info.index] = Some(pkt.ip);
                                }
                            }

                        } else {
                            // In this case, there is no packet here to insert inside graphPath
                            //debug!("Got error pkt in PingSubnet from traceroute, putting 0.0.0.0 there graphId: {:?}, index: {:?}", packet_and_callback_graph.1.graphId, packet_and_callback_graph.1.index);
                            //debug!("Trying to get hashmap: {:?}, index: {:?}", TraceroutesHashmap.get(&packet_and_callback_graph.1.graphId).unwrap(), packet_and_callback_graph.1.index);
                            TraceroutesHashmap.get_mut(&fd_utility_info.graphId).unwrap()[fd_utility_info.index] = Some("0.0.0.0".parse::<Ipv4Addr>().unwrap());
                        }

                    }

                },
                PacketCallbackChoose::PostDiscovery => {
                    // In that step we just send the fingerprint packets

                    // We are not interested in error packets here, no traceroutes will be done here
                    if packet.is_ok() {
                        let pkt = packet.clone().unwrap();

                        debug!("Received packet in PostDiscovery: {}", pkt);

                        // Graph is populated, we can go and request fingerprint and update the Graph
                        // Traceroute nodes are most likely Linux, but I should fingerprint
                        // endnodes too probably.
                        // TODO

                        if isLinuxTTL(pkt.ttl) {
                            for (fd, sockaddr, fingerprint_pkt) in sendFingerprintLinux(pkt.ip) {

                                let fd_utility_info_tmp = FdUtilityInfo { sockaddr: sockaddr, 
                                    requestPacket: fingerprint_pkt, 
                                    callback: PacketCallbackChoose::Fingerprint, 
                                    graphId: fd_utility_info.graphId, 
                                    index: fd_utility_info.index,
                                    sendTime: SystemTime::now()
                                };
                                addToEpoll(fd, epollfd as u64, fd_utility_info_tmp, &mut FdHashmap, &mut epoll_waitVec);
                            }
                        } // TODO else { sendFingerprintWindows }

                        let mut graph_path = vec![];
                        for (i, ip_op) in TraceroutesHashmap.get(&fd_utility_info.graphId).unwrap().iter().enumerate() {
                            // This check here to prevent full graphPath for nodes which are
                            // actually in the middle of graphPath
                            // NOTE, this case should handle the *findAndAddFingerprintOrIp*
                            // function however, but wont hurt to check twice for now.
                            // or actually no, seems like if the path is used as path, the path
                            // will be added and there might be no checks whatsoever
                            if i < fd_utility_info.index {
                                if ip_op.is_some() {
                                    graph_path.push(ip_op.unwrap());
                                } else {
                                    graph_path.push("0.0.0.0".parse::<Ipv4Addr>().unwrap());
                                }
                            }
                        }


                        debug!("Going to add to graph: {:?}, {}", graph_path, pkt);
                        Graph.populateGraph(&graph_path, pkt.clone(), None);

                    }

                },
                PacketCallbackChoose::Fingerprint => {
                    //debug!("Fingerprint: {packet_and_callback_graph:?}");

                    // Do I need to free TraceroutesHashmap here? Seems like yes, I will no
                    // longer need it.

                    // FIXME code duplication of this thing contructing graph_path twice and it
                    // does the same thing and output will be the same. I whould store it
                    // somewhere.
                    let mut graph_path = vec![];
                    for (i, ip_op) in TraceroutesHashmap.get(&fd_utility_info.graphId).unwrap().iter().enumerate() {
                        if i < fd_utility_info.index {
                            if ip_op.is_some() {
                                graph_path.push(ip_op.unwrap());
                            } else {
                                graph_path.push("0.0.0.0".parse::<Ipv4Addr>().unwrap());
                            }
                        }
                    }


                    // We are not interested in error packets here, no traceroutes will be done here
                    if packet.is_ok() {
                        let pkt = packet.clone().unwrap();
                        debug!("Received packet in Fingerprint: {}", pkt);
                        
                        /*
                            There has to be 2 checks:
                            1. Is there already a machine with presented fingerprint?
                            2. Is there already a machine with presented IP?

                            If yes, update it (add new IP or new fingerprint to existing node) and dont add a new node. If no, otherwise.
                        */

                        let fingerprint = recvFingerprintLinux(&pkt);
                        // If we didnt find the graph node to the fingerprint, then add a new node
                        if !Graph.findAndAddFingerprintOrIp(&graph_path, &pkt, fingerprint.clone()) {
                            debug!("Going to add to graph: {:?}, {}, {:?}", graph_path, pkt, fingerprint);
                            // TODO pass fingerprint here to add it right away
                            Graph.populateGraph(&graph_path, pkt.clone(), Some(fingerprint));
                        }


                    }
                }
            }
            last_successfull_index = i;
        }

        // Clear the successfully handled packets
        if last_successfull_index + 1 == return_packets.len() { return_packets.clear() }
        else if last_successfull_index > 0 { return_packets.drain(0..last_successfull_index); }
        last_successfull_index = 0;
    }
    // TODO make a save to json for user use
    //let json: String = serde_json::to_string(&Graph).unwrap(); // serialize to a JSON string 
    //let out = std::fs::write("graph.json", json);
    //let new: Super = serde_json::from_str(&json).unwrap(); // Deserialize from a json string

    Graph.renderInTerminal();
}

// Purely debugging
/*
fn getOpenedFds() {
    let mut path = "/proc/".to_string();
    path.push_str(&std::process::id().to_string());
    path.push_str("/fd/");
    let paths = std::fs::read_dir(path).unwrap();

    println!("opened fds: {:?}", paths.count());
}
*/
