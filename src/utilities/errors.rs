use crate::ErrFormat;

pub fn error_handling(error_code: u8) -> ErrFormat {
    match error_code/10 {
        1 => {
            return ErrFormat {
                code: error_code,
                name: format!("Reading File"),
                msg: format!(
                    "{} could nor be read.\n{}\n{}",
                    match error_code {
                        10 => "Settings file",
                        11 => "Score file",
                        _ => todo!()
                    },
                    "If the file is being automativally remoced by your anti-virus,",
                    "pleade add an exception to it for the game to work."
                )
            }
        },
        2 => {
            ErrFormat {
                code: error_code,
                name: format!("Writing File"),
                msg: format!(
                    "{} couldn't be created or modified.\n{}",
                    match error_code {
                        20 => "Settings file",
                        21 => "Score file",
                        _ => todo!()
                    },
                    "If the game isn't in a writable directory, please move it."
                )
            }
        },
        _ => {
            ErrFormat {
                code: 255,
                name: format!("New Error!"),
                msg: format!(
                    "This error has never been seen before!\n{}\n{}",
                    "If you see this message, you managed to break the game in a way I didn't manage to forsee as possible.",
                    "Please open an issue on github with a screenshot of this message to know what to do next."
                )
            }
        }
    }
}

pub fn err_print(error_handler: &ErrFormat) {
    println!(
        "Error code : \n{}\n\n{}\n{}\n\n{}\n{}",
        error_handler.code, "Error :", error_handler.name, "Probable cause : ", error_handler.msg,
    );
}
