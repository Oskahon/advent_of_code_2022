const fs = require('fs');

fs.readFile('data.txt', 'utf8', (err, data)  => {
    const lines = data.split('\n');
    lines.pop();

    const pointPart1 = new Map([
        ["A X", 4],
        ["A Y", 8],
        ["A Z", 3],
        ["B X", 1],
        ["B Y", 5],
        ["B Z", 9],
        ["C X", 7],
        ["C Y", 2],
        ["C Z", 6],
    ]);

    const pointPart2 = new Map([
        ["A X", 3],
        ["A Y", 4],
        ["A Z", 8],
        ["B X", 1],
        ["B Y", 5],
        ["B Z", 9],
        ["C X", 2],
        ["C Y", 6],
        ["C Z", 7],
    ]);

    let totalScorePart1 = 0;
    let totalScorePart2 = 0;

    for (let i = 0; i < lines.length; i++) {
        const line = lines[i];
        totalScorePart1 += pointPart1.get(line); 
        totalScorePart2 += pointPart2.get(line); 
    }

//    lines.forEach(line => {
//        totalScorePart1 += pointPart1.get(line); 
//        totalScorePart2 += pointPart2.get(line); 
//    });

    console.log('Part 1:');
    console.log(totalScorePart1);

    console.log('Part 2:');
    console.log(totalScorePart2);
});
