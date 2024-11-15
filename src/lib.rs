mod bindings;

use std::{fs, sync::Arc};

pub use bindings::wasi::http::types::{
    Fields, IncomingRequest, OutgoingBody, OutgoingResponse, ResponseOutparam,
};

struct Component;

bindings::export!(Component with_types_in bindings);

fn print_dir() {
    let dir_path = "."; // Current directory in the sandbox

    // Read the directory
    let entries = fs::read_dir(dir_path).unwrap();

    println!("Files and directories in '{}':", dir_path);

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        println!("{}", path.display());
    }
}

fn find_entry(path: Arc<String>) -> Vec<u8> {
    // Get the directory of the executable
    // Open and read the file at the path, returning the contents
    println!("Reading  {path:?}");
    match fs::read(&*path) {
        Ok(contents) => {
            // Return the executable contents
            contents
        }
        Err(e) => format!("Failed to read executable: {e:?}").into_bytes(),
    }
}

impl bindings::exports::wasi::http::incoming_handler::Guest for Component {
    fn handle(_request: IncomingRequest, outparam: ResponseOutparam) {
        println!("Handling request");
        print_dir();
        let hdrs = Fields::new();
        let resp = OutgoingResponse::new(hdrs);
        let body = resp.body().expect("outgoing response");

        ResponseOutparam::set(outparam, Ok(resp));

        let out = body.write().expect("outgoing stream");
        let data = find_entry(Arc::new(
            "./target/wasm32-wasip1/debug/gram.wasm".to_string(),
        ));
        println!("Writing response");
        // println!("{:?}", data);
        for chunk in data.chunks(4096) {
            if let Err(e) = out.blocking_write_and_flush(chunk) {
                eprintln!("Failed to write response chunk: {:?}", e);
                return;
            }
        }
        drop(out);
        OutgoingBody::finish(body, None).unwrap();
    }
}
