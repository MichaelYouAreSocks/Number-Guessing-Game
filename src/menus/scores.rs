use crate::{
    utilities::{
        cls_scr::cls_title, questions::yes_no_else_input, score_board::score_layout::score_board,
    },
    Comunication,
};

pub fn show_score_board(high_scores: &Vec<String>) {
    let comunication = Comunication {
        msg: score_board(&high_scores),
        user_in_alpha: String::new(),
        user_in_u32: 0,
    };

    let wrong: bool = false;

    match yes_no_else_input(&comunication, &wrong) {
        _ => {
            cls_title();
            return;
        }
    };
}
