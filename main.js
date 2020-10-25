const { generateFileName, convertNmapOutput } = require('./utils.js');
const { startScan } = require('./scan.js');
const { startServer } = require('./server.js');

var dirToSave = '/tmp/';
var diaps = ['192.168.1.0/24', '192.168.1.1'];
// var diaps = ['192.168.1.1'];

var scansPromises = [];
for (let i = diaps.length - 1; i >= 0; i--) {
	let fileName = generateFileName();
	scansPromises.push(startScan(diaps[i], dirToSave, fileName));
}

Promise.all(scansPromises).then((xmlPaths) => {
	console.log(xmlPaths);

	let jsonPaths = [];

	for (let i = xmlPaths.length - 1; i >= 0; i--) {
		let fileName = generateFileName();
		jsonPaths.push(convertNmapOutput(dirToSave, xmlPaths[i], fileName));
	}
	console.log(jsonPaths);
	startServer(jsonPaths);
});