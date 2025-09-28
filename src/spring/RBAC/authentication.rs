use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct Authentication {
    pub principal: String,
    pub credentials: Option<String>,
    pub authorities: HashSet<String>,
    pub is_authenticated: bool,
}

#[derive(Debug)]
pub struct AuthenticationRequest {
    pub username: String,
    pub password: String,
}

pub trait AuthenticationManager: Send + Sync {
    fn authenticate(&self, request: &AuthenticationRequest) -> Result<Authentication, AuthError>;
}


#[derive(Debug)]
pub enum AuthError {
    InvalidCredentials,    // 凭证无效
    UserNotFound,          // 用户不存在
    AuthenticationFailed,  // 认证失败（其他原因）
}