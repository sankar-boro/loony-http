use crate::{
    service::ServiceRequest,
    resource::ResourceService,
    request::{Request, HttpRequest}, 
    extensions::Extensions,
};
use ahash::AHashMap;
use s4nk4r_service::Service;
use std::{cell::RefCell,rc::Rc};
use futures::executor::block_on;

pub struct Response<'a> {
    routes: &'a AHashMap<String, Rc<RefCell<ResourceService>>>,
    extensions: Rc<Extensions>
}

impl<'a> Response<'a> {
    pub fn new(routes: &'a AHashMap<String, Rc<RefCell<ResourceService>>>, extensions: Rc<Extensions>) -> Self {
        Self {
            routes,
            extensions,
        }
    }

    pub fn build(&self, req: &Request) -> Result<String, ()> {
        if let Some(path) = &req.uri {
            let sp: Vec<&str> = path.split("?").collect();
            println!("{:?}", sp);
            let mut sp = sp.iter();
            if let Some(prefix_uri) = sp.next() {
                let service = self.routes.get(prefix_uri.to_owned());
                if let Some(s) = service {
                    let sr = ServiceRequest(HttpRequest { url: String::from(path), extensions: self.extensions.clone() });
                    let mut a = Rc::clone(s);
                    let b = a.call(sr);
                    let c = block_on(b).unwrap();
                    let d = c.0.value;
                    return Ok(d);
                }
            }
        }
        Err(())
    }
}
