use std::{fs::write, io::read_to_string};

use crate::{
    utilities::{errors::error_handling, file::open::open_and_read_existing_file},
    ErrFormat, RuntimeFunctionBlob,
};

use super::{defaults::default_scores, score_layout::score_board};

pub fn read_score_file(runtime_blob: &RuntimeFunctionBlob) -> Result<Vec<String>, ErrFormat> {
    let RuntimeFunctionBlob {
        comunication: _,
        core_functions,
        settings: _,
    } = runtime_blob;

    let scores_raw: String;

    let mut high_scores: Vec<String> = default_scores();

    match open_and_read_existing_file(&runtime_blob.core_functions.score_board_path) {
        Ok(scores_file) => {
            scores_raw = match read_to_string(scores_file) {
                Ok(scores_raw) => scores_raw,
                Err(_) => return Err(error_handling(021)),
            }
        }

        Err(_) => match write(&core_functions.score_board_path, score_board(&high_scores)) {
            Ok(_) => {
                println!("Default scoring file created and loaded.");
                return Ok(high_scores);
            }
            Err(_) => return Err(error_handling(020)),
        },
    }

    let mut scores_as_lines = scores_raw.lines();

    for _ in 0..9 {
        //
        let imported_score = match scores_as_lines.next() {
            Some(tmp) => tmp,
            None => break,
        };
        high_scores.push(imported_score.to_string());
    }

    Ok(high_scores)
}
