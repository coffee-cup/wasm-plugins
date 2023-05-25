use std::sync::Arc;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};

bindgen!("handler" in "../wit");

use host::*;

/// State available to the WASM module handler implementation
struct State {
    route: String,
}

/// Implementation of the handler-api interface
/// See `wit/handler.wit` for the interface definition
impl Host for State {
    fn get_route(&mut self) -> wasmtime::Result<String> {
        Ok(self.route.clone())
    }
}

/// Represents a WASM module that implements the handler-api interface
#[derive(Clone, Debug)]
struct HandlerComponent {
    pub route: String,
    pub name: String,
}

impl HandlerComponent {
    /// Creates a new handler component and loads the WASM module
    pub fn new(route: &str, name: &str) -> Self {
        Self {
            route: route.into(),
            name: name.into(),
        }
    }
}

/// Call the WASM module with required state
/// The response of the module is returned as the body of the request
async fn handle_route(handler: Arc<HandlerComponent>) -> Result<impl Responder> {
    // There is probably a more efficient way of doing this than creating a new engine for each request
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config).expect("Failed to create WASM engine");

    println!(
        "Creating and running WASM component: handlers/{}.wasm",
        handler.name
    );
    let component = Component::from_file(&engine, format!("handlers/{}.wasm", handler.name))
        .expect("Failed to create WASM component");

    let mut linker = Linker::new(&engine);
    Handler::add_to_linker(&mut linker, |state: &mut State| state)
        .expect("Failed to add handler to linker");

    let mut store = Store::new(
        &engine,
        State {
            route: handler.route.clone(),
        },
    );

    let (bindings, _) = Handler::instantiate(&mut store, &component, &linker)
        .expect("Failed to instantiate WASM module");

    let res = bindings
        .call_run(&mut store)
        .expect("Failed to call run on WASM module");

    Ok(HttpResponse::Ok().body(res))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (host, port) = ("127.0.0.1", 9090);

    // Get handler wasm modules
    let handlers = vec![Arc::new(HandlerComponent::new("/", "hello-handler"))];

    match HttpServer::new(move || {
        let mut app = App::new();

        for handler in &handlers {
            let handler_clone = Arc::clone(handler);
            app = app.route(
                handler.route.as_str(),
                web::get().to(move || handle_route(Arc::clone(&handler_clone))),
            );
        }

        app
    })
    .bind((host, port))
    {
        Ok(server) => {
            println!("Server started at http://{}:{}", host, port);
            server.run().await
        }
        Err(e) => {
            println!("Could not start the server: {}", e);
            Err(e)
        }
    }
}

// fn run_wasm(module: &str, route: &str) -> Result<()> {
//     let mut config = Config::new();
//     config.wasm_component_model(true);
//     let engine = Engine::new(&config)?;

//     let mut linker = Linker::new(&engine);
//     Handler::add_to_linker(&mut linker, |state: &mut State| state)?;

//     let mut store = Store::new(
//         &engine,
//         State {
//             route: route.into(),
//         },
//     );

//     let component = Component::from_file(&engine, "handlers/hello-handler.wasm")?;
//     let (bindings, _) = Handler::instantiate(&mut store, &component, &linker)?;

//     let res = bindings.call_run(&mut store)?;
//     println!("Got result {}", res);

//     Ok(())
// }
