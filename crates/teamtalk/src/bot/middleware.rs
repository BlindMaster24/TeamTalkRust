use super::router::HandlerResult;
use crate::events::Result;

pub trait Middleware {
    fn before(&mut self, _ctx: &mut super::Context<'_>) -> Result<HandlerResult> {
        Ok(HandlerResult::Continue)
    }

    fn after(&mut self, _ctx: &mut super::Context<'_>) -> Result<()> {
        Ok(())
    }
}
