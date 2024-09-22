use actix_web::guard::Guard;
use log::debug;

use crate::{models::user::Role, routes::auth::Claims};

pub struct RoleGuard(pub Vec<Role>);

impl Guard for RoleGuard {
    fn check(&self, ctx: &actix_web::guard::GuardContext<'_>) -> bool {
        let path = ctx.head().uri.path();
        if let Some(claims) = ctx.req_data().get::<Claims>() {
            debug!(
                "Path: '{}', User: '{}', User Roles: {:?}, Required Roles: {:?}",
                path, claims.sub, claims.roles, self.0
            );
            claims.roles.iter().any(|role| self.0.contains(role))
        } else {
            debug!(
                "Path: '{}', No claims found, Required Roles: {:?}",
                path, self.0
            );
            false
        }
    }
}
