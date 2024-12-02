use std::net::{ Ipv4Addr };
use crate::netlib::{ debug, Packet };

use crate::cuttlefish::host_fingerprinting::{ RespFingerprintLinux, TypeFingerprintLinux::* };
use std::env;


#[derive(serde_derive::Deserialize, serde_derive::Serialize)]
pub struct Graph {
    nodes: Vec<Machine>
}

#[derive(serde_derive::Deserialize, serde_derive::Serialize, Debug, Clone, PartialEq)]
struct Machine {
    pub ttl: u8,
    pub ips: Vec<Ipv4Addr>,
    pub subnodes: Vec<Machine>,
    pub macs: Vec<String>,
    pub snmp_engineId: Vec<u8>,
    pub ssh_pubkey: Vec<u8>,
}

impl Graph {
    pub fn new (localhost: Ipv4Addr) -> Graph {
        Graph {
            nodes: vec![ Machine {
                ttl: 64,
                ips: vec![ localhost ],
                subnodes: Vec::new(),
                macs: vec![], 
                snmp_engineId: Vec::new(), ssh_pubkey: Vec::new(),
                // graphId: 0,
            }]
        }
    }


    pub fn populateGraph(&mut self, relativeRootNodes: &Vec<Ipv4Addr>, newSubnode: Packet, fingerprnt: Option<RespFingerprintLinux>) {
        // TODO refactor, there should be no fingerprint I think, it just better to be updated
        // TODO make a check for a path with re

        // Walking the path to the target node
        let mut Root = &mut self.nodes;
        for relativeNode in relativeRootNodes {

            // No need for now, but maybe better to be able to push nodes in the root here
            /*
            if !Root.contains(relativeNode) {
                Root.push(relativeNode);
            }
            else {
            */

            debug!{"Walking down the Graph nodes: {relativeNode:?}"};

            let mut found_relativeNode = false;
            // The first line on nodes
            'inner: for (i, node) in &mut Root.iter().enumerate() {

                if node.ips.contains(relativeNode) {
                    debug!{"Path found!: {relativeNode:?}"};
                    Root = &mut Root[i].subnodes;
                    found_relativeNode = true;
                    break 'inner;
                }

            }
            // TODO make put vec<Packet> here as relativeRootNodes
            // Check for duplicate
            if !found_relativeNode {
                debug!{"Adding: {relativeNode:?}"};
                Root.push(Machine { ttl: 64, ips: vec![*relativeNode], subnodes: Vec::new(), macs: Vec::new(), snmp_engineId: Vec::new(), ssh_pubkey: Vec::new() });
                let len = Root.len();
                Root = &mut Root[len - 1].subnodes;
            }
        }

        // I dont like this check for dups, need to use something better, maybe check in
        // previous loop.
        for sub in &mut *Root {
            if sub.ips.contains(&newSubnode.ip) {
                // Duplicate
                return;
            }
        }

