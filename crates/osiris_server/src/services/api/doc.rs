//! The `OpenAPI` document's frame: everything else comes from the routes.

use utoipa::{
    Modify, OpenApi,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};

#[derive(OpenApi)]
#[openapi(
    info(title = "Osiris", description = "Frubble's integrations service: OAuth connections to providers, their tokens kept fresh, and typed actions with them.", version = "2026-11-04.anubis"),
    modifiers(&SecuritySchemes),
    components(schemas(osiris_api_models::events::EventDelivery)),
    tags((name = "auth-configs", description = "The providers' apps (admin key)"),
         (name = "connected-accounts", description = "Connections and their consent (API key; refreshing one now takes the admin key)"),
         (name = "actions", description = "Reads with a connection (API key)"),
         (name = "events", description = "Test events (admin key)")),
)]
pub struct ApiDoc;

struct SecuritySchemes;

impl Modify for SecuritySchemes {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);
        for name in ["api_key", "admin_key"] {
            components.add_security_scheme(
                name,
                SecurityScheme::Http(HttpBuilder::new().scheme(HttpAuthScheme::Bearer).build()),
            );
        }
    }
}
