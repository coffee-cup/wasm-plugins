use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};

bindgen!("handler" in "../wit");

use host::*;

struct State;
// impl HandlerImports for State {
//     fn log(&mut self, msg: String) -> wasmtime::Result<()> {
//         println!("{}", msg);

//         Ok(())
//     }
// }

impl Host for State {
    fn get_route(&mut self) -> wasmtime::Result<String> {
        Ok("/".to_string())
    }
}

fn main() -> wasmtime::Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;

    let mut linker = Linker::new(&engine);
    Handler::add_to_linker(&mut linker, |state: &mut State| state)?;

    let mut store = Store::new(&engine, State);

    let component = Component::from_file(&engine, "handlers/hello-handler.wasm")?;
    let (bindings, _) = Handler::instantiate(&mut store, &component, &linker)?;

    let res = bindings.call_run(&mut store)?;
    println!("Got result {}", res);

    Ok(())
}
