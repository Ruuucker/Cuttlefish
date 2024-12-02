use std::net::{ Ipv4Addr };//, IpAddr };
use crate::netlib::*;

const SNMP_DATA: &str = "303a020103300f02024a69020300ffe30401040201030410300e0400020100020100040004000400301204000400a00c020237f00201000201003000";

pub fn fingerprint (ip: Ipv4Addr) -> (u64, sockaddr, Packet) {
    let send_pkt = Packet { protocol: IPPROTO_UDP, ip: ip, ttl: 64, port: 161, data: from_hex(SNMP_DATA) };
    match sendPacket(&send_pkt) {
        Ok((fd, sockaddr)) => { 
            (fd, sockaddr, send_pkt)
        },
        Err(LinuxError) => {
            println!("LinuxError at snmp::fingerprint: {:?}", LinuxError);
            unimplemented!();
        }, 
    }
}

pub fn parseEngineId (data: &Vec<u8>) -> Vec<u8> {
    (&data[28..45]).to_vec()
}
