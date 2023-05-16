use std::fs;

fn main() {

    println!("snapsweep v1.0 developed by florencio");

    let paths: fs::ReadDir = fs::read_dir("./").unwrap();

    println!("finding files and removing them..");

    let mut file: String;

    for path in paths {

        file = path.unwrap().path().display().to_string();
        let _ = file.remove(0);
        let _ = file.remove(0);

        if file.contains("Print 2") {
            println!("removing '{}'..", &file);
            fs::remove_file(&file).ok();
            
        }

        if file.contains("Preview 2") {
            println!("removing '{}'..", &file);
            fs::remove_file(&file).ok();
        }

    }

    println!("finished! exiting..");

}
