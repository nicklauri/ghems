use aliri::jwt;
use aliri_axum::scope_guards;
use aliri_braid::braid;
use aliri_clock::UnixTime;
use aliri_oauth2::{oauth2, scope};

scope_guards! {
    type Claims = GhemClaims;

    scope Admin = "admin";
    scope User = "user";
    scope Anonymous = *;
    scope DenyAll = [];
}

pub mod scope {
    use aliri_oauth2::{scope, Scope};

    pub const ADMIN: Scope = scope!["admin"];
}

#[braid(serde)]
struct ClientId;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GhemClaims {
    iss: jwt::Issuer,
    aud: jwt::Audiences,
    sub: jwt::Subject,
    exp: UnixTime,
    iat: UnixTime,
    azp: ClientId,
    scope: oauth2::Scope,
    perms: oauth2::Scope,
}

impl jwt::CoreClaims for GhemClaims {
    fn nbf(&self) -> Option<UnixTime> {
        None
    }
    fn exp(&self) -> Option<UnixTime> {
        Some(self.exp)
    }
    fn aud(&self) -> &jwt::Audiences {
        &self.aud
    }
    fn iss(&self) -> Option<&jwt::IssuerRef> {
        Some(&self.iss)
    }
    fn sub(&self) -> Option<&jwt::SubjectRef> {
        Some(&self.sub)
    }
}

impl oauth2::HasScope for GhemClaims {
    fn scope(&self) -> &oauth2::Scope {
        &self.scope
    }
}

pub fn generate_access_token() {
    // scope!
}
