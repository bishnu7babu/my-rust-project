
mod file_system_program {

    use std::{ fs, io::Read, path::Path} ;
    use fs_extra::dir::{self, CopyOptions};

    pub trait CreateFS {
        fn terminal_argument(&mut self);
        fn create_folder(&self, name:&str) -> std::io::Result<()>;
        fn create_file(&self, name:&str) -> std::io::Result<()>;
        fn read_file(&self, file_name:&str);
        fn copy_file(&self, source:&str, destination:&str) -> std::io::Result<bool>;
        fn move_file(&self, source:&str, destination:&str) -> std::io::Result<()>;
        fn copy_dir_all(&self, src:&Path, dest:&Path) -> std::io::Result<bool>;
        fn success_msg(&self);
        fn error_msg(&self);
    }


    pub struct CreateProgram {
        pub foldername: String,
        pub filename:String,
        pub read_file:String,
        pub match_result: clap::ArgMatches
    }

    impl CreateFS for CreateProgram {
        fn terminal_argument(&mut self) {

            if let Some(value) = self.match_result.get_one::<String>("Create Folder") {
                // println!()
                self.foldername = value.clone();
                let _ = self.create_folder(&self.foldername);
            }

            if let Some(value) = self.match_result.get_one::<String>("Create File") {
                self.filename = value.clone();
                let _ = self.create_file(&self.filename);
            }

            if let Some(value) = self.match_result.get_one::<String>("Read File") {
                self.read_file = value.clone();
                self.read_file(&self.read_file);
            }

            if let Some(matches) = self.match_result.subcommand_matches("copy") {
                let source = matches.get_one::<String>("source").unwrap().to_string();
                let destination = matches.get_one::<String>("destination").unwrap().to_string();
                let _ = self.copy_file(&source, &destination);
            }

            if let Some(matches) = self.match_result.subcommand_matches("move") {
                let source = matches.get_one::<String>("source").unwrap().to_string();
                let destination = matches.get_one::<String>("destination").unwrap().to_string();
                let _ = self.move_file(&source, &destination);
            }

        }

        fn success_msg(&self) {
            println!("Operation completed successfully.");
        }

        fn error_msg(&self) {
            println!("Operation failed");
        }


        fn create_folder(&self, name:&str) -> std::io::Result<()>{
            match fs::create_dir(name) {
                Ok(_) => println!("Folder is created"),
                Err(err) => eprintln!("Error on creating folder {}",err) 
            }
            Ok(())
            }

        fn create_file(&self, name:&str) -> std::io::Result<()>{
            match fs::File::create(name) {
                Ok(_) => println!("File created"),
                Err(err) => eprintln!("Error on creating file {}",err),
            }
            Ok(())
        }

        fn read_file(&self,file_name:&str) {
            let option = fs::OpenOptions::new()
                                                .read(true)
                                                .open(file_name);
                
            match option {
                Ok(mut file) => {
                    let mut content = String::new();
                    match file.read_to_string(&mut content) {
                        Ok(_) => println!("{}",content),
                        Err(err) => println!("Failed To Read File '{}': {}",file_name,err)
                    }
                }
                Err(err) => println!("Failed To Open File '{}': {}",file_name,err)
            }
        }

        fn copy_dir_all(&self, src:&Path, dest:&Path) -> std::io::Result<bool> {

            let options = CopyOptions::new();
            let _ = dir::copy(src, dest, &options);
            Ok(true)
        }

        fn copy_file(&self, source:&str, destination:&str) -> std::io::Result<bool> {
            
            let dest = std::path::Path::new(destination);
            let src = std::path::Path::new(source);

            if let Some(parent) = dest.parent() {
                fs::create_dir_all(parent)?;
            }

            if let Some(file_name) = dest.file_name() {
                if !file_name.to_string_lossy().contains('.') {
                    fs::create_dir_all(dest)?;
                }
            }

            if src.is_dir() {
                let is_sucess = self.copy_dir_all(&src, &dest);
                if is_sucess? {
                    self.success_msg();
                }else {
                    self.error_msg();
                }
            }else {
                if let Some(src_ext) = src.extension() {
                    if let Some(dest_ext) = dest.extension() {
                        if dest_ext == src_ext {
                            fs::copy(src, dest)?;
                        }else {
                            println!("file extension doesn't match. (Please give proper extension)")
                        }
                    }
                }else {
                    // This line will do copy the file in the destination folder without giving a file name.
                    let file_name = format!("{}/{}",dest.display(),src.display());
                    fs::copy(src, file_name)?;
                }
            }
            
            Ok(true)
        }

        fn move_file(&self, source:&str, destination:&str) -> std::io::Result<()> {

            let src = std::path::Path::new(source);
            let success = self.copy_file(source, destination);

            if success? {
                fs::remove_dir_all(src)?;
                self.success_msg();
            }else {
                self.error_msg();
            }

            Ok(())
        }
        
    }

}



use std::io;
use clap::Command;
use clap::{command, Arg};
use file_system_program::CreateFS;
use file_system_program::CreateProgram;

fn main() -> io::Result<()> {

    let matches = command!()
                .about("A CLI tool for simple file system operations")
                .version("1.1")
                .author("Shadow Code").subcommand(
                    Command::new("copy")
                        .about("Copy a file from source to destination")
                        .arg(
                            Arg::new("source")
                                .help("Sets The Source File Path")
                                .required(true)
                                .index(1)
                        ).arg(
                            Arg::new("destination")
                                .help("Sets The Destination File Path")
                                .required(true)
                                .index(2)
                        )
                ).subcommand(
                    Command::new("move")
                        .about("move a file from source to destination")
                        .arg(
                        Arg::new("source")
                            .help("Sets The Source File Path")
                            .required(true)
                            .index(1)
                    ).arg(
                        Arg::new("destination")
                            .help("Sets The Destination File Path")
                            .required(true)
                            .index(2)
                    )
                )
                .arg(
                    Arg::new("Create Folder")
                        .long("foldercrt")
                        .short('F')
                        .help("Create Your Folder In The Current Directory")
                        .aliases(["createfolder","create_folder","folder_create","foldercreate"])
                        

                ).arg(
                    Arg::new("Create File")
                        .long("filecrt")
                        .short('f')
                        .help("Create Your File With Extension In Your Current Directory")
                        .aliases(["createfile","create_file","filecreate","file_create"])
                        
                ).arg(
                    Arg::new("Read File")
                        .long("rf")
                        .help("Read File In Your Current Directory")
                        
                ).get_matches();

    let mut prog = CreateProgram {
        foldername: String::new(),
        filename: String::new(),
        read_file: String::new(),
        match_result: matches,
    };
    prog.terminal_argument();

    Ok(())
}