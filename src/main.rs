//Initialisation des "crates" ou des librairies suplémentaires nécessaires.
use number_guessing_game::{
    menus::start::main_menu,
    utilities::{
        cls_scr::cls_title, errors::err_print, score_board::file::read_score_file,
        settings::file::settings_file,
    },
    ErrFormat, RuntimeFunctionBlob,
};

//Logiciel mère.
fn main() {
    //
    cls_title();

    let settings_loading_check: Result<RuntimeFunctionBlob, ErrFormat> = settings_file();

    //Initialisation des vars, constantes et plages si applicable.
    match settings_loading_check {
        Ok(mut runtime_blob) => {
            let score_loading_check: Result<Vec<String>, ErrFormat> =
                read_score_file(&runtime_blob);

            let high_scores: Vec<String> = match &score_loading_check {
                Ok(_) => score_loading_check.unwrap(),
                Err(error_handler) => {
                    err_print(error_handler);
                    score_loading_check.unwrap()
                }
            };

            while !&runtime_blob.core_functions.stop {
                runtime_blob = main_menu(runtime_blob, &high_scores)
            }

            runtime_blob
        }
        Err(ref error_handler) => {
            err_print(error_handler);
            settings_loading_check.unwrap()
        }
    };
}
