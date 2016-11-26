
use super::iron::{Request, IronResult, IronError};
use super::iron::middleware::BeforeMiddleware;

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
