use player::{play_movie, play_audio};

#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_mut)]
#[allow(dead_code)]

mod player;

fn main() {
    play_movie("snatch.mp4");
    play_audio("rhpc.mp3");

    clean::perform_clean();
    clean::files::clean_files();
}

// encapsulating functionality inside a mod block
mod clean {
    pub fn perform_clean() {
        println!("Cleaning hdd");
    }
    // nested module
    pub mod files {
        pub fn clean_files() {
            println!("Remove unused files");
        }
    }
}
