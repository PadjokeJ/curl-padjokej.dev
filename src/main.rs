use std::fs::File;
use std::io::prelude::*;
use std::str::Split;

#[macro_use] extern crate rocket;

const RST: &str = "\x1B[0m";

const BLUE: &str = "\x1B[34m";
const RED: &str = "\x1B[31m";
const CYAN: &str = "\x1B[36m";

const MAX_LINE_LENGTH: usize = 32;
const WEBSITE_HEADER: &str = "\
\t.----------------.
\t|    PadjokeJ    |
\t'----------------'";

fn shorten(s: &str, len: usize) -> String {
    let mut short: String = String::new();

    let s = s.split(' ');

    let mut line_length = 0;

    s.for_each(|w| -> () {
        line_length += w.len();

        if line_length > len {
            line_length = w.len();
            short.push_str("\n\t");
        }

        short.push_str(w);
        short.push(' ');
    });

    short
}

fn read_file(filename: &str) -> String {
    let mut file = File::open(filename).unwrap();
    let mut cts = String::new();
    file.read_to_string(&mut cts).unwrap();
    cts
}

fn routes() -> String {
    let s: String = read_file("routes.txt");
    let r: Split<'_, char> = s.split('\n');
    let mut s: String = String::new();

    let p = format!("\t{}${} curl https://padjokej.dev{}", CYAN, BLUE, CYAN);

    r.for_each(|a| -> () { 
        if a.len() > 0 {
            s.push_str(&p);
            s.push_str(a);
            s.push_str(RST);
            s.push('\n');
        }
    });

    s
}

#[get("/socials")]
fn socials() -> String {
    let route: &str = "/socials:";
    let mut socials = read_file("socials.txt");

    socials = socials.replace("{cyan}", CYAN);
    socials = socials.replace("{reset}", RST);
    socials = socials.replace("{blue}", BLUE);

    format!("{}\n{}\n", route, socials)
}

#[get("/")]
fn index() -> String {
    let route: &str = "/:";
    let description = shorten("\n\tHello, I'm PadjokeJ, a computer science student from Switzerland.\n", MAX_LINE_LENGTH);
    format!("{}\n{}\n{}\n{}\n", route, WEBSITE_HEADER, description, routes())
}

#[launch]
fn rocket() -> _ {
    let routes = routes![index, socials];
    let mut file = File::create("routes.txt").unwrap();
    
    for i in &routes {
        file.write(format!("{}\n", i.uri.path()).as_bytes());
    }

    rocket::build().mount("/", routes)
}

