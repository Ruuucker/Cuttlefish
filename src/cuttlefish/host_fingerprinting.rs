use std::net::{ Ipv4Addr };
use crate::netlib::*;//{ debug, Packet };
use std::env;
mod ssh;
mod snmp;


#[derive(Debug, Clone)]
pub enum TypeFingerprintLinux {
    Ssh,
    Snmp
}

#[derive(Debug, Clone)]
pub struct RespFingerprintLinux {
    pub fingerprintType: TypeFingerprintLinux,
    pub value: Vec<u8>,
}

/*
 * Maybe do 
pub fn sendFingerprintLinux(ip: &Ipv4Addr) -> {
    // SSH
    let ssh_pkt = Packet {
        protocol: TCP,
        ip: *ip.clone(),
        ttl: 64,
        port: 22,
        data: SSH_FINGERPRINT_DATA
    };

    // SNMP
    let snmp_pkt = Packet {
        protocol: UDP,
        ip: *ip.clone(),
        ttl: 64,
        port: ,
        data: SNMP_FINGERPRINT_DATA
    };
}
*/

// Generic function to get every fingerprint avaliable for Linux
pub fn sendFingerprintLinux(ip: Ipv4Addr) -> Vec<(u64, sockaddr, Packet)> {
    debug!("sendFingerprintLinux: {ip:?}");

    let mut retvec: Vec<(u64, sockaddr, Packet)> = Vec::new();

    let ssh_keyscan = ssh::keyscan(ip); 
    let snmp_fingerprint = snmp::fingerprint(ip);

    // Ignore errors
    if ssh_keyscan.is_ok() {
        retvec.push(ssh_keyscan.unwrap());
    }
    //if snmp_fingerprint.is_ok() {
    retvec.push(snmp_fingerprint);
    //}

    debug!("sendFingerprintLinux, return: {retvec:?}");
    retvec
}

// Used for extracting fingerprint data out of response packet and send it to updateDataForMachine
pub fn recvFingerprintLinux(pkt: &Packet) -> RespFingerprintLinux {
    debug!("recvFingerprintLinux {pkt}");

    if pkt.protocol == IPPROTO_TCP && pkt.port == 22 {
        RespFingerprintLinux { fingerprintType: TypeFingerprintLinux::Ssh, value: ssh::parseKeyscan(&pkt.data) }

    } else if pkt.protocol == IPPROTO_UDP && pkt.port == 161 {
        RespFingerprintLinux { fingerprintType: TypeFingerprintLinux::Snmp, value: snmp::parseEngineId(&pkt.data) }
    } else {
        unimplemented!();
    }
}
