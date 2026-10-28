use std::fs;
use std::path::Path;

fn main() {
    clear_cache("/data/data/com.google.android.youtube/cache");
    clear_cache("/data/user_de/0/com.google.android.gms/app_chimera/m");
}

fn clear_cache(pasta: &str) {
    let P = Path::new(pasta);

    if !P.exists() {
        return;
    }

    match fs::remove_dir_all(P) {
        Ok(_) => println!("OK"),
        Err(e) => println!("ERRO em {pasta} e {e}"),  
    }
}
