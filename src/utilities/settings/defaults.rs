use crate::{RuntimeFunctionBlob,Settings,CoreFunctions,Comunication};

//Génère le fichier d'options s'il n'existe pas et le lit.
pub fn default_settings() -> Box<RuntimeFunctionBlob> {

    //
    let settings: Settings = Settings {
        max_range: 100,
        min_range: 1,
        max_tries: 7,
        guess_hint: true,
        settings_count: 4,
    };

    //
    let core_functions: CoreFunctions = CoreFunctions {
        stop: false,
        first_cycle: true,
    };

    //
    let comunication: Comunication = Comunication {
        user_in_alpha: String::new(),
        user_in_u32: 0,
        err_name: String::new(),
        err_msg: String::new(),
        msg: String::new(),
    };

    //Initialisation des vars, constantes et plages si applicable.
    let runtime_blob: RuntimeFunctionBlob = RuntimeFunctionBlob {
        settings: Box::new(settings),
        core_functions: Box::new(core_functions),
        comunication: Box::new(comunication),
    };

    //
    Box::new(runtime_blob)
}