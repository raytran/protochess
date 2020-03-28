const {ProtochessEngine} = require('bindings')('protochess_engine');

let myObj = new ProtochessEngine();
console.log(myObj.takeTurn(0, 1, 0, 2, 0));
console.log(myObj.toString());
console.log(myObj.takeTurn(0, 6, 0, 4, 1));
console.log(myObj.toString());

