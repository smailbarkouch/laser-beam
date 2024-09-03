use std::{env, fs, thread};
use std::collections::HashSet;
use std::time::Duration;

const OPEN_SECTIONS_URL: &str = "https://sis.rutgers.edu/soc/api/openSections.json";
const CLASS_YEAR: &str = "2024";
const TERM: &str = "9";
const CAMPUS: &str = "NB";

const REFRESH_SECONDS: u64 = 10;

const NTFY_URL: &str = "https://ntfy.sh/laser-beam-course-sniper";
const SEMESTER_CODE: &str = "92024";

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let contents = fs::read_to_string(file_path).unwrap();
    let requested_classes: Vec<&str> = contents.split("\n").filter(|line| { line.parse::<u32>().is_ok() }).collect();

    let client = reqwest::blocking::Client::new();

    client.post(NTFY_URL).body(format!("Started running laser beam course sniper. Test url: https://sims.rutgers.edu/webreg/editSchedule.htm?login=cas&semesterSelection={}&indexList={}", SEMESTER_CODE, 06837)).send().unwrap();

    loop {
        println!("Checking open courses...");
        let _ = check_webreg(&client, &requested_classes);

        println!("Finished! Waiting {} seconds...", REFRESH_SECONDS);
        thread::sleep(Duration::from_secs(REFRESH_SECONDS))
    }
}

fn check_webreg(client: &reqwest::blocking::Client, requested_classes: &Vec<&str>) -> Result<(), reqwest::Error> {
    let url = format!("{}?year={}&term={}&campus={}", OPEN_SECTIONS_URL, CLASS_YEAR, TERM, CAMPUS);
    let open_courses: HashSet<String> = client.get(url).send()?.json()?;

    for class in requested_classes {
        if open_courses.contains(*class) {
            println!("The class code \"{}\" is open: https://sims.rutgers.edu/webreg/editSchedule.htm?login=cas&semesterSelection={}&indexList={}", class, SEMESTER_CODE, class);
            client.post(NTFY_URL).body(format!("Class with class code {} is open!", class)).send()?;
        }
    }

    Ok(())
}
