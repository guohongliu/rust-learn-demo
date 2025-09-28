use std::cell::RefCell;
use crate::spring::RBAC::authentication::Authentication;

thread_local! {
    static SECURITY_CONTEXT: RefCell<Option<Authentication>> = RefCell::new(None);
}

pub struct SercurityContext;
impl SercurityContext {
    pub fn set_authentication(auth: Authentication) {
        SECURITY_CONTEXT.with(|security_context| {
            *security_context.borrow_mut() = Some(auth);
        })
    }

    pub fn get_authentication() -> Option<Authentication> {
        SECURITY_CONTEXT.with(|security_context| security_context.borrow().clone())
    }

    pub fn clear() {
        SECURITY_CONTEXT.with(|security_context| *security_context.borrow_mut() = None)
    }
}