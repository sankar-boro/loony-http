// use loony_http;
use crate::App;

// pub type AppInstance = Box<dyn Fn() -> App + Send + Sync + 'static>;
pub type AppInstance = Box<dyn Fn() -> App<'static> + 'static>;

pub struct HttpServer {
    app: AppInstance,
}

impl HttpServer {
    pub fn new<F: Fn() -> App<'static> + 'static>(app: F) -> Self {
        Self { app: Box::new(app) }
    }

    pub fn run(&self) {
        let app = (self.app)();
        println!("Hello world! Welcome to {}", app.app_name());
    }
}