use std::{ env, fs::{create_dir, create_dir_all}, path::{Path, PathBuf} };
use clap::Parser;
use colored::Colorize;
use polib::{message::Message, mo_file, po_file};

mod csv_struct;
use csv_struct::{Ship, Text};

#[derive(Debug, Parser)]
struct Args {
    po_path: String,

    /// Replace ship names via a .csv template
    #[arg(long)]
    ships: Option<String>,

    /// Translate in game text
    #[arg(long)]
    translate: Option<String>,

    /// Build a structured, ready to plug in mod folder
    #[arg(short, long)]
    build: bool
}

/// Creates 'out' dir if it doesn't already exist.
fn create_out_dir() -> PathBuf {
    let mut exe_dir = env::current_exe().expect("Failed to get current executable path");
    exe_dir.pop();

    let out_dir = exe_dir.as_path().join("out");

    match out_dir.try_exists() {
        Ok(boolean) => {
            if !boolean {
                create_dir(&out_dir).expect("Error creating 'out' directory");
            }

            return out_dir;
        },
        Err(e) => {
            panic!("Error while checking for 'out' directory: {:?}", e.raw_os_error());
        }
    }
}

fn main() {
    let args = Args::parse();
    let out_path = create_out_dir();
    let mod_out: Option<PathBuf> = match args.build {
        false => None,
        true => {
            let path = out_path.join("texts/en/LC_MESSAGES");
            create_dir_all(&path).unwrap();
            Some(path)
        }
    };

    let po_path = Path::new(&args.po_path);
    if !po_path.exists() {
        eprintln!("Argument 1 is not a valid path");
        return;
    }

    let mut catalog = po_file::parse(&po_path).unwrap();

    if args.ships.is_some() {
        let str = args.ships.unwrap();
        let path = Path::new(&str);
        let mut ships_csv = csv::Reader::from_path(&path).expect("Failed to read .csv file");

        for result in ships_csv.deserialize() {
            let ship: Ship = result.unwrap();

            match ship.correct_struct() {
                Err(why) => eprintln!("WARNING: {}", why),
                Ok(()) => {
                    println!("{}: Replacing '{}' with '{}'", ship.ids, ship.original, ship.replacement_full);
                    let ids = format!("IDS_{}", ship.ids);

                    if catalog.find_message(None, &ids, None).is_none() {
                        eprintln!("{}", format!("ERROR: No ships found with IDS '{}'", ship.ids).red().to_string());
                        continue;
                    }

                    catalog.append_or_update(Message::build_singular()
                        .with_msgid(format!("IDS_{}_FULL", ship.ids))
                        .with_msgstr(ship.replacement_full)
                        .done());

                    if ship.replacement_short.is_none() {
                        continue;
                    }

                    println!("With short name '{}'", ship.replacement_short.as_ref().unwrap());
                    catalog.append_or_update(Message::build_singular()
                        .with_msgid(ids)
                        .with_msgstr(ship.replacement_short.unwrap())
                        .done());
                }
            }
        }
    } else if args.translate.is_some() {
        let str = args.translate.unwrap();
        let path = Path::new(&str);
        let mut translations_csv = csv::Reader::from_path(&path).expect("Failed to read .csv file");

        for res in translations_csv.deserialize() {
            let translation: Text = res.unwrap();

            match translation.correct_struct() {
                Err(why) => eprintln!("WARNING: {}", why),
                Ok(()) => {
                    println!("Replacing text from IDS '{}' with '{}'", translation.ids, translation.replacement);
                    let ids = translation.ids;

                    if catalog.find_message(None, &ids, None).is_none() {
                        eprintln!("{}", format!("ERROR: No text found with IDS '{}'", ids).red().to_string());
                        continue;
                    }

                    catalog.append_or_update(Message::build_singular()
                        .with_msgid(ids)
                        .with_msgstr(translation.replacement)
                        .done());
                }
            }
        }
    }

    po_file::write_to_file(&catalog, &out_path.join("updated.po")).unwrap();
    mo_file::write(&catalog, &out_path.join("output_global.mo")).unwrap();

    if mod_out.is_some() {
        mo_file::write(&catalog, &mod_out.unwrap().join("global.mo")).unwrap();
    }

    println!("Completed");
}
