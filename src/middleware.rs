//! Custom Middleware
//!
use std::sync::{Arc, Mutex};
use std::collections::HashSet;

use iron::{Request, IronResult, IronError, Handler, Response, status};
use iron::middleware::{BeforeMiddleware, AroundMiddleware};
use iron::headers::Authorization;

use sessions::SessionStore;

type SStore = Arc<Mutex<SessionStore>>;


/// Simple info logger to display the incoming request method & url
pub struct InfoLog;
impl InfoLog {
    pub fn new() -> InfoLog {
        InfoLog{}
    }
}
impl BeforeMiddleware for InfoLog {
    fn before(&self, request: &mut Request) -> IronResult<()> {
        println!("[{:?}]: {}", request.method, request.url);
        Ok(())
    }
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<()> {
        Err(err)
    }
}


/// Session middleware handler to look for an auth/session token
/// in the request.headers typemap and either return an
/// unauthorized response or call the provided handler.
/// This handler is intended to be returned from SessionMiddleware (AroundMiddleware)
struct SessionMiddlewareHandler<H: Handler> {
    store: SStore,
    exempt_url_roots: HashSet<&'static str>,
    handler: H,
}
impl<H: Handler> Handler for SessionMiddlewareHandler<H> {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        { // move to inner scope so borrow in request is dropped before calling the handler
            let url_path = request.url.path();
            let url_root = url_path.iter().next().unwrap();
            if !url_root.is_empty() && !self.exempt_url_roots.contains(url_root) {
                // note: store-lock needs to get dropped before the handler is called below
                let mut store = self.store.lock().unwrap();
                let valid = match request.headers.get::<Authorization<String>>() {
                    Some(token) => {
                        store.check_delete(&token)
                    },
                    _ => false,
                };
                if !valid {
                    return Ok(Response::with((status::Unauthorized, "unauthorized")))
                }
            }
        }
        self.handler.handle(request)
    }
}


/// SessionMiddleware (AroundMiddleware) intended to check incoming
/// requests for an authorized session-token and reject any non token or
/// expired token requests.
pub struct SessionMiddleware {
    store: SStore,
    exempt_url_roots: HashSet<&'static str>,
}
impl SessionMiddleware {
    pub fn new(store: SStore, exempt_url_roots: HashSet<&'static str>) -> SessionMiddleware {
        SessionMiddleware {
            store: store,
            exempt_url_roots: exempt_url_roots,
        }
    }
}
impl AroundMiddleware for SessionMiddleware {
    fn around(self, handler: Box<Handler>) -> Box<Handler> {
        Box::new(SessionMiddlewareHandler {
            store: self.store,
            exempt_url_roots: self.exempt_url_roots,
            handler: handler,
        }) as Box<Handler>
    }
}
