const { exec } = require('child_process');

async function startScan (addr, fileName) {
	exec(`sudo nmap localhost -sn --traceroute ${addr} -oX /tmp/${fileName}.xml`, (error, stdout, stderr) => {
		if (error) {
			console.error(`exec error: ${error}`);
			return;
		}
		console.log(`stdout: ${stdout}`);
		return fileName;
	});
}

module.exports = {
	startScan: startScan
}