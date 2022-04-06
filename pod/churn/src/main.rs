use std::{collections::BTreeMap, path::Path};

use chrono::{Local, TimeZone};
use regex::Regex;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Episode {
    show_id: String,
    description: String,
    number: String,
    season: String,
    #[serde(rename = "type")]
    typ: String,
    title: String,
}

#[derive(Debug, Serialize)]
struct EpisodeWrapper {
    episode: Episode,
}
fn main() {
    let re = Regex::new(r"((\d+)_(\d+)_(\d+))\.md:## (.*)").unwrap();

    let data = std::fs::read_to_string("../sodes.txt").unwrap();
    let sodes = data.lines();
    let all = sodes
        .enumerate()
        .map(|(n, sode)| {
            println!("{}", sode);
            let captures = re.captures(sode).unwrap();
            println!("{:#?}", captures);

            let (ymd, yy, mm, dd, title) = (
                &captures[1],
                &captures[2],
                &captures[3],
                &captures[4],
                &captures[5],
            );

            let date = Local.ymd(
                yy.parse().unwrap(),
                mm.parse().unwrap(),
                dd.parse().unwrap(),
            );
            let path = format!("../html/{}.html", ymd);
            println!("path = {}", path);
            let description = std::fs::read_to_string(path).unwrap();

            (
                date,
                Episode {
                    show_id: "29256".to_string(),
                    description,
                    number: format!("{}", n + 1),
                    season: "2".to_string(),
                    typ: "full".to_string(),
                    title: format!("{} ({})", title, date.format("%Y-%m-%d")),
                },
            )
        })
        .collect::<BTreeMap<_, _>>();

    for (date, sode) in all {
        let contents = serde_json::to_string_pretty(&EpisodeWrapper { episode: sode }).unwrap();
        println!("{}", contents);

        std::fs::write(
            format!("../json/{}.json", date.format("%Y_%m_%d")),
            contents,
        )
        .unwrap();
    }
}
