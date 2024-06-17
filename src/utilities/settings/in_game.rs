//Initialisation des "crates" ou des librairies suplémentaires nécessaires.
use crate::{
    utilities::{
        cls_scr::cls_title,
        questions::{numeric_input, yes_no_else_input},
    },
    RuntimeFunctionBlob,
};

//Demande la taille de la plage numérique à chercher souhaité.
pub fn game_size(mut runtime_blob: RuntimeFunctionBlob) -> RuntimeFunctionBlob {
    runtime_blob.comunication.msg = format!(
        "Input the largest number you want.\nCurrent:\t{}",
        runtime_blob.settings.max_range
    );
    let tmp: u32 = numeric_input(&runtime_blob.comunication.msg);
    if tmp >= runtime_blob.settings.min_range {
        runtime_blob.settings.max_range = tmp;
    };
    cls_title();
    runtime_blob
}

//Demande le nombre de tentatives que le joueur aimerait avoir.
pub fn game_tries(mut runtime_blob: RuntimeFunctionBlob) -> RuntimeFunctionBlob {
    runtime_blob.comunication.msg = format!(
        "How many attempts do you want?\nCurrent:\t{}",
        runtime_blob.settings.max_tries
    );
    let tmp = numeric_input(&runtime_blob.comunication.msg);
    if tmp >= runtime_blob.settings.min_tries {
        runtime_blob.settings.max_tries = tmp;
    };
    cls_title();
    runtime_blob
}

//Demande si le joueur souhaite avoir des indices quand il joue.
pub fn game_hint(mut runtime_blob: RuntimeFunctionBlob) -> RuntimeFunctionBlob {
    //Initialisation des vars, constantes et plages si applicable.
    let mut wrong: bool = false; //Définit la var "wrong".

    runtime_blob.settings.guess_hint = loop {
        runtime_blob.comunication.msg = format!(
            "Do you want to get hints while playing? (Y/N)\nCurrent:\t{}",
            runtime_blob.settings.guess_hint
        );

        //Affiche la var "option_hint" et demande si le joueur veux la changer.
        runtime_blob.comunication.user_in_alpha =
            yes_no_else_input(&runtime_blob.comunication, &wrong);

        match runtime_blob.comunication.user_in_alpha.as_str() {
            //Retourne à la liste des options et indique que le joueur ne veux pas d'indices.
            "n" | "N" | "0" | "false" | "False" | "f" | "F" => {
                cls_title();
                break false;
            }
            //Retourne à la liste des options et indique que le joueur veux des indices.
            "y" | "Y" | "1" | "true" | "True" | "t" | "T" => {
                cls_title();
                break true;
            }
            "" => {
                cls_title();
                runtime_blob.comunication.msg = String::new();
                break runtime_blob.settings.guess_hint;
            }
            //Atrappe touts les autres inputs et indique qu'ils sont incorrect.
            _ => {
                cls_title();
                wrong = true;
            }
        }
    };
    runtime_blob
}
