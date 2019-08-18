// Typing prompts adapted from
// http://www.shareyouressays.com/paragraphs/5-sample-paragraph-for-typing-test-for-newbie-typists/1530

use difference::Changeset;
use std::convert::TryInto;
use std::io::stdin;
use std::time::Instant;
use textwrap::fill;

pub const PROMPT_1: &str = "Studying is the main source of knowledge. Books are indeed never failing friends of man. For a mature mind, reading is the greatest source of pleasure and solace to distressed minds. The study of good books ennobles us and broadens our outlook.\n";
pub const PROMPT_2: &str = "A teacher is called builder of the nation. The profession of teaching needs men and women with qualities of head and heart. There are many teachers in our school and a large number of teachers among them are highly qualified. I have great respect for all of them. Yet I have a special liking for Miss Y. Miss Y is a woman of great principles. She is jewel among all the teachers.\n";
pub const PROMPT_3: &str = "Recently, an exhibition ‘Building A New India’ was held in the capital. It was organized by the Ministry of Information and Broadcasting, Government of India. The exhibition was set up in the Triveni Kala Sangam. The chief exhibits were photographs, novels, some sculptures by Indian modern artists presenting Indian cultural inheritance. I visited the general section of the exhibition where different charts and photographs depicting India’s development in various fields were set.\n";

fn main() {
    println!("Please choose a prompt (1 is short, 2 is medium, 3 is long)");
    println!("1. The Habit of Reading");
    println!("2. My Favorite Teacher");
    println!("3. A visit to an Exhibition");

    let mut choice = String::new();
    stdin().read_line(&mut choice).expect("Failed to read line");

    let chosen_prompt = match choice.as_str().trim() {
        "1" => PROMPT_1,
        "2" => PROMPT_2,
        "3" => PROMPT_3,
        _ => "Invalid choice",
    };

    println!("\n{}", fill(chosen_prompt, 80));
    println!("Type start to begin. You must type the entire prompt to get an accurate score.");
    let mut begin = String::new();
    stdin().read_line(&mut begin).expect("Failed to read line");

    println!("\nThe test has begun hit return when you are finished.");
    let start = Instant::now();
    let mut response = String::new();
    stdin()
        .read_line(&mut response)
        .expect("Failed to read line");
    let duration = start.elapsed();
    let seconds: u64 = duration.as_secs().try_into().unwrap();
    let minutes: f64 = seconds as f64 / 60.0;

    let check_mistakes = Changeset::new(chosen_prompt, response.as_str(), " ");
    let mistakes = if check_mistakes.diffs.len() == 1 {
        0
    } else {
        ((check_mistakes.diffs.len() + 2 - 1) / 2) - 1
    };

    let wpm: f64 = ((response.as_str().chars().count() / 5) - mistakes) as f64 / minutes;
    println!(
        "\nYou typed {} characters",
        response.as_str().chars().count()
    );
    println!("You took {} seconds to finish", duration.as_secs());
    println!("You made {} mistakes", mistakes);
    println!("Your WPM is {}", wpm);
}
