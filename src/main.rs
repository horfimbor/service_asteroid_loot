use crate::mod_service::ServiceServer;

mod mod_service {
    use std::{env, fs};
    use std::collections::HashMap;
    use std::io::Read;
    use std::path::Path;

    use tiny_http::{Method, Request, Response, Server, StatusCode};

    const ENV_SENTRY_DSN: &str = "MOD_SERVICE_SENTRY_DSN";

    pub struct ServiceServer {
        server: tiny_http::Server,
        get: HashMap<String, fn(Request)>,
    }

    impl ServiceServer {
        pub fn new() -> Self {
            match env::var(&ENV_SENTRY_DSN) {
                Ok(val) => {
                    let _guard = sentry::init(val);

                    sentry::capture_message("Hello World!", sentry::Level::Info);

                    sentry::integrations::panic::register_panic_handler();
                }
                Err(_e) => panic!("mod_token require an env value for MOD_TOKEN_SECRET"),
            }

            ServiceServer {
                server: Server::http("0.0.0.0:8000").unwrap(),
                get: HashMap::<String, fn(Request)>::new(),
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
                    match self.handle(request, &dbs) {
                        Err(_e) => {
                            println!("cannot respond")
                        }
                        Ok(()) => {}
                    }
                }
            }
        }

        fn handle(&self, request: Request, dbs: &Dbs) -> Result<(), IoError> {
            if request.method() == &Method::Post {
                self.handle_post(request, &dbs)
            } else if request.method() == &Method::Get {
                self.handle_get(request)
            } else if request.method() == &Method::Options {
                self.handle_option(request)
            } else {
                let mut response = Response::new_empty(StatusCode(405));
                add_cors(&request, &mut response);
                request.respond(response)
            }
        }

        fn handle_get(&self, request: Request) -> Result<(), IoError> {
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
            }

            if authorized {
                match self.get.contains_key(&url) {
                    Some(fn_get) => request.respond(response),
                    None => {
                        let mut response = Response::new_empty(StatusCode(404));
                        add_cors(&request, &mut response);
                        request.respond(response)
                    }
                }
            } else {
                let mut response = Response::new_empty(StatusCode(403));
                add_cors(&request, &mut response);
                request.respond(response)
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
    let server = ServiceServer::new();

    server.start();
}
