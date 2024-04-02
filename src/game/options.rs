//Initialisation des "crates" ou des librairies suplémentaires nécessaires.
use crate::{
    game::game_logic::{
        cls_scr::cls_title,
        questions::{
            numeric_input,
            yes_no_else_input,
        },
    },
    Settings,
};
use std::{
    fmt::Display,
    fs::{
        read_to_string,
        write,
        OpenOptions,
    },
    u32,
};

//Génère le fichier d'options s'il n'existe pas et le lit.
pub fn default_settings() -> Settings {

    //Initialisation des vars, constantes et plages si applicable.

    let mut settings: Settings = Settings {
        max_range: 100,
        min_range: 1,
        max_tries: 10,
        guess_hint: true,
        settings_count: 4,

        stop: false,
        first_cycle: true,
        
        msg: String::new(),
        err_name: String::new(),
        err_msg: String::new(),
    };
    let mut settings_raw: String;
    let mut loop_count;
    
    //let err_name: str;

    //let err_msg: str;


    settings.msg = format!(
        "//This file contains the settings for the Number Guessing Game.\n\n{}\n\n{}\n\n{}\n\n{}",

        "//Up to what number do you want to guess?\n100",
        
        "//From what number do you want to guess?\n1",
        
        "//How many atempts do you want to guess the random number?\n10",
        
        "//Do you want hints in your game?\ntrue"
    );

    let read_err_msg: String = format!(
        "Settings file could not be read.\n{}\n{}",
        "If the file is being automatically removed by your anti-virus,",
        "please add an exception to it for this game."
    );

    let write_err_msg: String = format!(
        "Settings file already exists or couldn't be created.\n{}",
        "If the game isn't in a writable directory, please move it."
    );
    
    //
    if let Ok(settings_raw) = read_to_string("Settings.txt") {

        //
        let mut settings_raw = settings_raw.lines();

        //
        for loop_count in settings.settings_count..=1 {
            
            //Concatène le message d'erreur de la lecture d'un fichier d'options déja présent.
            (settings.err_name,settings.err_msg) = { 
                match loop_count {
                    1 => ("max_range","a number from 1 to 4'294'967'295"),
                    2 => ("min_range","a number from 0 to 4'294'967'294"),
                    3 => ("max_tries","a number from 1 to 4'294'967'295"),
                    4 => ("guess_hint","'true' or 'false'"),
                }
            };

            settings.guess_hint = if settings_raw
            .next_back()
            .unwrap()
            .parse::<bool>()
            .expect(
                format!(
                    "'{}' should be {}.",settings.err_name,settings.err_msg
                )
                .as_str()
            ) else {

            };
            for _ in 1..=2 {settings_raw.next_back();};

            let settings = settings_raw.next().unwrap();

            if let Ok(settings) = settings_raw
            .to_string()
            .parse::<u32>()
            .expect(
                format!(
                    "'{}' should be {}.",
                    err_name,
                    err_msg,
                )
                .as_str()
            );
            let Err(settings) = settings
            .parse()
            .expect(
                format!(
                    "'{}' should be {}.",
                    err_name,
                    err_msg,
                )
                .as_str()
            );
        };

    } else {

        //Écrit le contenu de la var "msg" dans le fichier "Settings.txt" 
        //et affiche le contenu de la var "write_err_msg" si une erreur est encontré.
        if let Ok(settings_raw) = OpenOptions::new()
        .write(true)
        .create(true)
        .open("Settings.txt") {
            write("Settings.txt", settings.msg)
            .expect("Settings.txt");
        } else {
            println!("{}",read_err_msg);
            };

    };

    return (max_range, min_range, max_tries, guess_hint);
}

//Demande la taille de la plage numérique à chercher souhaité.
pub fn game_size(mut settings: Settings) -> Settings {
    settings.max_range = format!("Input the largest number you want.\nCurrent:\t{}",settings.max_range);
    settings.max_range = numeric_input(settings.msg);
    cls_title();
    settings
}

//Demande le nombre de tentatives que le joueur aimerait avoir.
pub fn game_tries(mut settings: Settings) -> Settings {
    settings.max_tries = numeric_input(settings.msg);
    settings.msg = format!("How many attempts do you want?\nCurrent:\t{}",settings.max_tries);
    cls_title();
    settings
}

//Demande si le joueur souhaite avoir des indices quand il joue.
pub fn game_hint(mut settings: Settings) -> Settings {

    //Initialisation des vars, constantes et plages si applicable.
    let mut wrong: bool = false; //Définit la var "wrong".
    let mut input: String = String::new(); //Définit la var "input".

    settings.guess_hint = loop {
        settings.msg = format!("Do you want to get hints while playing? (Y/N)\nCurrent:\t{}",settings.guess_hint);
        
        //Affiche la var "option_hint" et demande si le joueur veux la changer.
        input = yes_no_else_input(settings.msg, input, wrong);
            
        match input.as_str() {
            //Retourne à la liste des options et indique que le joueur ne veux pas d'indices.
            "n" | "N" | "0" | "false" | "False" | "f" | "F" => {
                cls_title();
                break false;
            },
            //Retourne à la liste des options et indique que le joueur veux des indices.
            "y" | "Y" | "1" | "true" | "True" | "t" | "T" => {
                cls_title();
                break true;
            },
            //Atrappe touts les autres inputs et indique qu'ils sont incorrect.
            _ => {
                cls_title();
                wrong = true;
            },
        }
    };
    return settings
}