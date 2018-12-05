use chrono::prelude::*;
use regex::{ Regex };

pub struct CalendarEvent {
    pub datetime: DateTime<Local>,
    pub desc: String,
    pub location: Option<String>
}

enum DateTimeElt {
    RelativeTime(NaiveTime),
    RelativeDate(NaiveDate),
    Today,
    Tomorrow,
    RelativeWeekday(Weekday)
}

fn parse_as_time(token : &str) -> Option<DateTimeElt> {
    let token = token.to_lowercase();

    // Looks for basic strings: "today", "tomorrow", and weekdays
    if token.eq("today") {
        return Some(DateTimeElt::Today)
    } else if token.eq("tomorrow") {
        return Some(DateTimeElt::Tomorrow)
    } else if let Ok(weekday) = token.parse::<Weekday>() {
        return Some(DateTimeElt::RelativeWeekday(weekday))
    }

    // Statically compiles regexes to decrease run-time overhead
    lazy_static! {
        static ref AM : Regex = Regex::new(r"^(\d{1, 2})(:(\d{2}))?am?$").unwrap();
        static ref PM : Regex = Regex::new(r"^(\d{1, 2})(:(\d{2}))?pm?$").unwrap();
        static ref DATE : Regex = Regex::new(r"^(\d{1, 2})[-/](\d{1, 2})([-/]\d{4})?$").unwrap();
        static ref DATE2 : Regex = Regex::new(r"^([a-z]+)(\d{1, 2})$").unwrap();
    }

    // Parses for a time of the form __am or __:__am
    if let Some(caps) = AM.captures(token.as_str()) {
        let hours = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        if hours >= 1 && hours <= 12 {
            if let Some(minutes) = caps.get(2) {
                let minutes = minutes.as_str()[1..3].parse::<u32>().unwrap();
                if minutes <= 60 {
                    return Some(DateTimeElt::RelativeTime(NaiveTime::from_hms(hours, minutes, 0)));
                }
            } else {
                return Some(DateTimeElt::RelativeTime(NaiveTime::from_hms(hours, 0, 0)));
            }
        }
    }

    // Parses for a time of the form __pm or __:__pm
    if let Some(caps) = PM.captures(token.as_str()) {
        let hours = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        if hours >= 1 && hours <= 12 {
            if let Some(minutes) = caps.get(2) {
                let minutes = minutes.as_str()[1..3].parse::<u32>().unwrap();
                if minutes <= 60 {
                    return Some(DateTimeElt::RelativeTime(NaiveTime::from_hms(hours + 12, minutes, 0)));
                }
            } else {
                return Some(DateTimeElt::RelativeTime(NaiveTime::from_hms(hours + 12, 0, 0)));
            }
        }
    }

    // Parses for a date of the form mm/dd or mm/dd/yyyy
    if let Some(caps) = DATE.captures(token.as_str()) {
        let month = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let day = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let year = if let Some(year_str) = caps.get(3) {
            year_str.as_str()[1..5].parse::<i32>().unwrap()
        } else {
            Local::today().year()
        };
        if let Some(time) = NaiveDate::from_ymd_opt(year, month, day) {
            return Some(DateTimeElt::RelativeDate(time));
        }
    }

    None
}

fn with_date(input : DateTime<Local>, date : NaiveDate) -> DateTime<Local> {
    input
        .with_day(date.day()).expect("with_date : invalid day")
        .with_month(date.month()).expect("with_date : invalid month")
        .with_year(date.year()).expect("with_date : invalid year")
}

fn with_time(input : DateTime<Local>, time : NaiveTime) -> DateTime<Local> {
    input
        .with_hour(time.hour()).expect("with_time : invalid hour")
        .with_minute(time.minute()).expect("with_time : invalid minute")
        .with_second(time.second()).expect("with_time : invalid second")
}

fn first_weekday_after_today(weekday : Weekday) -> Date<Local> {
    // As far as I can tell, there does not exist a better way to do this. I'm sorry.
    let todays_weekday = Local::today().weekday();
    let num_days_from_monday = (7 + weekday.num_days_from_monday() - todays_weekday.num_days_from_monday()) % 7;
    let mut date = Local::today();
    for _ in 0..num_days_from_monday {
        date = date.succ();
    };
    date
}

fn transform_date(input : DateTime<Local>, elt : &DateTimeElt) -> DateTime<Local> {
    match elt {
        DateTimeElt::Today => with_date(input, Local::today().naive_local()),
        DateTimeElt::Tomorrow => with_date(input, Local::today().succ().naive_local()),
        DateTimeElt::RelativeWeekday(weekday) =>
            with_date(input, first_weekday_after_today(*weekday).naive_local()),
        DateTimeElt::RelativeDate(date) => with_date(input, *date),
        DateTimeElt::RelativeTime(time) => with_time(input, *time)
    }
}

enum ParseTarget {
    Description, Date, Location
}

