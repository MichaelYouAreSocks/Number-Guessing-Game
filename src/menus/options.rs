//Initialisation des "crates" ou des librairies suplémentaires nécessaires.
use crate::{
    utilities::{
        cls_scr::cls_title,
        questions::numeric_input,
        settings::{
            edit::save_setting_to_file,
            in_game::{game_hint, game_size, game_tries},
        },
    },
    RuntimeFunctionBlob,
};

//Menu d'options de jeu.
pub fn options_menu(mut runtime_blob: RuntimeFunctionBlob) -> RuntimeFunctionBlob {
    loop {
        //Concatène le menu des option.
        let msg: String = format!(
            "Options:\n{}{}{}{}\n{}{}\n{}{}\n{}",
            "1 -> Size of search.\tMin: ",
            runtime_blob.settings.min_range,
            "\tMax: ",
            runtime_blob.settings.max_range,
            "2 -> Number of tries.\t: ",
            runtime_blob.settings.max_tries,
            "3 -> Game hints.\t: ",
            runtime_blob.settings.guess_hint,
            "0 -> Back to main menu."
        );

        //Permet de choisir quelle option le joueur aimerait changer.
        match numeric_input(&msg) {
            //Retourne au menu d'acueil.
            0 => {
                cls_title();
                break;
            }
            //Option de la taille de la plage à chercher chaque manche.
            1 => {
                cls_title();
                runtime_blob = save_setting_to_file(game_size(runtime_blob));
            }
            //Option du nombre de tentatives par manches.
            2 => {
                cls_title();
                runtime_blob = save_setting_to_file(game_tries(runtime_blob));
            }
            //Option d'indice.
            3 => {
                cls_title();
                runtime_blob = save_setting_to_file(game_hint(runtime_blob));
            }
            //Atrappe touts les autres inputs et indique qu'ils sont incorrect.
            _ => {
                cls_title();
                println!(
                    "Couldn't load setting n°{}.",
                    runtime_blob.comunication.user_in_u32
                );
            }
        };
    }
    runtime_blob
}
