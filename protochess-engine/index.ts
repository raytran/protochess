const {ProtochessEngineJS} = require('bindings')('protochess_engine_js');
let test = new ProtochessEngineJS();

console.log(test.toString());
console.log(test.takeTurn("E2", "E4", 0));
console.log(test.toString());
console.log(test.takeTurn("D7", "D5", 1));
console.log(test.toString());
console.log(test.takeTurn("B1", "C3", 0));
console.log(test.toString());
console.log(test.takeTurn("E8", "D7", 1));
console.log(test.toString());

export default ProtochessEngineJS;
