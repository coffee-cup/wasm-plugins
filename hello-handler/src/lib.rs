wit_bindgen::generate!({path: "../wit/handler.wit"});

struct MyHandler;

impl Handler for MyHandler {
    fn run() -> wit_bindgen::rt::string::String {
        // log("Hello from rust!");

        "yoo".into()
    }
}

export_handler!(MyHandler);
