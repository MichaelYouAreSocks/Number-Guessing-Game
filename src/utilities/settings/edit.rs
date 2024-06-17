use super::settings_layout::settings_layout;
use crate::{utilities::errors::error_handling, RuntimeFunctionBlob};
use std::fs::write;

pub fn save_setting_to_file(mut runtime_blob: RuntimeFunctionBlob) -> RuntimeFunctionBlob {
    match write(
        &runtime_blob.core_functions.settings_file_path,
        settings_layout(&runtime_blob.settings),
    ) {
        Ok(_) => runtime_blob,
        Err(_) => {
            runtime_blob.core_functions.error_handler = error_handling(020);
            runtime_blob.core_functions.stop = true;
            runtime_blob
        }
    }
}
