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
        Список зарезервированных мультикаст адресов
        
        Адрес   Значение
        224.0.0.0   Не используется
        224.0.0.1   Все узлы данного сегмента
        224.0.0.2   Все мультикастовые узлы данного сегмента
        224.0.0.4   Данный адрес выделялся для покойного протокола DVMRP
        224.0.0.5   Все OSPF-маршрутизаторы сегмента
        224.0.0.6   Все DR маршрутизаторы сегмента
        224.0.0.9   Все RIPv2-маршрутизаторы сегмента
        224.0.0.10  Все EIGRP-маршрутизаторы сегмента
        224.0.0.13  Все PIM-маршрутизаторы сегмента
        224.0.0.18  Все VRRP-маршрутизаторы сегмента
        224.0.0.19-21   Все IS-IS-маршрутизаторы сегмента
        224.0.0.22  Все IGMP-маршрутизаторы сегмента (v2 и v3)
        224.0.0.102 Все HSRPv2/GLBP-маршрутизаторы сегмента
        224.0.0.107 PTPv2 — Precision Time Protocol
        224.0.0.251 mDNS
        224.0.0.252 LLMNR
        224.0.0.253 Teredo
        224.0.1.1   NTP
        224.0.1.39  Cisco Auto-RP-Announce
        224.0.1.40  Cisco Auto-RP-Discovery
        224.0.1.41  H.323 Gatekeeper
        224.0.1.129-132 PTPv1/PTPv2
        239.255.255.250 SSDP


Диапазон 224.0.0.0/24 зарезервирован под link-local коммуникации. Мультикастовые пакеты с такими адресами назначения не могут выходить за пределы одного широковещательного сегмента.
Диапазон 224.0.1.0/24 зарезервирован под протоколы, которым необходимо передавать мультикаст по всей сети, то есть проходить через маршрутизаторы.


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
