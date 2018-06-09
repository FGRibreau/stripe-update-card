
extern crate serde_json;

use rocket::local::Client;
use la_metric::LaMetricResponse;
use la_metric::LaMetricFrame;
use la_metric::TextFrame;

lazy_static! {
    #[derive(Copy, Clone, Debug)]
    static ref RSS_FEED: String = "http://www.mocky.io/v2/59fcfedb310000cb1b4fc7a9".to_string();
}

fn get_lametric_response(url: String) -> LaMetricResponse {
    let rocket = super::rocket();
    let client = Client::new(rocket).expect("Rocket client");
    let mut res = client.get(url).dispatch();
    res.body_string()
        .map(|x: String| serde_json::from_str(&x).unwrap())
        .expect("Response should be deserializable to LaMetricResponse")
}

fn get(url: &str) -> String {
    let rocket = super::rocket();
    let client = Client::new(rocket).expect("Rocket client");
    let mut res = client.get(url).dispatch();
    res.body_string().expect("Response should be a string")
}


#[test]
fn test_main_route() {
    assert!(get("/").contains("rss-to-lametric"));
}


#[test]
fn test_custom_title_and_icon_with_empty_rss() {
    assert_eq!(
        get_lametric_response(format!(
            "/convert/?title=Custom&icon=icon&limit=0&url={}",
            &**RSS_FEED
        )),
        LaMetricResponse {
            frames: vec![
                LaMetricFrame::TextFrame(TextFrame {
                    text: "Custom".to_string(),
                    icon: Some("icon".to_string()),
                }),
            ],
        }
    );
}

#[test]
fn test_invalid_rss() {
    let resp: LaMetricResponse = get_lametric_response(format!(
        "/convert/?title=Custom&icon=icon&limit=0&url=http://127.0.0.1.com/plop"
    ));

    assert_eq!(resp.frames.len(), 2);

    let last = &resp.frames[1];

    let text_frame = match last {
        LaMetricFrame::TextFrame(text_frame) => text_frame,
    };

    assert!(text_frame.text.contains(
        "http://127.0.0.1.com/plop: failed to lookup address information",
    ))
}

#[test]
fn test_valid_la_metric_data_output() {
    assert_eq!(
        get_lametric_response(format!(
            "/convert/?title=Ouest-France&icon=i14532&limit=4&url={}",
            &**RSS_FEED
        )),
        LaMetricResponse {
            frames: vec![
                LaMetricFrame::TextFrame(TextFrame {
                    text: "Ouest-France".to_string(),
                    icon: Some("i14532".to_string()),
                }),
                LaMetricFrame::TextFrame(TextFrame {
                    text: "Stade Rennais. Le président du club René Ruello annonce sa démission"
                        .to_string(),
                    icon: None,
                }),
                LaMetricFrame::TextFrame(TextFrame {
                    text: "Direction de LREM. 4 listes en compétition pour le bureau exécutif"
                        .to_string(),
                    icon: None,
                }),
                LaMetricFrame::TextFrame(TextFrame {
                    text: "La police de New York a un \"vrai dossier\" sur Weinstein".to_string(),
                    icon: None,
                }),
                LaMetricFrame::TextFrame(TextFrame {
                    text: "Ligue 1. Le Stade Rennais poursuit sa belle série face à Bordeaux"
                        .to_string(),
                    icon: None,
                }),
            ],
        }
    );
}

#[test]
fn test_valid_la_metric_json_output() {
    assert_eq!(
        get(&format!(
            "/convert/?title=Ouest-France&icon=i14532&limit=4&url={}",
            &**RSS_FEED
        )),
        "{\"frames\":[{\"text\":\"Ouest-France\",\"icon\":\"i14532\"},{\"text\":\"Stade Rennais. Le président du club René Ruello annonce sa démission\",\"icon\":null},{\"text\":\"Direction de LREM. 4 listes en compétition pour le bureau exécutif\",\"icon\":null},{\"text\":\"La police de New York a un \\\"vrai dossier\\\" sur Weinstein\",\"icon\":null},{\"text\":\"Ligue 1. Le Stade Rennais poursuit sa belle série face à Bordeaux\",\"icon\":null}]}"
    );
}
