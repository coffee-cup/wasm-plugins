wit_bindgen::generate!({path: "../wit/handler.wit"});

struct MyHandler;

impl Handler for MyHandler {
    fn run() -> wit_bindgen::rt::string::String {
        // log("Hello from rust!");

        let route = host::get_route();
        format!("I am a string in the WASM module\nRoute: {}", route)
    }
}

export_handler!(MyHandler);
