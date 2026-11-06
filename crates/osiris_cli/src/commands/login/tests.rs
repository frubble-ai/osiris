//! Tests of `login`.

use super::*;
use fake::{Fake, faker::internet::en::Password};

fn no_flags() -> Login {
    Login {
        url: None,
        api_key: None,
        admin_key: None,
        pubsub_project: None,
        events_topic: None,
        pubsub_emulator_host: None,
    }
}

fn saved() -> Profile {
    Profile {
        url: "https://osiris.example".into(),
        api_key: Some(Password(32..40).fake()),
        admin_key: Some(Password(32..40).fake()),
        pubsub_project: Some("project".into()),
        events_topic: Some("topic".into()),
        pubsub_emulator_host: Some("localhost:8086".into()),
    }
}

#[test]
fn points_a_new_profile_at_the_default_url() {
    let profile = no_flags().merged_into(None);

    assert_eq!(profile.url, DEFAULT_URL);
    assert!(profile.api_key.is_none());
}

#[test]
fn keeps_every_saved_value_for_a_flag_left_out() {
    let old = saved();

    let profile = no_flags().merged_into(Some(old.clone()));

    assert_eq!(profile.url, old.url);
    assert_eq!(profile.api_key, old.api_key);
    assert_eq!(profile.admin_key, old.admin_key);
    assert_eq!(profile.pubsub_project, old.pubsub_project);
    assert_eq!(profile.events_topic, old.events_topic);
    assert_eq!(profile.pubsub_emulator_host, old.pubsub_emulator_host);
}

#[test]
fn replaces_only_the_values_given() {
    let old = saved();
    let new_key: String = Password(32..40).fake();
    let login = Login {
        api_key: Some(new_key.clone()),
        url: Some("http://localhost:9000".into()),
        ..no_flags()
    };

    let profile = login.merged_into(Some(old.clone()));

    assert_eq!(profile.api_key, Some(new_key));
    assert_eq!(profile.url, "http://localhost:9000");
    assert_eq!(profile.admin_key, old.admin_key);
}

#[test]
fn falls_back_to_the_default_url_when_the_saved_one_is_empty() {
    let old = Profile {
        url: String::new(),
        ..saved()
    };

    assert_eq!(no_flags().merged_into(Some(old)).url, DEFAULT_URL);
}
