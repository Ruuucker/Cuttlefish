"use strict";

const { generateFileName, convertNmapOutput, convertIpIntoSubnet } = require('./utils.js');
const { startScan, checkIP } = require('./scan.js');
const { startServer } = require('./server.js');

var dirToSave = '/tmp/';
var diaps = [];

// Здесь будут ip которые должы быть доступны если есть такой диапазон, тоесть 192.168.1.1, 192.168.0.254, что то такое, еще паработаю над состовлением списка
var ipSubnets = ['172.31.31.11', '192.168.1.1', '192.168.0.254', '192.168.221.254'];

// Check if we have internet connetion
checkIP('8.8.8.8').then((tmpInfo) => {
	if (tmpInfo[1])
		console.log('We have internet connetion!');
});

// #####################
// ### Subnet checks ###
// #####################

var checkPromises = [];
for (var addrIndex = ipSubnets.length - 1; addrIndex >= 0; addrIndex--) {
	checkPromises.push(checkIP(ipSubnets[addrIndex]));
}

Promise.all(checkPromises).then((allIsHostUpInfo) => {
	for (let i = allIsHostUpInfo.length - 1; i >= 0; i--) {
		// Resolve cannot process 2 varibables per time (or I just dont know how to do it) so I return massive
		// allIsHostUpInfo[0] - ip addres, allIsHostUpInfo[1] - is host up
		let ipAddr = allIsHostUpInfo[i][0];
		let isHostUp = allIsHostUpInfo[i][1];

		console.log(ipAddr, isHostUp);
		if (isHostUp)
			diaps.push(convertIpIntoSubnet(ipAddr));
	}

	// Пока что оставлю эту штуку здесь, в дальнейшем её можно вызывать из функции, для удобства и красоты
	var scansPromises = [];
	for (let i = diaps.length - 1; i >= 0; i--) {
		let fileName = generateFileName();
		scansPromises.push(startScan(diaps[i], dirToSave, fileName));
	}

	Promise.all(scansPromises).then((xmlPaths) => {
		// console.log(xmlPaths);

		let jsonPaths = [];

		for (let i = xmlPaths.length - 1; i >= 0; i--) {
			let fileName = generateFileName();
			jsonPaths.push(convertNmapOutput(dirToSave, xmlPaths[i], fileName));
		}
		startServer(jsonPaths);
	});
});
