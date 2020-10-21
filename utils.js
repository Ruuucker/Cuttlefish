const xmlConvert = require('xml-js');
const fs = require('fs');


function convertNmapOutput (fileName) {
	// Импорт и преобразование
	let nmapOutputXml = fs.readFileSync(`/temp/${fileName}.xml`, {encoding:'utf8', flag:'r'});
	let nmapOutputJSON = xmlConvert.xml2json(nmapOutputXml, {compact: true, spaces: 0});
	
	// Запись JSON
	fs.writeFileSync(`/temp/${fileName}.json`, nmapOutputJSON);
	// nmapOutputJSON = JSON.parse(nmapOutputJSON);
	return;
}

function generateFileName () {
	let randomNumber = getRandomInt (10000, 20000);
	return randomNumber + '_octopus';
}

function getRandomInt(min, max) {
	min = Math.ceil(min);
	max = Math.floor(max);
	return Math.floor(Math.random() * (max - min) + min); //The maximum is exclusive and the minimum is inclusive
}

module.exports = {
	generateFileName: generateFileName,
	convertNmapOutput: convertNmapOutput
}