pub fn parse(input : &str) -> Option<CalendarEvent> {
    // Empty strings to hold return values
    let mut desc = String::with_capacity(input.len());
    let mut location = String::new();
    let mut datetime = Local::today().and_hms(9, 0, 0);

    let mut parsing_target = ParseTarget::Description;
    let mut changed_time = false;

    for token in input.split_whitespace() {
        // Attempts to parse each token as a time
        // Unsuccesful parses are added to location/description
        match parse_as_time(&token.to_string()) {
            Some(datetime_elt) => {
                parsing_target = ParseTarget::Date;
                datetime = transform_date(datetime, &datetime_elt);
                changed_time = true;
            },
            None => {
                if token == "at" {
                    parsing_target = ParseTarget::Location;
                    continue
                }
                match parsing_target {
                    ParseTarget::Description => {
                        desc.push_str(token);
                        desc.push(' ')
                    },
                    ParseTarget::Location => {
                        location.push_str(token);
                        location.push(' ')
                    }
                    ParseTarget::Date => continue,
                }
            }
        }
    }

    if changed_time {
        desc.pop(); // Remove extra ' ' from end
        if location.is_empty() {
            Some(CalendarEvent { datetime, desc, location : None })
        } else {
            location.pop();
            Some(CalendarEvent { datetime, desc, location : Some(location) })
        }
    } else {
        None // If we never parsed any date info, don't allow it
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(input : &str, exp_datetime : String, exp_desc : &str) {
        let out = parse(input);
        match out {
            Some(CalendarEvent { datetime, desc, location : None }) => {
                assert_eq!(desc, exp_desc);
                assert_eq!(datetime.to_rfc3339(), exp_datetime)
            },
            _ => assert!(false)
        }
    }

    fn test_with_location(input : &str, exp_datetime : String, exp_desc : &str, exp_location : &str) {
        let out = parse(input);
        match out {
            Some(CalendarEvent { datetime, desc, location : Some(location) }) => {
                assert_eq!(desc, exp_desc);
                assert_eq!(location, exp_location);
                assert_eq!(datetime.to_rfc3339(), exp_datetime)
            },
            _ => assert!(false)
        }
    }

    fn failing_test(input : &str) {
        let out = parse(input);
        assert!(out.is_none())
    }

    #[test]
    fn empty_as_my_heart() {
        failing_test("");
    }

    #[test]
    fn today() {
        test("help me today",
             Local::today().and_hms(9, 0, 0).to_rfc3339(),
             "help me");
    }

    #[test]
    fn tomorrow() {
        test("help me tomorrow",
             Local::today().succ().and_hms(9, 0, 0).to_rfc3339(),
             "help me");
    }

    #[test]
    fn ignores_extra() {
        test("you only see this today not this",
             Local::today().and_hms(9, 0, 0).to_rfc3339(),
             "you only see this");
    }

    #[test]
    fn am1_10am() {
        test("test 10am",
             Local::today().and_hms(10, 0, 0).to_rfc3339(),
             "test");
    }

    #[test]
    fn am1_8am() {
        test("test 8am",
             Local::today().and_hms(8, 0, 0).to_rfc3339(),
             "test");
    }

    #[test]
    fn am1_1030am() {
        test("test 10:30am",
             Local::today().and_hms(10, 30, 0).to_rfc3339(),
             "test");
    }

    #[test]
    fn am1_13am() {
        failing_test("test 13am");
    }

    #[test]
    fn pm1_10am() {
        test("test 10pm",
             Local::today().and_hms(22, 0, 0).to_rfc3339(),
             "test");
    }

    #[test]
    fn pm1_8am() {
        test("test 8p",
             Local::today().and_hms(20, 0, 0).to_rfc3339(),
             "test");
    }

    #[test]
    fn pm_108p() {
        test("test 1:08p",
             Local::today().and_hms(13, 8, 0).to_rfc3339(),
             "test");
    }

    #[test]
    fn am_garbage() {
        failing_test("test asdf8a");
    }

    #[test]
    fn am1_moregarbage() {
        failing_test("test 10:333am");
    }

    #[test]
    fn pm_garbage() {
        failing_test("test 8pasdf");
    }

    #[test]
    fn weekday1() {
        test("test sun",
             "2018-12-09T09:00:00+00:00".to_string(),
             "test");
    }

    #[test]
    fn weekday2() {
        test("test monday 10p",
             "2018-12-10T22:00:00+00:00".to_string(),
             "test");
    }

    #[test]
    fn date1() {
        test("test 9-12",
             "2018-09-12T09:00:00+00:00".to_string(),
             "test");
    }

    #[test]
    fn date2() {
        test("test 10-12 10p garbage for fun",
             "2018-10-12T22:00:00+00:00".to_string(),
             "test");
    }

    #[test]
    fn date_next_year() {
        test("test asdf 10-12-2019 10p garbage for fun",
             "2019-10-12T22:00:00+00:00".to_string(),
             "test asdf");
    }

    #[test]
    fn date_location() {
        test_with_location("test asdf at the park 10-12-2019 10p garbage for fun",
                           "2019-10-12T22:00:00+00:00".to_string(),
                           "test asdf",
                           "the park");
    }

    #[test]
    fn date_location() {
        test_with_location("test asdf at the park 10-12-2019 10p garbage for fun",
                           "2019-10-12T22:00:00+00:00".to_string(),
                           "test asdf",
                           "the park");
    }
}
