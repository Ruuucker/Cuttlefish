const xmlConvert = require('xml-js');
const fs = require('fs');

var createGraph = require('ngraph.graph');
var g = createGraph();

g.addNode('hello');
g.addNode('world');
g.addLink('hello', 'world');

var asciiGraphics = require('ngraph.ascii').graphics(g);
asciiGraphics.run();
// const viva = require('vivagraphjs');
// var graph = viva.Graph.graph();
// graph.addLink(1, 2);
 
// var renderer = viva.Graph.View.renderer(graph);
// renderer.run();

// // Импорт и преобразование
// var nmapOutputXml = fs.readFileSync('./test3.xml', {encoding:'utf8', flag:'r'});
// var nmapOutputJSON = xmlConvert.xml2json(nmapOutputXml, {compact: true, spaces: 0});
// nmapOutputJSON = JSON.parse(nmapOutputJSON);


// // console.log(nmapOutputJSON.nmaprun.host.trace.hop);




// if (typeof nmapOutputJSON.nmaprun.host.length == 'undefined') {
// 	console.log('this is 1 host');
// }
// else {
// 	for (let i = 0; i < nmapOutputJSON.nmaprun.host.length; i++) {
// 		// console.log(nmapOutputJSON.nmaprun.host[i]);
// 		// nmapOutputJSON[i]
// 		let host = nmapOutputJSON.nmaprun.host[i];

// 		console.log('NEW HOST');
// 		console.log(host);
// 		for (let y = 0; y < host.trace.hop.length; y++) {
// 			let hop = host.trace.hop[y]._attributes;
// 			console.log('NEW HOP');
// 			console.log(hop);
// 			// Тут делаю коннект этой ноды с предидущей
// 		}
// 	}
// }