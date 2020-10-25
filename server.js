const xmlConvert = require('xml-js');
const fs = require('fs');
const express = require('express');
const app = express();

function startServer (jsonPaths) {
	app.get('/', (req, res) => {
	    res.sendFile('test.html', {root: __dirname })
	});

	app.get('/libs/vivagraph.js', (req, res) => {
	    res.sendFile('libs/VivaGraphJS/dist/vivagraph.js', {root: __dirname })
	});

	app.get('/image', (req, res) => {
	    res.sendFile('libs/none.jpg', {root: __dirname })
	});

	app.get('/test', (req, res) => {
	    res.send(jsonPaths)
	});

	var t;
	for (var i = jsonPaths.length - 1; i >= 0; i--) {
		// Из за того что експресс не желает видеть переменные цикла for, буду действовать в обход
		t = i;
		app.get(jsonPaths[t], (req, res) => {
			console.log(jsonPaths[t]);
		    res.sendFile(jsonPaths[t])
		});
	}

	app.listen(8080, () => console.log('App listening on port 8080 \nhttp://localhost:8080'));
}

module.exports = {
	startServer: startServer
}