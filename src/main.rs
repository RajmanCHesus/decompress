use std::fs;
use std::io;

fn main() {
    // Call real_main and exit with its return value
    std::process::exit(real_main())
}

fn real_main() -> i32 {
    // Collect command-line arguments
    let args: Vec<_> = std::env::args().collect();
    
    // Check if the required command-line argument (filename) is provided
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return 1;
    }

    // Extract the filename from the command-line arguments
    let fname = std::path::Path::new(&*args[1]);
    
    // Open the specified file
    let file = fs::File::open(&fname).unwrap();
    
    // Create a ZipArchive from the opened file
    let mut archive = zip::ZipArchive::new(file).unwrap();

    // Iterate over each file in the archive
    for i in 0..archive.len() {
        // Get a reference to the current file in the archive
        let mut file = archive.by_index(i).unwrap();

        // Determine the output path for the extracted file
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue, // Skip files without a valid path
        };

        // Check if the file has a comment and print it
        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        // Check if the file represents a directory
        if (*file.name()).ends_with('/') {
            // Print and create the directory
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath).unwrap();
        } else {
            // Print and create the file with its size
            println!(
                "File {} extracted to \"{}\" ({} bytes)", 
                i,
                outpath.display(),
                file.size()
            );

            // Create parent directories if they don't exist
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }

            // Create and write the extracted file
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Platform-specific code for Unix systems to set file permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    // Return success status
    0
}
