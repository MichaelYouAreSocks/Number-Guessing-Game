use crate::RuntimeFunctionBlob;

pub fn start_menu_layout(runtime_blob: &RuntimeFunctionBlob, high_scores: &Vec<String>) -> String {
    //Affiche un message spécifique dépendant de si le joueur joue sa première manche de la partie.
    format!(
        "{}{}{}{}\n{}{}{}{}\n{}{}{}{}\n{}{}{}{}{}",
        "1 -> Play",
        match runtime_blob.comunication.msg.as_str() {
            "" => {
                format!(
                    "{tmp}",
                    tmp = if runtime_blob.core_functions.first_cycle == true {
                        "!\t\t\t"
                    } else {
                        " again?\t"
                    }
                )
            }
            _ => format!("\t\t\t"),
        },
        "1st : ",
        high_scores[0],
        "2 -> Options",
        match runtime_blob.comunication.msg.as_str() {
            //"" => "".to_string(),
            _ => format!("\t\t\t"),
        },
        "2nd : ",
        high_scores[1],
        "3 -> Score Board",
        match runtime_blob.comunication.msg.as_str() {
            //"" => "".to_string(),
            _ => format!("\t\t"),
        },
        "3rd : ",
        high_scores[2],
        "0 -> Quit",
        match runtime_blob.comunication.msg.as_str() {
            //"" => "".to_string(),
            _ => format!("\t\t\t"),
        },
        "4th : ",
        high_scores[4],
        match runtime_blob.comunication.msg.as_str() {
            //"" => "".to_string(),
            _ => format!("\n{}", runtime_blob.comunication.msg),
        },
    )
}
