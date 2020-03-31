const {ProtochessEngineJS} = require('bindings')('protochess_engine_js');
let test = new ProtochessEngineJS();

console.log(test.perft(4));

export default ProtochessEngineJS;
