# The Football Database Web Scraper

> A webscraper for football box scores from the football database, used to create a box score generator model

## Overview

This repository contains a command-line utility which scrapes data from [The Football Database](https://www.footballdb.com).  Given a year, it can be used to scrape that year's worth of box scores into a structured JSON document.  The below box score data, for example

| Date | Away Team | Away Score | Home Team | Home Score |
| ---- | --------- | ---------- | --------- | ---------- |
| 1/2/03 | Team A | 28 | Team B | 31 |
| 1/3/03 | Team C | 13 | Team D | 6 |

is structured by `fbdb-webscrape` into a JSON document like so.

```json
[
    {
        "date": "1/2/03",
        "away-team": "Team A",
        "away-score": 28,
        "home-team": "Team B",
        "home-score": 31
    },
    {
        "date": "1/3/03",
        "away-team": "Team C",
        "away-score": 13,
        "home-team": "Team D",
        "home-score": 6
    }
]
```

## Usage

The `fbdb-webscrape` CLI usage is given below for each of its main subcommands

### Box Scores

**Examples**
```sh
# Prints the 2024 box scores in the default format to stdout
fbdb-webscrape boxscores

# Writes the 2019 box scores in the default format to stdout
fbdb-webscrape boxscores -y 2019

# Writes the 2021 box scores in JSON format to stdout
fbdb-webscrape boxscores -y 2021 -o json

# Writes the 2023 box scores in JSON format to a file
fbdb-webscrape boxscores -y 2023 -o json -f ./out.json
```

**Usage**
```
Retrieve historic box scores from the football database

Usage: fbdb-webscrape[.exe] boxscores [OPTIONS]

Options:
  -o, --output <OUTPUT_FORMAT>  The format to output
  -f, --file <OUTPUT_FILE>      The file to write to
  -y, --year <YEAR>             The year of scores to retrieve [default: 2024]
  -h, --help                    Print help
```
