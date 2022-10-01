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

use crate::configs::Project;
use crate::configs::Vtex;

/// # Routes Struct
/// This struct contains the routes to the VTEX IO Builder.
pub enum Routes {
    Link,
    Relink,
    Availability,
    Clean,
}

/// # Routes implementation
/// This implementation contains the routes to the Builder.
impl Routes {
    /// # Routes::assemble
    /// This function assembles the routes.
    pub fn assemble(route: Routes) -> String {
        let project = Project::info().unwrap();
        let env = Vtex::info();

        let base = format!(
            "https://app.io.vtex.com/vtex.builder-hub/v0/{}/{}/_v/builder/0/",
            project.vendor, env.workspace,
        );

        let link_path = format!(
            "link/{}.{}@{}?tsErrorsAsWarnings=false",
            project.vendor, project.name, project.version
        );

        let relink_path = format!(
            "relink/{}.{}@{}?tsErrorsAsWarnings=false",
            project.vendor, project.name, project.version
        );

        let availability_path = format!(
            "availability/{}.{}@{}",
            project.vendor, project.name, project.version
        );

        let clean_path = format!(
            "clean/{}.{}@{}",
            project.vendor, project.name, project.version
        );

        match route {
            Routes::Link => format!("{}{}", base, link_path),
            Routes::Relink => format!("{}{}", base, relink_path),
            Routes::Availability => format!("{}{}", base, availability_path),
            Routes::Clean => format!("{}{}", base, clean_path),
        }
    }
}
