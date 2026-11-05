//! Tests of `outcome`.

use super::*;
use google_cloud_gax::error::rpc::Status;
use rstest::rstest;

fn refused_with(code: Code) -> Error {
    Error::service(Status::default().set_code(code).set_message("refused"))
}

#[test]
fn counts_a_new_task_created_and_an_existing_one_as_scheduled_once() {
    assert_eq!(created(Ok(())).unwrap(), Scheduled::Created);
    assert_eq!(
        created(Err(refused_with(Code::AlreadyExists))).unwrap(),
        Scheduled::AlreadyScheduled
    );
}

#[rstest]
#[case::permission_denied(Code::PermissionDenied)]
#[case::unavailable(Code::Unavailable)]
#[case::invalid_argument(Code::InvalidArgument)]
fn fails_any_other_refusal_to_create(#[case] code: Code) {
    assert!(created(Err(refused_with(code))).is_err());
}

#[test]
fn counts_a_task_already_gone_as_cancelled() {
    assert!(cancelled(Ok(())).is_ok());
    assert!(cancelled(Err(refused_with(Code::NotFound))).is_ok());
}

#[rstest]
#[case::permission_denied(Code::PermissionDenied)]
#[case::unavailable(Code::Unavailable)]
fn fails_any_other_refusal_to_cancel(#[case] code: Code) {
    assert!(cancelled(Err(refused_with(code))).is_err());
}
