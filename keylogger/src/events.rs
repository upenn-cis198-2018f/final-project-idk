extern crate hyper;
extern crate hyper_rustls;
extern crate yup_oauth2 as oauth2;
extern crate google_calendar3 as calendar3;
extern crate notify_rust;

use events::notify_rust::Notification;
use std::path::Path;
use events::calendar3::{Calendar, EventDateTime, Event, Channel, Result, Error, CalendarHub};
use std::default::Default;
use events::oauth2::{Authenticator, FlowType, ApplicationSecret, DiskTokenStorage,
                 DefaultAuthenticatorDelegate, read_application_secret, MemoryStorage};
use std::option::Option;
use events::hyper::status::{StatusClass, StatusCode};
use events::hyper::client::response::Response;

struct CalHub {
    hub: calendar3::CalendarHub<hyper::Client,
         Authenticator<DefaultAuthenticatorDelegate,
         MemoryStorage, hyper::Client>>,
}

impl CalHub {
    fn create_event(&self, cal_event : super::parser::CalendarEvent) {
        let date = cal_event.datetime.to_rfc3339();
        let end = (cal_event.datetime + chrono::Duration::hours(1)).to_rfc3339();

        let mut event_date = EventDateTime{date_time: Some(date),
                                           time_zone: Some("America/Atikokan".to_string()),
                                           date: None};

        let mut end_date = EventDateTime{date_time: Some(end),
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
                if target.is_success() {
                    println!("HERE!!!!");
                    Notification::new()
                    .summary("Event Creation")
                    .body("Event Created Successfully")
                    .show();
                }
                else {
                     Notification::new()
                    .summary("Event Creation")
                    .body("Event Creation Unsuccessful")
                    .show();
                }
            },
            Err(err) => {

            }
        }
    }
}

fn read_client_secret(file: String) -> ApplicationSecret {
    read_application_secret(Path::new(&file)).unwrap()
}

fn create_hub() -> CalHub {
    const CLIENT_SECRET_FILE: &'static str = "client_id.json";
    let secret: ApplicationSecret = read_client_secret(CLIENT_SECRET_FILE.to_string());

    let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                            hyper::Client::with_connector(hyper::net::HttpsConnector::new(hyper_rustls::TlsClient::new())),
                            <MemoryStorage as Default>::default(), None);
    let mut hub = CalendarHub::new(hyper::Client::with_connector(
                                    hyper::net::HttpsConnector::new(
                                    hyper_rustls::TlsClient::new())), auth);
    return CalHub {hub: hub}
}
