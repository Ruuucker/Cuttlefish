"use strict";

const xmlConvert = require('xml-js');
const fs = require('fs');
const express = require('express');
const app = express();

function startServer (jsonPaths) {

	// Впоследствии использовать это вместо постоянных ручных прописей всех реквестов 
	app.use(express.static('libs'))

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

	// Из за того что експресс не желает видеть переменные цикла for, буду действовать в обход
	app.use(express.static('/'))

	app.listen(8080, () => console.log('App listening on port 8080 \nhttp://localhost:8080'));
}

module.exports = {
	startServer: startServer
}