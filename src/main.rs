use std::fmt;

use clap::{Parser, ValueEnum};
use colored::Colorize;
use num_format::{Locale, ToFormattedString};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ConvertTo {
    Hourly,
    Salary,
}

impl fmt::Display for ConvertTo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConvertTo::Hourly => write!(f, "hour"),
            ConvertTo::Salary => write!(f, "year"),
        }
    }
}

impl ConvertTo {
    const WORKING_HOURS_PER_YEAR: f32 = 40.0 * 52.0;

    fn opposite(&self) -> &str {
        match self {
            ConvertTo::Hourly => "year",
            ConvertTo::Salary => "hour",
        }
    }

    fn new_rate(&self, rate: f32) -> i32 {
        match self {
            ConvertTo::Hourly => (rate * ConvertTo::WORKING_HOURS_PER_YEAR) as i32,
            ConvertTo::Salary => (rate / ConvertTo::WORKING_HOURS_PER_YEAR) as i32,
        }
    }
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[clap(color = concolor_clap::color_choice())]
struct Args {
    mode: ConvertTo,
    pay_rate: f32,
    #[command(flatten)]
    color: concolor_clap::Color,
}

fn get_conversion_msg(convert_to: ConvertTo) -> &'static str {
    match convert_to {
        ConvertTo::Hourly => "a salary",
        ConvertTo::Salary => "an hourly pay",
    }
}

fn get_current_rate(rate: f32, convert_to: ConvertTo) -> String {
    let formattable_rate = rate as i32;
    format!(
        "${} per {}",
        formattable_rate.to_formatted_string(&Locale::en).green(),
        convert_to.to_string(),
    )
}

fn get_new_rate(rate: f32, convert_to: ConvertTo) -> String {
    format!(
        "${} per {}",
        convert_to.new_rate(rate).to_formatted_string(&Locale::en).green(),
        convert_to.opposite(),
    )
}

fn print_other_pay_rate(rate: f32, convert_to: ConvertTo) {
    println!(
        "Your current pay rate of {} converted to {} would be {}",
        get_current_rate(rate, convert_to),
        get_conversion_msg(convert_to).underline(),
        get_new_rate(rate, convert_to),
    );
}

fn main() {
    let args: Args = Args::parse();
    print_other_pay_rate(args.pay_rate, args.mode);
}
