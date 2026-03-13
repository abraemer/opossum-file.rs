use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use tracing::info;
use zip::ZipArchive;

use crate::core::entities::opossum::Opossum;
use crate::core::services::input_reader::InputReader;
use crate::error::OpossumError;

use super::convert::convert_to_opossum;
use super::entities::{OpossumInputFileModel, OpossumOutputFileModel};

const INPUT_JSON_NAME: &str = "input.json";
const OUTPUT_JSON_NAME: &str = "output.json";

pub struct OpossumFileReader {
    path: PathBuf,
}

impl OpossumFileReader {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }
}

impl InputReader for OpossumFileReader {
    fn read(&self) -> Result<Opossum, OpossumError> {
        let input_file = self.read_opossum_file()?;
        Ok(convert_to_opossum(
            input_file.input_file,
            input_file.output_file,
        ))
    }
}

struct OpossumFileModel {
    input_file: OpossumInputFileModel,
    output_file: Option<OpossumOutputFileModel>,
}

impl OpossumFileReader {
    fn read_opossum_file(&self) -> Result<OpossumFileModel, OpossumError> {
        info!("Converting opossum to opossum {:?}", self.path);

        let file = File::open(&self.path)?;
        let mut archive = ZipArchive::new(file)?;

        let input_file = self.read_input_json(&mut archive)?;
        let output_file = self.read_output_json_if_exists(&mut archive)?;

        Ok(OpossumFileModel {
            input_file,
            output_file,
        })
    }

    fn read_input_json(
        &self,
        archive: &mut ZipArchive<File>,
    ) -> Result<OpossumInputFileModel, OpossumError> {
        let result = archive.by_name(INPUT_JSON_NAME);
        match result {
            Ok(mut input_file) => {
                let mut contents = String::new();
                input_file.read_to_string(&mut contents)?;
                let input: OpossumInputFileModel = serde_json::from_str(&contents)?;
                Ok(input)
            }
            Err(zip::result::ZipError::FileNotFound) => Err(OpossumError::ParseError(format!(
                "Opossum file {:?} is corrupt and does not contain '{}'",
                self.path, INPUT_JSON_NAME
            ))),
            Err(e) => Err(e.into()),
        }
    }

    fn read_output_json_if_exists(
        &self,
        archive: &mut ZipArchive<File>,
    ) -> Result<Option<OpossumOutputFileModel>, OpossumError> {
        match archive.by_name(OUTPUT_JSON_NAME) {
            Ok(mut output_file) => {
                let mut contents = String::new();
                output_file.read_to_string(&mut contents)?;
                let output: OpossumOutputFileModel = serde_json::from_str(&contents)?;
                Ok(Some(output))
            }
            Err(zip::result::ZipError::FileNotFound) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}
