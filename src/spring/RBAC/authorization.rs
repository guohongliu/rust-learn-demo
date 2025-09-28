use crate::spring::RBAC::authentication::Authentication;

#[derive(Debug, PartialEq)]
pub enum AuthorizationDecision {
    Alowed,
    Denied(DeniedReason),
}

#[derive(Debug, PartialEq)]
pub enum DeniedReason {
    NoAuthenticated,
    InsufficientPermissions,
}

#[derive(Debug, Clone)]
pub struct Resource {
    pub path: String,
    pub method: String,
}

pub trait AuthorizationManager: Send + Sync {
    fn check(
        &self,
        auth: Option<Authentication>,
        resource: &Resource,
    ) -> AuthorizationDecision;
}