use std::sync::{Arc, Mutex};

use super::iron::{Request, IronResult, IronError, Handler, Response, status};
use super::iron::middleware::{BeforeMiddleware, AroundMiddleware};
use super::iron::headers::Authorization;
use super::iron::modifiers::RedirectRaw;
use super::plugin::Extensible;
use super::uuid::Uuid;

use super::sessions::{SessionStore, SessionKey, Session};

type SStore = Arc<Mutex<SessionStore>>;


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


struct SessionWatchHandler<H: Handler> {
    store: SStore,
    handler: H,
}
impl<H: Handler> Handler for SessionWatchHandler<H> {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        let mut store = self.store.lock().unwrap();
        let sess = match request.headers.get::<Authorization<String>>() {
            Some(token) => match store.get_mut(&token) {
                Some(session) => Some(session),
                _ => None,
            },
            _ => None,
        };
        println!("in around!");
        let login_path = request.url.path().iter().map(|p| p.to_string()).next().unwrap_or("".to_string());
        if sess.is_none() && login_path != "login" {
            return Ok(Response::with((status::Found,
                                      RedirectRaw("/login".to_string()))));
        }
        self.handler.handle(request)
    }
}

pub struct SessionWatch {
    store: SStore,
}
impl SessionWatch {
    pub fn new(store: SStore) -> SessionWatch {
        SessionWatch {
            store: store,
        }
    }
}
impl AroundMiddleware for SessionWatch {
    fn around(self, handler: Box<Handler>) -> Box<Handler> {
        Box::new(SessionWatchHandler {
            store: self.store,
            handler: handler,
        }) as Box<Handler>
    }
    //fn before(&self, request: &mut Request) -> IronResult<()> {
    //    match request.headers.get::<Authorization<String>>() {
    //        Some(ref token) => {
    //            let sess = self.store.get_mut(&token);
    //            println!("auth: {:?}", token);
    //        }
    //        _ => (),
    //    };
    //    let ext = request.extensions_mut();
    //    ext.insert::<SessionKey>(Session::new(&Uuid::new_v4()));
    //    println!("in sessionwatch");
    //    Ok(())
    //}
    //fn catch(&self, _: &mut Request, err: IronError) -> IronResult<()> {
    //    Err(err)
    //}
}
