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
			console.log(`stdout: ${stdout}`);
			resolve (wholePath);
		});
	});	
}

module.exports = {
	startScan: startScan
}