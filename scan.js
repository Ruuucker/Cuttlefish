"use strict";

const { exec } = require('child_process');

function startScan (addr, dirToSave, fileName) {

	return new Promise ((resolve, reject) => {
		let wholePath = dirToSave + fileName + '.xml';
		exec(`sudo nmap -sn --traceroute ${addr} -oX ${wholePath}`, (error, stdout, stderr) => {
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

module.exports = {
	startScan: startScan,
	checkIP: checkIP
}