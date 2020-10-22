const { generateFileName, convertNmapOutput } = require('./utils.js');
const { startScan } = require('./scan.js');
const { startServer } = require('./server.js');

var dirToSave = '/tmp/';
var fileName = generateFileName();
var diap = '192.168.1.0/24';

startScan(diap, dirToSave, fileName)
	.then(xmlPath => {
		let jsonPath = convertNmapOutput(dirToSave, xmlPath, fileName);
		startServer(jsonPath);
		console.log('Иди и смотри');
	});

