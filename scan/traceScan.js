"use strict";

const { exec } = require('child_process');
const { convertNmapOutput } = require('../utils.js');

function traceScan (addr, dirToSave, fileName) {

	return new Promise ((resolve, reject) => {
		let wholePath = dirToSave + fileName + '.xml';
		exec(`sudo nmap -T5 --min-parallelism 100 --max-parallelism 256 -sn --traceroute ${addr} -oX ${wholePath}`, (error, stdout, stderr) => {
			if (error) {
				console.error(`exec error: ${error}`);
				return;
			}
			// Just for logs
			// console.log(`stdout: ${stdout}`);
			resolve (wholePath);
		});
	});	
}

function checkIP (addr) {

    return new Promise ((resolve, reject) => {
        exec(`sudo nmap -sn ${addr}`, (error, stdout, stderr) => {
            if (error) {
                console.error(`exec error: ${error}`);
                return;
            }
    
            let tmpString = stdout.slice(stdout.lastIndexOf('('), stdout.lastIndexOf(')'));
            let isHostUp = tmpString.includes('1');
            let infoTmp = [addr, isHostUp];
    
            // Resolve cannot process 2 varibables per time (or I just dont know how to do it) so I return massive
            resolve (infoTmp);
        });
    }); 
}

function getFirstIPs (dirToSave, fileName) {

    return new Promise ((resolve, reject) => {
        traceScan('8.8.8.8', dirToSave, fileName).then((xmlPath) => {
            let tmpAr = convertNmapOutput(dirToSave, xmlPath, fileName);
            let json = JSON.parse(tmpAr[1]);
            
            let allIPs = json.nmaprun.host.trace.hop;
            let needeIPs = []; 
            
            for (let i = 0; i < allIPs.length; i++) {
                let tmpIP = allIPs[i]._attributes.ipaddr;
                let tmpDns = allIPs[i]._attributes.host;

                if (tmpIP == 'something' | typeof tmpDns != 'undefined') // TODO Тут должен быть IP на котором работа цикла прерывается и таким образом мы не выйдем за пределы внутренней сетки
                    break;

                needeIPs.push(tmpIP);
            }

            resolve (needeIPs);
        });
    });
}

module.exports = {
	traceScan:  traceScan,
    checkIP:    checkIP,
    getFirstIPs: getFirstIPs
}