        match fingerprnt {
            Some(fngprt) => {
                match fngprt.fingerprintType {
                    Ssh => Root.push(Machine { ttl: newSubnode.ttl, ips: vec![newSubnode.ip], subnodes: Vec::new(), macs: Vec::new(), snmp_engineId: Vec::new(), ssh_pubkey: fngprt.value }),
                    Snmp => Root.push(Machine { ttl: newSubnode.ttl, ips: vec![newSubnode.ip], subnodes: Vec::new(), macs: Vec::new(), snmp_engineId: fngprt.value, ssh_pubkey: Vec::new() })
                }
            },
            None => Root.push(Machine { ttl: newSubnode.ttl, ips: vec![newSubnode.ip], subnodes: Vec::new(), macs: Vec::new(), snmp_engineId: Vec::new(), ssh_pubkey: Vec::new() })
        }
    }

    pub fn findAndAddFingerprintOrIp(&mut self, relativeRootNodes: &Vec<Ipv4Addr>, pkt: &Packet, fingerprint: RespFingerprintLinux) -> bool {
        // TODO Maybe I shoud put Machine instead of Ipv4Addr
        debug!("findAndAddFingerprintOrIp, relativeRootNodes: {relativeRootNodes:?}, packet: {pkt}, fingerprint: {fingerprint:?}");

        // FIXME for now I use path to go backwards, maybe I should change it to something else

        let mut Root = &mut self.nodes;

        // Append our main search packet for relativeRootNodes for it to found itself but ONLY for findAndAddFingerprintOrIp
        let mut local_relativeRootNodes = relativeRootNodes.clone();
        local_relativeRootNodes.push(pkt.ip.clone());

        for relativeNode in &local_relativeRootNodes {
            debug!{"Walking down the Graph nodes: {relativeNode:?}"};

            // The first line on nodes
            'inner: for (i, node) in &mut Root.iter_mut().enumerate() {

                // First check IP, than check fingerprint, this way I elliminate double IPs in *node.ips*
                debug!{"IPs which I will check if the same: node.ips: {:?}, ip: {:?}", node.ips, pkt.ip };

                if node.ips.contains(&pkt.ip) {
                    debug!("This is the node which I will update (same ip): {node:?}");
                    match fingerprint.fingerprintType {
                        Ssh => node.ssh_pubkey = fingerprint.value,
                        Snmp => node.snmp_engineId = fingerprint.value,
                    }

                    return true;
                }

                if (node.snmp_engineId == fingerprint.value) | (node.ssh_pubkey == fingerprint.value) {
                    debug!("This is the node which I will update (same fingerprint): {node:?}");
                    node.ips.push(pkt.ip);

                    return true;
                }

                if node.ips.contains(relativeNode) {
                    debug!{"Path found!: {relativeNode:?}"};
                    Root = &mut Root[i].subnodes;
                    break 'inner;
                }
            }
        }


        // FIXME root clone is not ideal
        // Its not required at the moment
        /*
        debug!("Current root: {:?}", Root);
        for (i_sub, _node) in Root.clone().iter().enumerate() {
            //debug!("This is the node upper: {:?}", _node);

            for mut node in Root[i_sub].subnodes.iter_mut() {
                //debug!("This is the node below: {:?}", node);

                if node.ips.contains(&pkt.ip) {
                    debug!("This is the node which I will update (same ip): {node:?}");
                    match fingerprint.fingerprintType {
                        Ssh => node.ssh_pubkey = fingerprint.value,
                        Snmp => node.snmp_engineId = fingerprint.value,
                    }

                    return true;
                }

                if (node.snmp_engineId == fingerprint.value) | (node.ssh_pubkey == fingerprint.value) {
                    debug!("This is the node which I will update (same fingerprint): {node:?}");
                    node.ips.push(pkt.ip);

                    return true;
                }
            }
        }
        */

        return false;
    }

    pub fn renderInTerminal(&self) {
        let mut linesToPrint = vec![vec![]];

        // Render our machine in the node, special case for simplicity
        nodesToPrint(&self.nodes, &mut linesToPrint, 0);

        // debug!("renderInTerminal, linesToPrint: {:?}", linesToPrint);
        // TODO change height and wight
        let mut flag = true;
        let mut height = 0;
        while flag {
            flag = false;
            for wight in 0..=linesToPrint.len() - 1 {
                if height <= linesToPrint[wight].len() - 1 {

                    // Before each node gets printed, it should be aligned
                    alignPrintingNode(&mut linesToPrint[wight][height], 17);
                    print!("{}", linesToPrint[wight][height]);
                    flag = true;
                } else {
                    print!("{}", String::from(" ").repeat(17));
                }
            }
            height += 1;
            print!("\n");
        }
    }

    /*
    pub fn exportJSON(&self) {
        
    }
    */

}

