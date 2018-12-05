extern crate hyper;
extern crate hyper_rustls;
extern crate yup_oauth2 as oauth2;
extern crate google_calendar3 as calendar3;
extern crate notify_rust;

use events::notify_rust::{Notification, Timeout, server::NotificationServer};
use std::thread;
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
    pub fn create_event(&self, cal_event : super::parser::CalendarEvent) {
        let date = cal_event.datetime.to_rfc3339();
        let end = (cal_event.datetime + chrono::Duration::hours(1)).to_rfc3339();

        let event_date = EventDateTime{date_time: Some(date),
                                           time_zone: Some("America/Atikokan".to_string()),
                                           date: None};

        let end_date = EventDateTime{date_time: Some(end),
                                         time_zone: Some("America/Atikokan".to_string()),
                                         date: None};

        let mut event = Event::default();
        event.start = Some(event_date);
        event.end = Some(end_date);

        event.description = Some(cal_event.desc);
        event.summary = Some("Event Creation".to_string());
        let res = self.hub.events().insert(event, "primary").doit();
        match res {
            Ok(response) => {
                let target = response.0.status;
                let notification_res = if target.is_success() {
                    println!("Created new event!");
                    Notification::new()
                    .summary("Event Creation")
                    .body("Event Created Successfully")
                    .timeout(Timeout::Never)
                    .show()
                } else {
                     Notification::new()
                    .summary("Event Creation")
                    .body("Event Creation Unsuccessful")
                    .timeout(Timeout::Never)
                    .show()
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

pub fn create_hub() -> CalHub {
    const CLIENT_SECRET_FILE: &str = "client_id.json";
    let secret: ApplicationSecret = read_client_secret(CLIENT_SECRET_FILE);

    let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                            hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
                            <MemoryStorage as Default>::default(), None);
    let hub = CalendarHub::new(hyper::Client::with_connector(
                                    hyper::net::HttpsConnector::new(
                                        hyper_rustls::TlsClient::new())), auth);

    let server = NotificationServer::new();
    thread::spawn(move || NotificationServer::start(&server, |notification| println!("{:#?}", notification)));
    CalHub {hub}
}
