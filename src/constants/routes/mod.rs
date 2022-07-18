use crate::configs::Environment;
use crate::configs::Project;

pub enum Routes {
    Link,
}

impl Routes {
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
