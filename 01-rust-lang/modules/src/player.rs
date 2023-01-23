// by default functions are restricted to the file you define them in
// they are private
// pub keyword makes the publicly available
pub fn play_movie(name: &str) {
    println!("Playing movie {}", name);
}

pub fn play_audio(name: &str) {
    println!("Playing audio {}", name);
}