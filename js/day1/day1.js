const fs = require('fs');


fs.readFile('data.txt', 'utf8', (err, data)  => {
    let lines = data.split(/\r?\n/);

    const carriedCalories = [];
    let sumOfCalories = 0;

    lines.forEach(line => {
        if (line !== '') {
            sumOfCalories += parseInt(line);
        } else {
            carriedCalories.push(sumOfCalories);
            sumOfCalories = 0;
        }
    });

    carriedCalories.sort(compareNumbers);

    const topOne = carriedCalories[0];
    const topThree = carriedCalories.slice(0, 3).reduce((sum, v) => sum + v);

    console.log('Part 1:');
    console.log(topOne);

    console.log('\nPart 2:');
    console.log(topThree);
});

function compareNumbers(a, b) {
    return b - a;
}
