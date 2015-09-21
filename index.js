var ffi = require('ffi');
var path = require('path');
var fibPath = path.join(__dirname, './rust/fib/target/release/libfib.dylib');


//exports rust function
module.exports = ffi.Library(fibPath, {
  fib: ['int', ['int']],
  add: ['int', ['int', 'int']]
});
