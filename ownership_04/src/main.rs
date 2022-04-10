mod ownership;
mod references;
mod slices;

fn main() {

    let word = String::from("hello world");
    let slice = slices::first_word_with_slices(&word[..]);

    println!("data: {}", slice);

}