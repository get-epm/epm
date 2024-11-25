use crate::epm_config::{get_epm_config, EpmConfig};
use crate::files_composer::{ComposerJson, ComposerLock};
use crate::files_epm::{EpmJson, EpmLock};

pub(crate) struct ComposerProjectFiles {
    loaded_decl: Option<ComposerJson>,
    loaded_lock: Option<ComposerLock>,
}

pub(crate) struct EpmProjectFiles {
    loaded_decl: Option<EpmJson>,
    loaded_lock: Option<EpmLock>,
}

pub(crate) enum ProjectFiles {
    /// - composer.json and composer.lock files
    /// - composer commands
    Composer(ComposerProjectFiles),
    /// - .php-package/* files
    /// - composer commands
    Epm(EpmProjectFiles),
}

pub(crate) struct ProjectContext {
    pub(crate) config: EpmConfig,
    pub(crate) project_files: ProjectFiles,
}

pub(crate) fn get_project_context() -> Result<ProjectContext, ()> {
    Ok(ProjectContext {
        config: get_epm_config()?, // TODO: handle error
        project_files: ProjectFiles::Epm(EpmProjectFiles {
            loaded_decl: Some(EpmJson {}),
            loaded_lock: Some(EpmLock {}),
        }),
    })
}
