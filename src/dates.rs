use chrono::{Duration, Local, NaiveDate};
use regex::Regex;

/// Parses a date argument which could be an absolute date (YYYY-MM-DD) or
/// a relative date (e.g., +1w, -2d), returning a NaiveDate.
pub fn parse_date_arg(arg: Option<&str>) -> Option<NaiveDate> {
    arg.and_then(|s| {
        if let Ok(date) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
            // Absolute date
            Some(date)
        } else if let Some(rel_date) = parse_relative_date(s) {
            // Relative date
            Some(rel_date)
        } else {
            None
        }
    })
}

/// Parses a relative date specification (e.g., "+1w", "-3d") and returns the corresponding NaiveDate.
pub fn parse_relative_date(spec: &str) -> Option<NaiveDate> {
    let today = Local::today().naive_local();
    let re = Regex::new(r"([+-])(\d+)([dwmy])").unwrap();
    re.captures(spec).and_then(|caps| {
        let sign = caps.get(1)?.as_str();
        let quantity: i64 = caps.get(2)?.as_str().parse().ok()?;
        let unit = caps.get(3)?.as_str();

        let duration = match unit {
            "d" => Duration::days(quantity),
            "w" => Duration::weeks(quantity),
            "m" => Duration::days(quantity * 30), // Approximation
            "y" => Duration::days(quantity * 365), // Approximation
            _ => return None,
        };

        match sign {
            "+" => Some(today + duration),
            "-" => Some(today - duration),
            _ => None,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Local};

    #[test]
    fn parse_absolute_date() {
        // Test with a valid absolute date
        let date_str = "2024-01-01";
        let expected_date = NaiveDate::from_ymd(2024, 1, 1);
        assert_eq!(parse_date_arg(Some(date_str)), Some(expected_date));
    }

    #[test]
    fn parse_relative_date_weeks() {
        // Assuming today's date for testing purposes
        let today = Local::today().naive_local();
        let expected_date = today + Duration::weeks(1);

        // Test with a valid relative date of "+1w" (1 week from today)
        assert_eq!(parse_date_arg(Some("+1w")), Some(expected_date));
    }

    #[test]
    fn parse_relative_date_days() {
        // Assuming today's date for testing purposes
        let today = Local::today().naive_local();
        let expected_date = today - Duration::days(2);

        // Test with a valid relative date of "-2d" (2 days before today)
        assert_eq!(parse_date_arg(Some("-2d")), Some(expected_date));
    }

    #[test]
    fn parse_invalid_date() {
        // Test with an invalid date string
        assert_eq!(parse_date_arg(Some("invalid-date")), None);

        // Test with an invalid relative date format
        assert_eq!(parse_date_arg(Some("+1x")), None); // "x" is not a recognized duration unit
    }

    #[test]
    fn parse_none() {
        // Test with None as input
        assert_eq!(parse_date_arg(None), None);
    }
}
