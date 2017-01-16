#[macro_use]
extern crate neon;

use neon::vm::{Call, JsResult};
use neon::js::JsInteger;

fn fib(n: i32) -> i32 {
  match n {
    1 => 1,
    2 => 1,
    _ => fib(n - 1) + fib(n - 2)
  }
}

fn fib_wrapper(call: Call) -> JsResult<JsInteger> {
  let scope = call.scope;
  let args = call.arguments.require(scope, 0)?.check::<JsInteger>()?;
  let num = args.value() as i32;
  Ok(JsInteger::new(scope, fib(num)))
}

register_module!(m, {
  m.export("fib", fib_wrapper)
});
