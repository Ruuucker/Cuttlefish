const xmlConvert = require('xml-js');
const fs = require('fs');


const express = require('express');
const app = express();

app.get('/', (req, res) => {
    res.sendFile('test.html', {root: __dirname })
});

app.get('/libs/vivagraph.js', (req, res) => {
    res.sendFile('libs/VivaGraphJS/dist/vivagraph.js', {root: __dirname })
});

app.get('/testJSON', (req, res) => {
    res.sendFile('./testJSON.json', {root: __dirname })
});

app.get('/image', (req, res) => {
    res.sendFile('libs/none.jpg', {root: __dirname })
});

app.listen(8080, () => console.log('App listening on port 8080'));


// var createGraph = require('ngraph.graph');
// const viva = require('vivagraphjs');
// var graph = viva.Graph.graph();
// graph.addLink(1, 2);
 
// var renderer = viva.Graph.View.renderer(graph);
// renderer.run();

// Импорт и преобразование
var nmapOutputXml = fs.readFileSync('./test3.xml', {encoding:'utf8', flag:'r'});
var nmapOutputJSON = xmlConvert.xml2json(nmapOutputXml, {compact: true, spaces: 0});

// Запись JSON
fs.writeFileSync('./testJSON.json', nmapOutputJSON);
nmapOutputJSON = JSON.parse(nmapOutputJSON);


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