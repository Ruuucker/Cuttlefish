"use strict";

const { exec } = require('child_process');

function startScript (scriptNameArr, dirToSave, fileName) {

	return new Promise ((resolve, reject) => {
		let wholePath = dirToSave + fileName + '.xml';
        
        exec(`sudo nmap -T5 --min-parallelism 100 --max-parallelism 256 --script ${scriptNameArr} -oX ${wholePath}`, (error, stdout, stderr) => {	
            if (error) {
				console.error(`exec error: ${error}`);
				return;
			}
			// console.log(`stdout: ${stdout}`);
            
            // Соединяю 2 значения чтобы вернуть и путь и вывод
            resolve ([ wholePath, stdout ]);
		});
	});	
}

function scriptParseIP(array) {
    // Из этой функции должны вылетать IP
    let clearIPsArray = [];

    for (let i = 0; i < array.length; i++) {
        let dataString = array[i]._attributes.output; 
        let returnArr = [];
        dataString = dataString.match(/[0-9][0-9]*[0-9]*\.[0-9][0-9]*[0-9]*\.[0-9][0-9]*[0-9]*\.[0-9][0-9]*[0-9]*/gim);
        
        if (dataString != null) {
            dataString.forEach(value => {
                if (!value.includes('255'))
                    returnArr.push(value.replace(/\D\./gi, '')); 
            });
        }

        clearIPsArray = clearIPsArray.concat(returnArr);
    }

    // И тут я вновь прогоняю весь массив чтобы удалить дубликаты и все символы кроме ip. Почему я не сделал этого в предидущем фильтре? Потому что оно не хочет, может потом пойму как укротить
    clearIPsArray = clearIPsArray.filter((value, index, self) => { return self.indexOf(value) === index });
    
    return clearIPsArray;
}

module.exports = {
	startScript:  startScript,
    scriptParseIP:   scriptParseIP 
}
