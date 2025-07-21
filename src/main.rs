use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use encoding_rs::Encoding;
use unicode_normalization::UnicodeNormalization;

/// For each line in file1, check if it is present anywhere in file2.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The file containing the lines to search for (the "needles").
    #[arg(value_name = "file1")]
    file1: PathBuf,

    /// The file to search within (the "haystack").
    #[arg(value_name = "file2")]
    file2: PathBuf,

    /// Output file for lines not found.
    #[arg(short, long, value_name = "ouput")]
    output: Option<PathBuf>,

    /// Enable debug information display.
    #[arg(long)]
    debug: bool,

    /// Compare lines based on the first N characters.
    #[arg(long, value_name = "N")]
    length: Option<usize>,

    /// Display comparison statistics at the end.
    #[arg(long)]
    stat: bool,

    /// Display found lines instead of missing lines.
    #[arg(long)]
    found: bool,
}

// Attempts to decode raw bytes into a String by trying several common encodings.
fn decode_file_to_string(path: &PathBuf) -> io::Result<String> {
    let bytes = std::fs::read(path)?;

    let (cow, _encoding, had_errors) = if let Some((detected_encoding, _bom_len)) = Encoding::for_bom(&bytes) {
        // BOM detected, use the detected encoding
        let (cow_str, had_errors_decode) = detected_encoding.decode_with_bom_removal(&bytes);
        (cow_str, detected_encoding, had_errors_decode)
    } else {
        // No BOM detected, try common encodings
        let (cow_utf8, had_errors_utf8) = encoding_rs::UTF_8.decode_with_bom_removal(&bytes);
        if !had_errors_utf8 {
            (cow_utf8, encoding_rs::UTF_8, had_errors_utf8)
        } else {
            let (cow_utf16le, had_errors_utf16le) = encoding_rs::UTF_16LE.decode_with_bom_removal(&bytes);
            if !had_errors_utf16le {
                (cow_utf16le, encoding_rs::UTF_16LE, had_errors_utf16le)
            } else {
                let (cow_utf16be, had_errors_utf16be) = encoding_rs::UTF_16BE.decode_with_bom_removal(&bytes);
                if !had_errors_utf16be {
                    (cow_utf16be, encoding_rs::UTF_16BE, had_errors_utf16be)
                } else {
                    // Fallback: use UTF-8 with replacements and print a warning
                    let (cow_fallback, had_errors_fallback) = encoding_rs::UTF_8.decode_with_bom_removal(&bytes);
                    (cow_fallback, encoding_rs::UTF_8, had_errors_fallback)
                }
            }
        }
    };

    if had_errors {
        eprintln!("Warning: Decoding errors were encountered for file {}. Some characters might be incorrect.", path.display());
    }
    Ok(cow.into_owned())
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let start_time = Instant::now();

    // Step 1: Read and decode the full content of file 2 to UTF-8 and normalize.
    let file2_full_content = decode_file_to_string(&args.file2)?;
    let lines_in_file2_count = file2_full_content.lines().count();

    // Step 2: Store file 2 lines in a HashSet for quick lookup.
    let mut lines_in_file2: HashSet<String> = HashSet::new();
    for line in file2_full_content.lines() {
        let processed_line = line.trim().nfc().collect::<String>();
        let final_line = match args.length {
            Some(len) => processed_line.chars().take(len).collect(),
            None => processed_line,
        };
        lines_in_file2.insert(final_line);
    }

    // Step 3: Count lines in the first file for the progress bar.
    let file1_content = decode_file_to_string(&args.file1)?;
    let total_lines = file1_content.lines().count() as u64;

    // Step 4: Progress bar configuration.
    let pb = ProgressBar::new(total_lines);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );
    pb.enable_steady_tick(Duration::from_millis(100));

    // Step 5: File comparison.
    let mut missing_lines: Vec<String> = Vec::new();
    let mut found_lines: Vec<String> = Vec::new();
    for (i, line) in file1_content.lines().enumerate() {
        let trimmed_and_normalized_line = line.trim().nfc().collect::<String>();
        let line_to_compare = match args.length {
            Some(len) => trimmed_and_normalized_line.chars().take(len).collect(),
            None => trimmed_and_normalized_line.clone(),
        };
        
        if args.debug {
            eprintln!("DEBUG: Line {}: '{}'", i, line_to_compare);
            eprintln!("DEBUG: Hex: {:x?}", line_to_compare.as_bytes());
        }

        if !line_to_compare.is_empty() && !lines_in_file2.contains(&line_to_compare) {
            missing_lines.push(line.to_string());
            if args.debug {
                eprintln!("DEBUG: Not found.");
            }
        } else {
            if !line_to_compare.is_empty() {
                found_lines.push(line.to_string());
            }
            if args.debug {
                eprintln!("DEBUG: Found.");
            }
        }
        pb.inc(1);
    }

    pb.finish_and_clear();

    // Step 6: Write the result to the output.
    let lines_to_output = if args.found {
        &found_lines
    } else {
        &missing_lines
    };

    if let Some(output_path) = args.output {
        let mut output_file = File::create(output_path)?;
        for line in lines_to_output {
            writeln!(output_file, "{}", line)?;
        }
    } else {
        for line in lines_to_output {
            println!("{}", line);
        }
    }

    if args.stat {
        let duration = start_time.elapsed();
        println!("\n-- Statistics --");
        println!("File 1 (needles): {} lines", total_lines);
        println!("File 2 (haystack): {} lines", lines_in_file2_count);
        println!("Lines found: {}", found_lines.len());
        println!("Lines not found: {}", missing_lines.len());
        println!("Processing time: {:?}", duration);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static TEST_DIR_COUNTER: AtomicUsize = AtomicUsize::new(0);

    // Creates a unique temporary directory for each test.
    // Returns the path to the directory.
    fn setup_test_dir() -> PathBuf {
        let pid = std::process::id();
        let count = TEST_DIR_COUNTER.fetch_add(1, Ordering::SeqCst);
        let temp_dir = std::env::temp_dir().join(format!("compar_tests_{}_{}", pid, count));
        fs::create_dir_all(&temp_dir).unwrap();
        temp_dir
    }

    // Cleans up the test directory.
    fn teardown_test_dir(temp_dir: &PathBuf) {
        fs::remove_dir_all(temp_dir).unwrap();
    }

    #[test]
    fn test_decode_utf8_no_bom() {
        let temp_dir = setup_test_dir();
        let file_path = temp_dir.join("utf8_no_bom.txt");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"hello world").unwrap();

        let content = decode_file_to_string(&file_path).unwrap();
        assert_eq!(content, "hello world");
        teardown_test_dir(&temp_dir);
    }

    #[test]
    fn test_decode_utf8_with_bom() {
        let temp_dir = setup_test_dir();
        let file_path = temp_dir.join("utf8_with_bom.txt");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"\xEF\xBB\xBFhello world").unwrap();

        let content = decode_file_to_string(&file_path).unwrap();
        assert_eq!(content, "hello world");
        teardown_test_dir(&temp_dir);
    }

    #[test]
    fn test_decode_utf16le_with_bom() {
        let temp_dir = setup_test_dir();
        let file_path = temp_dir.join("utf16le_with_bom.txt");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"\xFF\xFEh\x00e\x00l\x00l\x00o\x00").unwrap();

        let content = decode_file_to_string(&file_path).unwrap();
        assert_eq!(content, "hello");
        teardown_test_dir(&temp_dir);
    }

    #[test]
    fn test_decode_utf16be_with_bom() {
        let temp_dir = setup_test_dir();
        let file_path = temp_dir.join("utf16be_with_bom.txt");
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"\xFE\xFF\x00h\x00e\x00l\x00l\x00o").unwrap();

        let content = decode_file_to_string(&file_path).unwrap();
        assert_eq!(content, "hello");
        teardown_test_dir(&temp_dir);
    }

    #[test]
    fn test_line_comparison_with_length() {
        let line1 = "abcde123";
        let line2 = "abcde456";

        let processed_line1 = line1.trim().nfc().collect::<String>();
        let processed_line2 = line2.trim().nfc().collect::<String>();

        let final_line1 = processed_line1.chars().take(5).collect::<String>();
        let final_line2 = processed_line2.chars().take(5).collect::<String>();

        assert_eq!(final_line1, "abcde");
        assert_eq!(final_line2, "abcde");
        assert_eq!(final_line1, final_line2);
    }
}