use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use clap::{Parser, Subcommand};
use opossum_lib::{
    merge_opossums, write_opossum_file, InputReader, OpossumFileReader,
    OwaspDependencyScanFileReader, ScanCodeFileReader,
};
use tracing::info;

#[derive(Parser)]
#[command(name = "opossum-file")]
#[command(about = "Convert scan results to Opossum format", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Generate an Opossum file from various input formats")]
    Generate {
        #[arg(long, help = "Path to ScanCode JSON file (can be repeated)")]
        scan_code_json: Vec<PathBuf>,

        #[arg(long, help = "Path to .opossum file (can be repeated)")]
        opossum_file: Vec<PathBuf>,

        #[arg(
            long,
            help = "Path to OWASP dependency check JSON file (can be repeated)"
        )]
        dependency_check_json: Vec<PathBuf>,

        #[arg(short, long, help = "Output file path")]
        output: PathBuf,

        #[arg(long, help = "Project title")]
        project_title: Option<String>,

        #[arg(long, help = "Project ID")]
        project_id: Option<String>,
    },
    #[command(about = "Merge multiple .opossum files into one")]
    Merge {
        #[arg(required = true, help = "Input .opossum files to merge")]
        input_files: Vec<PathBuf>,

        #[arg(short, long, help = "Output file path")]
        output: PathBuf,
    },
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            scan_code_json,
            opossum_file,
            dependency_check_json,
            output,
            project_title: _,
            project_id: _,
        } => {
            run_generate(scan_code_json, opossum_file, dependency_check_json, output)?;
        }
        Commands::Merge {
            input_files,
            output,
        } => {
            run_merge(input_files, output)?;
        }
    }

    Ok(())
}

fn run_generate(
    scan_code_json: Vec<PathBuf>,
    opossum_file: Vec<PathBuf>,
    dependency_check_json: Vec<PathBuf>,
    output: PathBuf,
) -> Result<()> {
    let total_files = scan_code_json.len() + opossum_file.len() + dependency_check_json.len();

    if total_files == 0 {
        info!("No input provided. Exiting.");
        return Ok(());
    }

    let mut opossums = Vec::new();

    for path in &scan_code_json {
        info!("Reading ScanCode JSON: {:?}", path);
        let reader = ScanCodeFileReader::from_file(path)
            .with_context(|| format!("Failed to read ScanCode file: {:?}", path))?;
        let opossum = reader
            .read()
            .with_context(|| format!("Failed to parse ScanCode file: {:?}", path))?;
        opossums.push(opossum);
    }

    for path in &opossum_file {
        info!("Reading Opossum file: {:?}", path);
        let reader = OpossumFileReader::new(path);
        let opossum = reader
            .read()
            .with_context(|| format!("Failed to read Opossum file: {:?}", path))?;
        opossums.push(opossum);
    }

    for path in &dependency_check_json {
        info!("Reading OWASP dependency check JSON: {:?}", path);
        let reader = OwaspDependencyScanFileReader::from_file(path)
            .with_context(|| format!("Failed to read OWASP file: {:?}", path))?;
        let opossum = reader
            .read()
            .with_context(|| format!("Failed to parse OWASP file: {:?}", path))?;
        opossums.push(opossum);
    }

    let final_opossum = if opossums.len() == 1 {
        opossums.remove(0)
    } else {
        merge_opossums(opossums).map_err(|e| anyhow::anyhow!("{}", e))?
    };

    info!("Writing output to: {:?}", output);
    write_opossum_file(&final_opossum, &output)
        .with_context(|| format!("Failed to write output file: {:?}", output))?;

    info!("Successfully generated {:?}", output);
    Ok(())
}

fn run_merge(input_files: Vec<PathBuf>, output: PathBuf) -> Result<()> {
    if input_files.len() < 2 {
        bail!(
            "Merge requires at least 2 input files. Got: {}",
            input_files.len()
        );
    }

    let mut opossums = Vec::new();

    for path in &input_files {
        info!("Reading Opossum file: {:?}", path);
        let reader = OpossumFileReader::new(path);
        let opossum = reader
            .read()
            .with_context(|| format!("Failed to read Opossum file: {:?}", path))?;
        opossums.push(opossum);
    }

    let merged = merge_opossums(opossums).map_err(|e| anyhow::anyhow!("{}", e))?;

    info!("Writing output to: {:?}", output);
    write_opossum_file(&merged, &output)
        .with_context(|| format!("Failed to write output file: {:?}", output))?;

    info!("Successfully merged to {:?}", output);
    Ok(())
}
