"use strict";

const { generateFileName, convertNmapOutput} = require('./utils.js');
const { startScan, checkIP } = require('./scan.js');
const { startServer } = require('./server.js');

var dirToSave = '/tmp/';

// Посмотрю какие IP точно должны существовать в этих конкретных подсетках
// Коли таких нет - сканирую всё ИЛИ беру диапазоны из трейсроутов к 8.8.8.8, неизвестным IP и такие вот вещи

/*      Диапазоны внутренних IP от RFC 1918
 *
 *      10.0.0.0        -   10.255.255.255  (10/8 prefix)
 *      172.16.0.0      -   172.31.255.255  (172.16/12 prefix)
 *      192.168.0.0     -   192.168.255.255 (192.168/16 prefix)
 *
 */

// Ссылки которые мне нужно проверить для быстрого скана
// https://stackoverflow.com/questions/14038606/fastest-way-to-ping-a-network-range-and-return-responsive-hosts
// https://serverfault.com/questions/665311/fastest-way-to-scan-all-hosts-that-are-online
// Экспериментальным путём было выяснено что подобный формат комманды: sudo nmap -sn -T5 --min-parallelism 100 --max-parallelism 256 192.168.0.0/24 самый быстрый

var internalSubnets = ['172.16.0.0/24', '192.168.0.0/16', '10.0.0.0/8'];

// Check if we have internet connetion
checkIP('8.8.8.8').then((tmpInfo) => {
	if (tmpInfo[1])
		console.log('We have internet connetion!');
});

/* #####################
*  #####################
*  ### Subnet checks ###
*  #####################
*  #####################
*/

var scansPromises = [];
for (let i = internalSubnets.length - 1; i >= 0; i--) {
    let fileName = generateFileName();
        scansPromises.push(startScan(internalSubnets[i], dirToSave, fileName));
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
