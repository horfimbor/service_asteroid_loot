use crate::mod_service::ServiceServer;

mod mod_service {
    use std::fs;
    use std::io::Read;
    use std::path::Path;

    use tiny_http::{Request, Response, Server};

    pub struct ServiceServer {
        server: tiny_http::Server
    }

    impl ServiceServer {
        pub fn new() -> Self {
            ServiceServer {
                server: Server::http("0.0.0.0:8000").unwrap()
            }
        }

        pub fn start(&self) -> ! {
            println!("listening on 8000");

            loop {
                for request in self.server.incoming_requests() {
                    println!("received request!\n, method: {:?}\n, url: {:?}\n, headers: {:?}\n",
                             request.method(),
                             request.url(),
                             request.headers(),
                    );

                    let url = request.url().to_string();
                    let path = Path::new(&url);

                    let metadata = fs::metadata(&path);


                    if metadata.is_ok() && metadata.unwrap().is_file() {
                        let file = fs::File::open(&path).unwrap();
                        let mut response = tiny_http::Response::from_file(file);
                        let header = tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"application/javascript"[..]).unwrap();

                        response.add_header(header);
                        add_cors(&request, &mut response);

                        request.respond(response).unwrap();
                    } else {
                        let mut response = Response::from_string("hello world");
                        add_cors(&request, &mut response);
                        request.respond(response).unwrap();
                    }
                }
            }
        }
    }


    fn add_cors<T: Read>(request: &Request, response: &mut Response<T>) {

        //TODO check main domain

        for h in request.headers() {
            if h.field.equiv("Origin") {
                let header = tiny_http::Header::from_bytes(&b"Access-Control-Allow-Origin"[..], h.value.as_bytes()).unwrap();
                response.add_header(header);
            }
        }

        let header = tiny_http::Header::from_bytes(&b"Access-Control-Allow-Methods"[..], &b"POST, GET"[..]).unwrap();
        response.add_header(header);
        let header = tiny_http::Header::from_bytes(&b"Access-Control-Max-Age"[..], &b"86400"[..]).unwrap();
        response.add_header(header);
        let header = tiny_http::Header::from_bytes(&b"Vary"[..], &b"Origin"[..]).unwrap();
        response.add_header(header);
        let header = tiny_http::Header::from_bytes(&b"Access-Control-Allow-Headers"[..], &b"body, cache, Content-Type"[..]).unwrap();
        response.add_header(header);
    }
}

fn main() {
    let _guard = sentry::init("http://8de1844f5c2c414dae0931f8254b1c07@sentry-base:9000/3");

    sentry::capture_message("Hello World!", sentry::Level::Info);

    sentry::integrations::panic::register_panic_handler();

    let server = ServiceServer::new();

    server.start();
}
