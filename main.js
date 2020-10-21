const { generateFileName, convertNmapOutput } = require('./utils.js');
const { startScan } = require('./scan.js');
const { startServer } = require('./server.js');

var fileNameIn = generateFileName();

startScan('192.168.1.0/24', fileNameIn)
	.then(fileNameOut => {
		convertNmapOutput(fileNameOut);
		startServer(fileNameOut);
	});
