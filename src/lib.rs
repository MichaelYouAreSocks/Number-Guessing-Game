pub mod game;
pub mod menus;
pub mod utilities;

//
#[derive(Clone)]
pub struct Settings {
    pub max_range: u32,     //le plus grand numéro de la plâge à chercher.
    pub min_range: u32,     //Le plus petit numéro de la plâge à chercher.
    pub max_tries: u32,     //Quantité d'essais manqué avant la fin du jeu.
    pub min_tries: u32,     //Quantité d'éssais minimum autorisé par le jeu.
    pub guess_hint: bool,   //Affiche un indice avec la plâge numérique restante à chercher.
    pub settings_count: u8, //Quantité de variables qui sont des options pour le joueur.
}

//
#[derive(Clone, Debug)]
pub struct ErrFormat {
    pub code: u8,     //
    pub name: String, //
    pub msg: String,  //
}

//
#[derive(Clone)]
pub struct CoreFunctions {
    pub first_cycle: bool,          //Détecte si le joueur joue sa première partie.
    pub stop: bool,                 //Permet de quiter le jeu.
    pub error_handler: ErrFormat,   //Concatène tout ce qui se rapport à une erreure.
    pub settings_file_path: String, //Contien l'emplacement par défaut du fichier de paramêtres.
    pub score_board_path: String,   //Contien l'emplacement par défaut du fichier de scores.
}

//
#[derive(Clone)]
pub struct Comunication {
    pub msg: String,           //Concatène les messages pour l'utilisateur.
    pub user_in_alpha: String, //Concatène les inputs alphabêtiques de l'utilisateur.
    pub user_in_u32: u32,      //Concatène les inputs numériques de l'utilisateur.
}

#[derive(Clone)]
pub struct RuntimeFunctionBlob {
    pub settings: Settings,            //
    pub core_functions: CoreFunctions, //
    pub comunication: Comunication,    //
}
