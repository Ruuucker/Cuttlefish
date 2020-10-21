const xmlConvert = require('xml-js');
const fs = require('fs');
const express = require('express');
const app = express();

function startServer (fileName) {
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
}

module.exports = {
	startServer: startServer
}