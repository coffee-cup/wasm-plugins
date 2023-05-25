wit_bindgen::generate!({path: "../wit/handler.wit"});

struct MyHandler;

impl Handler for MyHandler {
    fn run() -> wit_bindgen::rt::string::String {
        // log("Hello from rust!");

        "I am a string in the WASM module".into()
    }
}

export_handler!(MyHandler);
