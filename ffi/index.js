//ffi
var ffi = require('ffi')

var addon = ffi.Library('./native/fib/target/release/libfib.dylib', {
  fib: ['int', ['int']]
})

module.exports = addon.fib;
