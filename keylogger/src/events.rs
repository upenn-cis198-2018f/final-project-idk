extern crate hyper;
extern crate hyper_rustls;
extern crate yup_oauth2 as oauth2;
extern crate google_calendar3 as calendar3;

use std::path::Path;
use events::calendar3::{ EventDateTime, Event, CalendarHub};
use std::default::Default;
use events::oauth2::{Authenticator, ApplicationSecret, DefaultAuthenticatorDelegate,
                     read_application_secret, MemoryStorage};

pub struct CalHub {
    hub: calendar3::CalendarHub<hyper::Client,
         Authenticator<DefaultAuthenticatorDelegate,
         MemoryStorage, hyper::Client>>,
}

impl CalHub {
    pub fn create_event<T: super::notification::Notifyer>(&self, cal_event : super::parser::CalendarEvent, notes: &T) {

        //Regularizing dates and durations, with the default duration of 1 hour
        let start = cal_event.start_time.to_rfc3339();
        let end = match cal_event.end_time {
            Some(time) => time.to_rfc3339(),
            None => (cal_event.start_time + chrono::Duration::hours(1)).to_rfc3339()
        };
        let event_date = EventDateTime{date_time: Some(start),
                                           time_zone: Some("America/Atikokan".to_string()),
                                           date: None};
        let end_date = EventDateTime{date_time: Some(end),
                                         time_zone: Some("America/Atikokan".to_string()),
                                         date: None};

        //Creating the event itself
        let mut event = Event::default();
        event.start = Some(event_date);
        event.end = Some(end_date);
        event.location = cal_event.location;
        event.description = Some(cal_event.desc);
        event.summary = Some("RustCal: New Event".to_string());

        //inserts event into primary calendar (Could potentially be made more interesting in the future)
        let res = self.hub.events().insert(event, "primary").doit();
        match res {
            Ok(response) => {
                let target = response.0.status;

                // Utilizes the Notifyer as the first notification method, then defaults to printing
                let notification_res = if target.is_success() {
                    notes.notify_success()
                } else {
                    notes.notify_failure()
                };
                match notification_res {
                    Ok(_) => {
                        ()  
                    },
                    Err(problem) => {
                        println!("Error on notif: {}", problem.to_string());
                    },
                };
            },
            Err(err) => {
                println!("Error on event creation: {}", err.to_string());
            }
        }
    }
}

fn read_client_secret(file: &str) -> ApplicationSecret {
    read_application_secret(Path::new(&file)).unwrap()
}

//Creates the initial calendar hub with the necessary secrets and authentication, mostly boilerplate
pub fn create_hub() -> CalHub {
    const CLIENT_SECRET_FILE: &str = "client_id.json";
    let secret: ApplicationSecret = read_client_secret(CLIENT_SECRET_FILE);

    let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                            hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
                            <MemoryStorage as Default>::default(), None);
    let hub = CalendarHub::new(hyper::Client::with_connector(
                                    hyper::net::HttpsConnector::new(
                                        hyper_rustls::TlsClient::new())), auth);

    CalHub {hub}
}
