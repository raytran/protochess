const {ProtochessEngine} = require('bindings')('protochess_engine');
let myObj = new ProtochessEngine();
console.log(myObj.toPieceString());
console.log(myObj.toBoardString().split("|"));
console.log(myObj.takeTurn("E2", "E4", 0));
console.log(myObj.toString());
console.log(myObj.takeTurn("A7", "A5", 1));
console.log(myObj.toString());
console.log(myObj.takeTurn("F1", "C4", 0));
console.log(myObj.toString());
console.log(myObj.takeTurn("A5", "A4", 1));
console.log(myObj.toString());
console.log(myObj.takeTurn("D1", "H5", 0));
console.log(myObj.toString());
console.log(myObj.takeTurn("A4", "A3", 1));
console.log(myObj.toString());
console.log(myObj.takeTurn("H5", "F7", 0));
console.log(myObj.toString());

