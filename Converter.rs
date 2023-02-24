use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::Path;
use std::process;
use std::fs;
use std::ffi::OsString;

fn extract_attachments(file_path: &Path) -> io::Result<()> {
    let file = File::open(file_path)?;
    let mut tnef = ytnef::Tnef::from_reader(file)?;
    let output_dir = file_path.parent().unwrap().join("ficheros winmail en pdf");
    fs::create_dir_all(&output_dir)?;
    for attachment in tnef.attachments() {
        let output_path = output_dir.join(attachment.name());
        let mut output_file = File::create(output_path)?;
        output_file.write_all(attachment.data())?;
        println!("Archivo '{:?}' guardado correctamente.", attachment.name());
    }
    Ok(())
}

fn convert_to_pdf(file_path: &Path) -> io::Result<()> {
    let file = File::open(file_path)?;
    let mut tnef = ytnef::Tnef::from_reader(file)?;
    for attachment in tnef.attachments() {
        if attachment.name().ends_with(".pdf") {
            let mut pdf_name = String::new();
            println!("Introduce el nombre del archivo PDF generado:");
            io::stdin().read_line(&mut pdf_name)?;
            let pdf_path = file_path.parent().unwrap().join("ficheros winmail en pdf").join(pdf_name.trim_end_matches('\n')).with_extension("pdf");
            let mut pdf_file = File::create(pdf_path)?;
            pdf_file.write_all(attachment.data())?;
            println!("Archivo '{:?}' creado correctamente.", pdf_path);
            return Ok(())
        }
    }
    println!("No se encontraron archivos adjuntos de PDF.");
    Ok(())
}

fn menu() -> io::Result<()> {
    println!("\nSelecciona una opción:\n1. Extraer archivos adjuntos\n2. Convertir a PDF\n3. Salir");

    let mut opcion = String::new();
    io::stdin().read_line(&mut opcion)?;
    let opcion = opcion.trim_end_matches('\n').parse::<u32>().unwrap_or(0);

    match opcion {
        1 => {
            println!("Introduce la ruta del archivo winmail.dat:");
            let mut archivo = String::new();
            io::stdin().read_line(&mut archivo)?;
            let archivo = archivo.trim_end_matches('\n');

            let file_path = Path::new(&archivo);
            if !file_path.exists() {
                println!("El archivo no existe.");
                return Ok(())
            }

            if let Some(parent) = file_path.parent() {
                let output_dir = parent.join("ficheros winmail en pdf");
                fs::create_dir_all(&output_dir)?;
            }

            extract_attachments(&file_path)?;
        },
        
                2 => {
            println!("Introduce la ruta del archivo winmail.dat:");
            let mut archivo = String::new();
            io::stdin().read_line(&mut archivo)?;
            let archivo = archivo.trim_end_matches('\n');

            let file_path = Path::new(&archivo);
            if !file_path.exists() {
                println!("El archivo no existe.");
                return Ok(())
            }

            if let Some(parent) = file_path.parent() {
                let output_dir = parent.join("ficheros winmail en pdf");
                fs::create_dir_all(&output_dir)?;
            }

            convert_to_pdf(&file_path)?;
        },
        3 => {
            println!("Saliendo del programa.");
            process::exit(0);
        },
        _ => {
            println!("Opción inválida.");
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    loop {
        menu()?;
    }
}
