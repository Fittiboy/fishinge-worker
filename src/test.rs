use super::*;

#[test]
fn webhook_ok() {
    let headers = TwitchHeaders {
        id: "ID".to_string(),
        timestamp: "TIMESTAMP".to_string(),
        signature: "sha256=6883f95e18aaf4ff2e84bbbf47640f2ab719c18f92bba0b595146a11e6a49aef"
            .to_string(),
        ..Default::default()
    };
    webhook("test", &headers, "BODY").unwrap();
}

#[test]
#[should_panic]
fn error_on_bad_signature() {
    let headers = TwitchHeaders {
        id: "ID".to_string(),
        timestamp: "TIMESTAMP".to_string(),
        signature: "sha256=6883f95e18aaf4ff2e84bbbf47640f2ab719c18f92bba0b595146a11e6a49ae" //truncated
            .to_string(),
        ..Default::default()
    };
    webhook("test", &headers, "BODY").unwrap();
}
