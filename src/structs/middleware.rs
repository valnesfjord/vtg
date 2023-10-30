use std::{future::Future, pin::Pin};

use super::context::UnifyedContext;

type Middleware =
    fn(UnifyedContext) -> Pin<Box<dyn Future<Output = UnifyedContext> + Send + 'static>>;

#[derive(Clone)]
pub struct MiddlewareChain {
    middlewares: Vec<Middleware>,
}

impl Default for MiddlewareChain {
    fn default() -> Self {
        MiddlewareChain::new()
    }
}
impl MiddlewareChain {
    pub fn new() -> Self {
        MiddlewareChain {
            middlewares: Vec::new(),
        }
    }

    pub fn add_middleware(&mut self, middleware: Middleware) {
        self.middlewares.push(middleware);
    }

    pub async fn execute(&self, ctx: UnifyedContext) -> UnifyedContext {
        let mut ctx = ctx;
        for middleware in &self.middlewares {
            ctx = middleware(ctx).await;
        }
        ctx
    }
}
