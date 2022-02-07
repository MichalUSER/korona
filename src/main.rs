use colored::*;
use prettytable::{Cell, Row, Table};
use scraper::{Html, Selector};

//.header("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.212 Safari/537.36")

fn main() -> Result<(), minreq::Error> {
    let req = minreq::get("https://korona.gov.sk/koronavirus-na-slovensku-v-cislach").send()?;
    let document = Html::parse_document(req.as_str()?);

    let _selector = Selector::parse(
        "
        #block_60379179c4f8b > div
        #block_603791edc4f8d > div
        #block_603780b691b98 > div
        #block_6037862491b9a > div
        #block_60378ba2c4f83 > div
        #block_60378c0bc4f85 > div
    ",
    )
    .unwrap();
    // Vaccine stats
    let first_dose_selector = Selector::parse("#block_6151968e9659e > div").unwrap();
    let second_dose_selector = Selector::parse("#block_603791edc4f8d > div").unwrap();
    let dose_selector = Selector::parse("#block_60379179c4f8b > div").unwrap();

    // Covid stats
    let pcr_test_selector = Selector::parse("#block_603780b691b98 > div").unwrap();
    let pcr_pozitivny_selector = Selector::parse("#block_6037862491b9a > div").unwrap();
    let ag_test_selector = Selector::parse("#block_60378ba2c4f83 > div").unwrap();
    let ag_pozitivny_selector = Selector::parse("#block_60378c0bc4f85 > div").unwrap();

    // Fix this, make it shorter
    let first = document.select(&first_dose_selector).next().unwrap();
    let second = document.select(&second_dose_selector).next().unwrap();
    let dose = document.select(&dose_selector).next().unwrap();

    let pcr_test = document.select(&pcr_test_selector).next().unwrap();
    let pcr_pozitivny = document.select(&pcr_pozitivny_selector).next().unwrap();
    let ag_test = document.select(&ag_test_selector).next().unwrap();
    let ag_pozitivny = document.select(&ag_pozitivny_selector).next().unwrap();

    let first_vec: Vec<&str> = first.text().collect();
    let second_vec: Vec<&str> = second.text().collect();
    let dose_vec: Vec<&str> = dose.text().collect();

    let pcr_test_vec: Vec<&str> = pcr_test.text().collect();
    let pcr_pozitivny_vec: Vec<&str> = pcr_pozitivny.text().collect();
    let ag_test_vec: Vec<&str> = ag_test.text().collect();
    let ag_pozitivny_vec: Vec<&str> = ag_pozitivny.text().collect();

    let mut table1 = Table::new();
    // TODO: fix colors
    table1.add_row(Row::new(vec![
        Cell::new(""),
        Cell::new("Pribudlo testov".blue().to_string().as_str()),
        Cell::new("Pribudlo pozitývnych".magenta().to_string().as_str()),
    ]));
    table1.add_row(Row::new(vec![
        Cell::new("PCR".magenta().to_string().as_str()),
        Cell::new(pcr_test_vec[1]),
        Cell::new(pcr_pozitivny_vec[1]),
    ]));
    table1.add_row(Row::new(vec![
        Cell::new("AG".blue().to_string().as_str()),
        Cell::new(ag_test_vec[1]),
        Cell::new(ag_pozitivny_vec[1]),
    ]));
    table1.printstd();

    println!();

    let mut table2 = Table::new();
    table2.add_row(Row::new(vec![
        Cell::new(""),
        Cell::new("Pribudlo".blue().to_string().as_str()),
        Cell::new("Celkovo".magenta().to_string().as_str()),
    ]));
    table2.add_row(Row::new(vec![
        Cell::new("Prvá dávka".magenta().to_string().as_str()),
        Cell::new(first_vec[1]),
        Cell::new(dose_vec[1]),
    ]));
    table2.add_row(Row::new(vec![
        Cell::new("Druhá dávka".blue().to_string().as_str()),
        Cell::new(second_vec[1]),
        Cell::new(dose_vec[5]),
    ]));
    table2.printstd();

    Ok(())
}
