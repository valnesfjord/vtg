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
/// MiddlewareChain is a struct that stores middlewares and executes them in order
///
/// # Examples
///```
///use vtg::{
///    structs::{
///        context::UnifyedContext,
///        middleware::MiddlewareChain,
///    },
///}
///async fn catch_new_message(ctx: UnifyedContext) -> UnifyedContext {
///    ctx
///}
///let mut middleware_chain = MiddlewareChain::new();
///middleware_chain.add_middleware(|ctx| Box::pin(catch_new_message(ctx)));
///```
impl MiddlewareChain {
    pub fn new() -> Self {
        MiddlewareChain {
            middlewares: Vec::new(),
        }
    }

    pub fn add_middleware(&mut self, middleware: Middleware) {
        self.middlewares.push(middleware);
    }

    pub async fn execute(&self, mut ctx: UnifyedContext) {
        for middleware in &self.middlewares {
            ctx = middleware(ctx).await;
        }
    }
}
