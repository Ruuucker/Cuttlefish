"use strict";

const xmlConvert = require('xml-js');
const fs = require('fs');

function convertNmapOutput (dirToSave, xmlPath, fileName) {
	// Импорт и преобразование
	let nmapOutputXml = fs.readFileSync(xmlPath, {encoding:'utf8', flag:'r'});
	let nmapOutputJSON = xmlConvert.xml2json(nmapOutputXml, {compact: true, spaces: 0});
	
	// Запись JSON
	let wholePath = dirToSave + fileName + '.json';
	fs.writeFileSync(wholePath, nmapOutputJSON);
	// nmapOutputJSON = JSON.parse(nmapOutputJSON);
	return [wholePath, nmapOutputJSON];
}

function generateFileName () {
	let randomNumber = getRandomInt (10000, 20000);
	
	// Работает только на лине, ну и норм
	return randomNumber + '_octopus';
}

function getRandomInt(min, max) {
	min = Math.ceil(min);
	max = Math.floor(max);
	return Math.floor(Math.random() * (max - min) + min); //The maximum is exclusive and the minimum is inclusive
}

function convertIpIntoSubnet (ip) {
	let tmpIp = ip.slice(0, ip.lastIndexOf('.') + 1);
	tmpIp = tmpIp + '0/24';
	return tmpIp;
}

module.exports = {
	generateFileName: generateFileName,
    convertNmapOutput: convertNmapOutput,
    convertIpIntoSubnet: convertIpIntoSubnet
}
