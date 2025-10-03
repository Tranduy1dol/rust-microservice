use std::{collections::HashMap, ops::Deref, sync::LazyLock};

use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    Json,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use serde::{Deserialize, Serialize};

use crate::{
    auth::jwt::{decode_jwt_token, TokenClaims},
    config::JWT_CONFIG,
};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AdminScopes {
    // Reading scopes
    AnalyticsRead,
    ProductsRead,
    UsersRead,
    AdminsRead,
    // Writing scopes
    ProductsWrite,
    UsersWrite,
    AdminsWrite,
}

static ADMIN_SCOPE: LazyLock<HashMap<i32, Vec<AdminScopes>>> = LazyLock::new(|| {
    HashMap::from([
        (1, vec![AdminScopes::AnalyticsRead]),
        (2, vec![AdminScopes::ProductsRead, AdminScopes::UsersRead]),
        (
            3,
            vec![
                AdminScopes::AdminsRead,
                AdminScopes::ProductsWrite,
                AdminScopes::UsersWrite,
            ],
        ),
        (4, vec![AdminScopes::AdminsWrite]),
    ])
});

pub trait AdminRoute: Send + Sync {
    fn is_allow(&self, admin_level: Option<i32>) -> bool;
}

#[derive(Clone)]
pub struct RouteState {
    pub scope: AdminScopes,
}

impl RouteState {
    pub fn new(scope: AdminScopes) -> Self {
        RouteState { scope }
    }
}

impl AdminRoute for RouteState {
    fn is_allow(&self, admin_level: Option<i32>) -> bool {
        match admin_level {
            None => false,
            Some(level) => {
                if level > 4 {
                    return true;
                }

                for l in (1..=level).rev() {
                    let permission = ADMIN_SCOPE.get(&l).cloned().unwrap_or_default();

                    if permission.contains(&self.scope) {
                        return true;
                    }
                }

                false
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AdminAuth(pub TokenClaims);

impl<S: Send + Sync + AdminRoute> FromRequestParts<S> for AdminAuth {
    type Rejection = (StatusCode, Json<String>);
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| {
                    (
                        StatusCode::UNAUTHORIZED,
                        Json("Authorize header missing".into()),
                    )
                })?;

        let claim = decode_jwt_token(bearer.token(), JWT_CONFIG.deref())
            .map_err(|e| (StatusCode::UNAUTHORIZED, Json(e.to_string())))?;

        if !state.is_allow(claim.admin_level) {
            return Err((
                StatusCode::FORBIDDEN,
                Json("Admin have no permission to access this resource".into()),
            ));
        }

        Ok(AdminAuth(claim))
    }
}
