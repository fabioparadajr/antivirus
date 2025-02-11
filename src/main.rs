mod monitor;
mod signature;
mod hash;

use regex::Regex;
use std::path::PathBuf;
use crate::hash::get_file_hash;
use tokio::sync::mpsc;
use crate::signature::check_signature;

    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error>> {
        // for tokio console
         
        let rgx: Regex = Regex::new(r"^(/([a-z0-9_-]+/)*[a-z0-9_-]+)?/$").unwrap();
        let path ;
        loop {
            println!("Hello, type a path to monitor: ");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input_trim = input.trim();

            if rgx.is_match(&input_trim) {
                path = String::from(input_trim);
                println!("Path: {}", path);

                break;
            } else {
                println!("Wrong path, my friend!");
            }
        }


        let (tx, mut rx) = mpsc::channel::<Option<PathBuf>>(100);

        monitor::monitor_directory(&path, tx).await?;

        while let Some(file_path) = rx.recv().await {
            match file_path {
                Some(file_path) => {
                    //let file_path_str = file_path.to_str().unwrap_or_default();
                    let file_path_str = format!("{}{}", path, file_path.to_str().unwrap_or_default());

                    if let Ok(hash) = get_file_hash(&file_path_str).await {
                        
                        if check_signature(&hash).await {
                            println!("Signature found it : {}, file hash {}", file_path_str, hash);
                        } else {
                            println!("Sginature not found it: {}, file hash {}", file_path_str, hash);
                        }
                    } else {
                        println!("Erro ao calcular o hash do arquivo: {}", file_path_str);  
                    }
                }
                None => {
                    println!("No events");
                    break
                },
            }
        }


        Ok(())
    }
