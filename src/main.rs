mod boxscore;
mod cli;

use crate::boxscore::BoxScore;
use crate::cli::{FbdbWebScrapeCli, FbdbSubcommand, OutputFormat};

use clap::Parser;
use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};
use scraper::element_ref::Select;
use serde_json;
use std::str::FromStr;
use std::fs;

/// Get the box scores HTML response from the football database as a string
fn get_box_scores(year: u32) -> String {
    // Initialize a blocking client
    let client = Client::new();

    // Query the football database for HTML response
    let res = client.get(
            &format!("https://www.footballdb.com/games/index.html?lg=NFL&yr={}", year)
        )
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/88.0.4324.150 Safari/537.36")
        .send();
    
    // Extract the html from the response
    let res_str = res.unwrap().text().unwrap();
    return res_str;
}

/// Parse a box score from table data from the football database's box score
/// table
fn parse_box_score(data: &mut Select) -> BoxScore {
    // Initialize selector for collapsable data
    let collapsable_selector = Selector::parse("span.hidden-xs").unwrap();

    // Extract the date box score property
    let date_datum = data.next().unwrap();
    let date: String = date_datum.select(&collapsable_selector)
        .flat_map(|el| el.text())
        .collect();
    
    // Extract the away team box score property
    let away_team_datum = data.next().unwrap();
    let away_team: String = away_team_datum.select(&collapsable_selector)
        .flat_map(|el| el.text())
        .collect();
    
    // Extract the away score box score property
    let away_score_datum = data.next().unwrap();
    let away_score_string: String = away_score_datum.text().collect();
    let away_score: i32 = away_score_string.parse().unwrap();
    
    // Extract the home team box score property
    let home_team_datum = data.next().unwrap();
    let home_team: String = home_team_datum.select(&collapsable_selector)
        .flat_map(|el| el.text())
        .collect();
    
    // Extract the home score box score property
    let home_score_datum = data.next().unwrap();
    let home_score_string: String = home_score_datum.text().collect();
    let home_score: i32 = home_score_string.parse().unwrap();

    // Initialize as a BoxScore
    BoxScore::from_properties(
        date,
        away_team,
        away_score,
        home_team,
        home_score
    )
}

/// Parse the box scores from the HTML response from the football database into
/// a Vec of BoxScore structs
fn parse_box_scores(res_html: &String) -> Vec<BoxScore> {
    // Initialize a Vec of BoxScores for the parsed data
    let mut box_scores: Vec<BoxScore> = Vec::new();

    // Initialize an HTML scraper from the response HTML
    let document = Html::parse_document(res_html);

    // Select all tables of class statistics
    let table_selector = Selector::parse("table.statistics").unwrap();
    let stat_tables = document.select(&table_selector);

    // For each table, parse each table row of class row0 or row1
    for stat_table in stat_tables {
        let row_selector = Selector::parse("tr.row0,tr.row1").unwrap();
        let stat_rows = stat_table.select(&row_selector);

        // For each row, parse each datum
        for stat_row in stat_rows {
            let data_selector = Selector::parse("td").unwrap();
            let mut stat_data = stat_row.select(&data_selector);

            // Parse each datum into a BoxScore and append
            let box_score = parse_box_score(&mut stat_data);
            box_scores.push(box_score);
        }
    }

    // Return the vector of box scores
    box_scores
}

fn main() {
    // Parse the command-line args
    let fbdb_cli = FbdbWebScrapeCli::parse();

    // Perform the subcommand
    let command = fbdb_cli.command();
    match &command {
        FbdbSubcommand::Boxscores(args) => {
            // Get the box scores from the football database
            let res_html = get_box_scores(args.year);
            let box_scores = parse_box_scores(&res_html);

            // Format the box scores as a string in the provided format
            let output_format = OutputFormat::from_str(
                &args.output_format.clone().unwrap_or(String::from(""))
            ).unwrap();
            let box_scores_str: String = match output_format {
                OutputFormat::Json => {
                    serde_json::to_string_pretty(&box_scores).unwrap()
                },
                OutputFormat::Default => {
                    let box_score_strs: Vec<String> = box_scores.iter().map(|s| format!("{}", s)).collect();
                    box_score_strs.join("\n")
                }
            };

            // Write the box scores either to stdout or to a file
            match &args.output_file {
                Some(x) => {
                    // Write the output to the output file
                    _ = fs::write(x, box_scores_str);
                },
                None => {
                    // Print the output to stdout
                    println!("{}", box_scores_str);
                }
            }
        }
    }
}