// TODO:
// 1. arrows
// 2. duplicate checks (check ips for depth next node and NOT add it as your subnode)
fn nodesToPrint (nodes: &Vec<Machine>, printList: &mut Vec<Vec<String>>, x: usize) {

    for (_i, node) in nodes.iter().enumerate() {

        debug!("==========");
        debug!("x:{}", x);
        debug!("printList {:#?}", printList);
        debug!("node {:?}", node.ips);
        

        if !node.subnodes.is_empty() {
            debug!("Going inside");
            if x + 1 >= printList.len() { printList.push(vec![]) };

            // Render everything what we have of this node
            printSingleNode(&node, x, printList);

            // Adding the start of the arrow
            debug!("Adding the start");
            let last_current_y = printList[x].len() - 1;
            debug!("last_current_y {last_current_y}");
            printList[x][last_current_y].push_str("--");

            // Before going inside we should draw a line upwards of downward the next avaliable
            // space, there will be our next node
            renderTheArrow(x, last_current_y, printList);

            // Adding the end of the arrow, pointing down on the node
            printList[x + 1].push("--v".to_string());

           nodesToPrint(&node.subnodes, printList, x + 1);

        } else {
            printSingleNode(&node, x, printList);
        }

    }
}

fn printSingleNode(node: &Machine, x: usize, printList: &mut Vec<Vec<String>>) {
    // Print metadata of the node, based on information provided inside the node itself
    //debug!("x:{x}, y:{:?}", printList[x].len());
    printList[x].push("=Linux=".to_string());

    for ip in &node.ips {
        printList[x].push(ip.to_string());
    }
}

fn alignPrintingNode (node: &mut String, fixed_length: usize) {
    // Fixed string lenght == 16
    //debug!("alignPrintingNode: node: {:?}, fixed_length: {fixed_length}, node.len(): {:?}", node, node.len());
    match node.chars().last().unwrap() {
        '|' => {
            let pipe_char = node.pop().unwrap();
            alignPrintingNode(node, fixed_length - 1);
            node.push(pipe_char);
        },
        '-' => {
            for _i in 0..=fixed_length - node.len() - 1 {
                // TODO colors?
                //node.push_str("\x1b[93m-\x1b[0m");
                node.push_str("-");
            }
        },
        _ => {
            for _i in 0..=fixed_length - node.len() - 1 {
                node.push_str(" ");
            }
        },
    } 

}

// TODO would be great to make arrow bodies colored so the user wont miss where is the turn
fn renderTheArrow(x: usize, last_y: usize, printList: &mut Vec<Vec<String>>) {
    let next_y = printList[x + 1].len(); // Getting the last avaliable y of the next collumn
    debug!("current node: {:?}", printList[x][last_y]);
    debug!("start x:{x}, last y:{last_y}, next_y: {next_y}");
    //println!("\x1b[93mError\x1b[0m");
    if next_y >= last_y {
        for y in last_y..next_y {
            printList[x + 1][y].insert_str(0, "|");
        }

    } else {
        //debug!("path_up");
        for y in next_y..last_y {
            printList[x][y].push_str("|");
        }
    }
}

// TODO make this happen so the nodes which actually the ips of the nodes deeper in the graph will
// be removed
/*
fn clearNodesWhoSubnodes(nodes: &mut Vec<Machine>) {
    let mut ips_to_check: Vec<Ipv4Addr> = Vec::new();
    for (i, node) in nodes.iter().enumerate() {

        // If subnodes, get their IP and check them against all nodes in current node
        if !node.subnodes.is_empty() {
            // Get the ips
            ips_to_check.extend(&node.ips.clone());
            
        } else {
            // Check if we have node with ips_to_check, delete the node if needed
            for ip in ips_to_check {
                // For now we focus on one ip nodes only
                if ip == node.ips[0] {
                    //let index = nodes.iter().position(|x| *x == node.clone()).unwrap();
                    nodes.remove(i);
                }
            }
        }
    }

    // We want to do that for every node so go to recursion
    for (_i, node) in nodes.iter().enumerate() {
        if !node.subnodes.is_empty() {
            clearNodesWhoSubnodes(&mut node.subnodes);
        }
    }
}
*/
