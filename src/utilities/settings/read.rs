use std::str::Lines;

use crate::{ErrFormat, RuntimeFunctionBlob};

pub fn settings_importer(
    settings_raw: String,
    mut runtime_blob: RuntimeFunctionBlob,
) -> (u8, RuntimeFunctionBlob) {
    let RuntimeFunctionBlob {
        mut settings,
        mut core_functions,
        comunication,
    } = runtime_blob;

    let mut imported_settings: u8 = 0;
    let settings_line_count: usize;
    let mut settings_as_lines: Lines;

    settings_as_lines = settings_raw.lines();

    settings_line_count = settings_raw.as_str().lines().count();
    //
    for lines_searched in 0..settings_line_count {
        //
        let individual_setting = match settings_as_lines.next() {
            Some(tmp) => tmp,
            None => break,
        };
        //
        core_functions.error_handler = match imported_settings {
            0 => {
                //
                if let Ok(tmp) = individual_setting.trim().parse::<u32>() {
                    settings.max_range = tmp;
                    //
                    imported_settings = imported_settings + 1;
                };
                //
                if lines_searched > 5 {
                    break;
                } else {
                    ErrFormat {
                        code: 010,
                        name: String::from("Max_range"),
                        msg: String::from("a number from 1 to 4'294'967'295"),
                    }
                }
            }
            1 => {
                //
                if let Ok(tmp) = individual_setting.trim().parse::<u32>() {
                    settings.min_range = tmp;
                    //
                    imported_settings = imported_settings + 1;
                };
                //
                if lines_searched > 8 {
                    break;
                } else {
                    ErrFormat {
                        code: 011,
                        name: String::from("Min_range"),
                        msg: String::from("a number from 0 to 4'294'967'294"),
                    }
                }
            }
            2 => {
                //
                if let Ok(tmp) = individual_setting.trim().parse::<u32>() {
                    settings.max_tries = tmp;
                    //
                    imported_settings = imported_settings + 1;
                };
                //
                if lines_searched > 11 {
                    break;
                } else {
                    ErrFormat {
                        code: 012,
                        name: String::from("Max_tries"),
                        msg: String::from("a number from 1 to 4'294'967'295"),
                    }
                }
            }
            3 => {
                //
                if let Ok(tmp) = individual_setting.trim().parse::<u32>() {
                    settings.min_tries = tmp;
                    //
                    imported_settings = imported_settings + 1;
                };
                //
                if lines_searched > 14 {
                    break;
                } else {
                    ErrFormat {
                        code: 013,
                        name: String::from("Min_tries"),
                        msg: String::from("a number from 1 to 4'294'967'295"),
                    }
                }
            }
            4 => {
                //
                if let Ok(tmp) = individual_setting.trim().parse::<bool>() {
                    settings.guess_hint = tmp;
                    //
                    imported_settings = imported_settings + 1;
                };
                //
                if lines_searched > 17 {
                    break;
                } else {
                    ErrFormat {
                        code: 014,
                        name: String::from("Guess_hint"),
                        msg: String::from("'true' or 'false'"),
                    }
                }
            }
            _ => core_functions.error_handler,
        };
        //
        if imported_settings == settings.settings_count {
            break;
        } else {
            continue;
        };
    }

    runtime_blob = RuntimeFunctionBlob {
        settings,
        core_functions,
        comunication,
    };

    (imported_settings, runtime_blob)
}
