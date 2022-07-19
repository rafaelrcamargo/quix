//! # Routes constants
//! Used to facilitate the routing of the application.
//! Connecting the client to the VTEX IO Builder via the API.
//!
//! # Routes
//! - `link`: The link to the VTEX IO Builder.
//!   - `"link/{}.{}@{}?tsErrorsAsWarnings=false"`
//!
//! # Panics
//! This module panics if the `link` route is not found.

use crate::configs::Environment;
use crate::configs::Project;

/// # Routes Struct
/// This struct contains the routes to the VTEX IO Builder.
pub enum Routes {
    Link,
}

/// # Routes implementation
/// This implementation contains the routes to the Builder.
impl Routes {
    /// # Routes::assemble
    /// This function assembles the routes.
    pub fn assemble(route: Routes) -> String {
        let project = Project::info();
        let env = Environment::info();

        let base = format!(
            "https://app.io.vtex.com/vtex.builder-hub/v0/{}/{}/_v/builder/0/",
            project.vendor, env.current_workspace,
        );
        let path = format!(
            "link/{}.{}@{}?tsErrorsAsWarnings=false",
            project.vendor, project.name, project.version
        );

        match route {
            Routes::Link => return format!("{}{}", base, path),
        }
    }
}
