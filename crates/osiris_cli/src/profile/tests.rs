//! Saving the profile.

#![allow(
    clippy::unwrap_used,
    reason = "tests: a failure here is the test failing"
)]

use super::{Profile, save::save_to};
use fake::{Fake, faker::internet::en::Password};

fn temporary_path() -> std::path::PathBuf {
    std::env::temp_dir()
        .join(format!(
            "osiris-cli-{}",
            16.fake::<String>()
                .replace(|c: char| !c.is_ascii_alphanumeric(), "x")
        ))
        .join("osiris")
        .join("config.toml")
}

#[test]
fn saves_the_keys_readable_by_their_owner_only_and_reads_them_back() {
    let path = temporary_path();
    let profile = Profile {
        url: "http://localhost:8090".into(),
        api_key: Some(Password(32..40).fake()),
        admin_key: Some(Password(32..40).fake()),
        ..Profile::default()
    };

    save_to(&path, &profile).unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let file = std::fs::metadata(&path).unwrap().permissions().mode() & 0o777;
        let directory = std::fs::metadata(path.parent().unwrap())
            .unwrap()
            .permissions()
            .mode()
            & 0o777;
        assert_eq!(file, 0o600);
        assert_eq!(directory, 0o700);
    }
    let read: Profile = toml::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
    assert_eq!(read.api_key, profile.api_key);
    std::fs::remove_dir_all(path.parent().unwrap().parent().unwrap()).unwrap();
}

#[cfg(unix)]
#[test]
fn tightens_an_older_profile_with_looser_permissions_before_writing_the_keys() {
    use std::os::unix::fs::PermissionsExt;
    let path = temporary_path();
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, "").unwrap();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();

    save_to(&path, &Profile::default()).unwrap();

    assert_eq!(
        std::fs::metadata(&path).unwrap().permissions().mode() & 0o777,
        0o600
    );
    std::fs::remove_dir_all(path.parent().unwrap().parent().unwrap()).unwrap();
}
