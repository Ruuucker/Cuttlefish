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
        let arrForCheck = array[i]._attributes.output.split(' ');
        let returnArr = [];

        // Пошло всё в жопу, пока что оставлю это так
        arrForCheck.forEach(value => { 
            value = value.match(/[0-9][0-9][0-9]\.[0-9][0-9][0-9]\.[0-9][0-9][0-9]\.[0-9][0-9][0-9]/gi);
           
            if (value != null) {
                value = value[0].replace(/\D\./gi, '');
                returnArr.push(value); 
            }
        });
        
        console.log(returnArr);
        console.log('end of iterarion');
        //clearIPsArray = clearIPsArray.concat(returnArr);
    }

    /*
     * function onlyUnique(value, index, self) {
  return self.indexOf(value) === index;
}

// usage example:
var a = ['a', 1, 'a', 2, '1'];
var unique = a.filter(onlyUnique);

console.log(unique); // ['a', 1, 2, '1']
*/
    // И тут я вновь прогоняю весь массив чтобы удалить дубликаты и все символы кроме ip. Почему я не сделал этого в предидущем фильтре? Потому что оно не хочет, может потом пойму как укротить
    //clearIPsArray = clearIPsArray.filter((value) => { return value.replace(/\D/gi, '') });
    return clearIPsArray;
}

module.exports = {
	startScript:  startScript,
    scriptParseIP:   scriptParseIP 
}
