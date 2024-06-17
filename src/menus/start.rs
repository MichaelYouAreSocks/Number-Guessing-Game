//Initialisation des "crates" ou des librairies suplémentaires nécessaires.
use crate::{
    game::game,
    menus::{options::options_menu, scores::show_score_board, start_layout::start_menu_layout},
    utilities::{cls_scr::cls_title, questions::yes_no_else_input},
    RuntimeFunctionBlob,
};

pub fn main_menu(
    mut runtime_blob: RuntimeFunctionBlob,
    high_scores: &Vec<String>,
) -> RuntimeFunctionBlob {
    //Initialisation des vars, constantes et plages si applicable.
    let mut wrong: bool = false; //Définit la var "wrong".
    let mut end_game_msg: String = String::new();

    loop {
        //Affiche un message différant dépendant de si le joueur joue plus d'une fois.
        runtime_blob.comunication.msg = start_menu_layout(&runtime_blob, high_scores);

        runtime_blob.comunication.user_in_alpha =
            yes_no_else_input(&runtime_blob.comunication, &wrong);

        //Laisse l'utilisateur choisir s'il veux jouer.
        match runtime_blob.comunication.user_in_alpha.as_str() {
            //Quite le jeu.
            "n" | "N" | "q" | "Q" | "0" => {
                cls_title();
                //Affiche un message dépandant de si le joueur à joué au minimum une partie avant de quiter le jeu.
                if &runtime_blob.core_functions.first_cycle != &true {
                    println!("Thanks for playing!");
                } else {
                    println!("Hope you'll play soon!")
                };
                runtime_blob.core_functions.stop = true;
                break runtime_blob;
            }
            //Joue au jeu.
            "y" | "Y" | "" | "1" => {
                cls_title();
                //Lance le jeu et transmet toutes les variables utiles à ce dernier.
                runtime_blob = game(runtime_blob);
                //Controle si c'est la première partie du joueur et indique que ce n'est plus la première si c'est le cas.
                runtime_blob.core_functions.first_cycle =
                    match runtime_blob.comunication.msg.as_str() {
                        "" => true,
                        _ => {
                            end_game_msg = runtime_blob.comunication.msg.clone();
                            false
                        }
                    };
            }
            //Affiche les oprions.
            "o" | "O" | "s" | "S" | "2" => {
                cls_title();
                runtime_blob = options_menu(runtime_blob);
                runtime_blob.comunication.msg = end_game_msg.to_owned()
            }
            //Affiche le scorboard.
            "b" | "B" | "r" | "R" | "3" => {
                cls_title();
                show_score_board(high_scores);
                runtime_blob.comunication.msg = end_game_msg.to_owned()
            }
            //Atrappe tout les autres inputs et indique qu'ils sont incorrect.
            _ => {
                cls_title();
                wrong = true
            }
        };
    }
}
