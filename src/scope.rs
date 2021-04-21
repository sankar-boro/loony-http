use crate::route::{Route, RouteService};
use crate::resource::{Resource, ResourceService};
use loony_service::{ServiceFactory};

type ScopeFactory = Box<
    dyn ServiceFactory<
        Request = String, 
        Response = String, 
        Error = (), 
        // Service= RouteService
        Service = ResourceService
    >
>;

pub struct Scope {
    pub scope: String,
    pub services: Vec<ScopeFactory>,
}

impl Scope {
    pub fn new(scope: &str) -> Self {
        Scope {
            scope: scope.to_owned(),
            services: Vec::new(),
        }
    }

    pub fn route(mut self, route: Route) -> Self {
        // self.services.push(Box::new(route));
        self.services.push(Box::new(Resource::new(self.scope.clone()).route(route)));
        self
    }
}


#[cfg(test)]
mod tests {

    async fn index(req: String) -> String {
        req
    }

    #[test]
    fn scope() {
    }
}