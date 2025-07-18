use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use encoding_rs::Encoding;
use unicode_normalization::UnicodeNormalization;

/// Pour chaque ligne du FICHIER1, vérifie si elle est présente n'importe où dans le FICHIER2.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Le fichier contenant les lignes à chercher (les "aiguilles").
    #[arg(value_name = "FICHIER1")]
    file1: PathBuf,

    /// Le fichier dans lequel chercher (la "meule de foin").
    #[arg(value_name = "FICHIER2")]
    file2: PathBuf,

    /// Fichier de sortie pour les lignes non trouvées.
    #[arg(short, long, value_name = "FICHIER_SORTIE")]
    output: Option<PathBuf>,

    /// Activer l'affichage des informations de débogage.
    #[arg(long)]
    debug: bool,

    /// Comparer les lignes sur les N premiers caractères.
    #[arg(long, value_name = "N")]
    length: Option<usize>,

    /// Afficher les statistiques de comparaison à la fin.
    #[arg(long)]
    stat: bool,

    /// Affiche les lignes trouvées au lieu des lignes manquantes.
    #[arg(long)]
    found: bool,
}

// Tente de décoder des octets bruts en String en essayant plusieurs encodages courants.
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
        eprintln!("Avertissement: Des erreurs de décodage ont été rencontrées pour le fichier {}. Certains caractères pourraient être incorrects.", path.display());
    }
    Ok(cow.into_owned())
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let start_time = Instant::now();

    // Étape 1: Lire et décoder le contenu complet du fichier 2 en UTF-8 et normaliser.
    let file2_full_content = decode_file_to_string(&args.file2)?;
    let lines_in_file2_count = file2_full_content.lines().count();

    // Étape 2: Stocker les lignes du fichier 2 dans un HashSet pour une recherche rapide.
    let mut lines_in_file2: HashSet<String> = HashSet::new();
    for line in file2_full_content.lines() {
        let processed_line = line.trim().nfc().collect::<String>();
        let final_line = match args.length {
            Some(len) => processed_line.chars().take(len).collect(),
            None => processed_line,
        };
        lines_in_file2.insert(final_line);
    }

    // Étape 3: Compter les lignes du premier fichier pour la barre de progression.
    let file1_content = decode_file_to_string(&args.file1)?;
    let total_lines = file1_content.lines().count() as u64;

    // Étape 4: Configuration de la barre de progression.
    let pb = ProgressBar::new(total_lines);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );
    pb.enable_steady_tick(Duration::from_millis(100));

    // Étape 5: Comparaison des fichiers.
    let mut missing_lines: Vec<String> = Vec::new();
    let mut found_lines: Vec<String> = Vec::new();
    for (i, line) in file1_content.lines().enumerate() {
        let trimmed_and_normalized_line = line.trim().nfc().collect::<String>();
        let line_to_compare = match args.length {
            Some(len) => trimmed_and_normalized_line.chars().take(len).collect(),
            None => trimmed_and_normalized_line.clone(),
        };
        
        if args.debug {
            eprintln!("DEBUG: Ligne {}: '{}'", i, line_to_compare);
            eprintln!("DEBUG: Hex: {:x?}", line_to_compare.as_bytes());
        }

        if !line_to_compare.is_empty() && !lines_in_file2.contains(&line_to_compare) {
            missing_lines.push(line.to_string());
            if args.debug {
                eprintln!("DEBUG: Non trouvée.");
            }
        } else {
            if !line_to_compare.is_empty() {
                found_lines.push(line.to_string());
            }
            if args.debug {
                eprintln!("DEBUG: Trouvée.");
            }
        }
        pb.inc(1);
    }

    pb.finish_and_clear();

    // Étape 6: Écrire le résultat dans la sortie.
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
        println!("\n-- Statistiques --");
        println!("Fichier 1 (aiguilles): {} lignes", total_lines);
        println!("Fichier 2 (meule de foin): {} lignes", lines_in_file2_count);
        println!("Lignes trouvées: {}", found_lines.len());
        println!("Lignes non trouvées: {}", missing_lines.len());
        println!("Temps de traitement: {:?}", duration);
    }

    Ok(())
}