#![feature(plugin, result_expect)]
#![plugin(docopt_macros)]
extern crate serde_json;
extern crate hyper;
extern crate time;
extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;
use hyper::client::Client;
use std::io::prelude::*;
use serde_json::Value as Json;
use std::error::Error;

fn http(c: &Client, id: Option<isize>) -> Result<Json, Box<Error>> {
    let query = match id {
        Some(id) => format!("/{}", id),
        None     => String::new()
    };
    let response = try!(
        c.get(&format!("http://buildbot.rust-lang.org/json/builders/nightly-dist-rustc-linux/builds{}", query)).send()
    );
    Ok( try!( serde_json::from_reader(response) ) ) // Ok(try!) is to invoke From::from
}

macro_rules! tryo{ ($e:expr) => (match $e { Some(e) => e, None => return None }) }

fn parse(j: Json) -> Option<(String, time::Tm)> {
    let seconds = tryo!( j.find("times").and_then(Json::as_array).and_then(|a|a.get(0)).and_then(Json::as_f64) );
    let tm = time::at_utc(time::Timespec{ sec: seconds as i64, nsec: 0 });
    let properties = tryo!( j.find("properties").and_then(Json::as_array) );
    for p in properties {
        let p = tryo!(p.as_array());
        match p.get(0).and_then(Json::as_string) {
            Some("revision") => return Some((tryo!( p.get(1).and_then(Json::as_string).map(Into::into) ), tm)),
            _ => ()
        }
    }
    return None
}

docopt!(Args derive Debug, "
rustnightlyhash computes a git commit hash for a nightly from a given date
in YYYY-MM-HH format.
When given no date, it shows the hash for most recent nightly
(even if the build is still in progress).

Usage:
  rustnightlyhash [options] [<date>]

Options:
  -h --help         Show this screen.
  -d --output-date  Displays the date after the hash.
");

fn display(&(ref hash, ref tm): &(String, time::Tm), args: &Args) {
    if args.flag_output_date {
        println!("{} {}", hash, tm.strftime("%F").expect("strftime failed"));
    } else {
        println!("{}", hash);
    }
}

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    
    let client = Client::new();
    
    // We compute tonight's hash even when the date is given. It's a kind of server sanity check.
    let tonightly = parse(http(&client, Some(-1)).expect("http error")).expect("json format error");
    
    if args.arg_date != "" {
        let needle_start = time::strptime(&args.arg_date, "%F").expect("date argument in invalid format");
        let needle_end = needle_start + time::Duration::days(1);
        
        let mut ids: Vec<isize> = http(&client, None).expect("http error (build list)")
            .as_object().expect("expected dict").keys().map(|s|s.parse().expect("non-integral key"))
            .collect()
        ;
        ids.sort();
        let ids = ids;

        let mut range: &[isize] = &ids;
        assert!(range.len() > 0);
        
        while range.len() > 1 {
            let mid = range.len() / 2;
            if http(&client, Some(range[mid])).ok().and_then(parse).expect("error in binsearch").1 <= needle_end {
                range = &range[mid..range.len()];
            } else {
                range = &range[0..mid];
            }
        }
        
        let tup = http(&client, Some(range[0])).ok().and_then(parse).expect("error in binsearch");
        let tm = tup.1;
        if tm >= needle_start && tm < needle_end {
            display(&tup, &args);
        } else {
            writeln!(std::io::stderr(), "Didn't find any build for {}", args.arg_date).ok();
            std::process::exit(1);
        }
    } else {
        display(&tonightly, &args);
    }
}
