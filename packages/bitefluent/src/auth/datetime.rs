use chrono::{DateTime, Utc};
use dioxus_auth::{AuthError, AuthResult};
use time::OffsetDateTime;

pub fn chrono_to_time(value: DateTime<Utc>) -> AuthResult<OffsetDateTime> {
    OffsetDateTime::from_unix_timestamp_nanos(
        value
            .timestamp_nanos_opt()
            .ok_or_else(|| AuthError::Adapter("invalid chrono timestamp".to_string()))?
            as i128,
    )
    .map_err(|err| AuthError::Adapter(err.to_string()))
}

pub fn chrono_to_time_option(value: Option<DateTime<Utc>>) -> AuthResult<Option<OffsetDateTime>> {
    value.map(chrono_to_time).transpose()
}

pub fn time_to_chrono(value: OffsetDateTime) -> AuthResult<DateTime<Utc>> {
    DateTime::<Utc>::from_timestamp(value.unix_timestamp(), value.nanosecond())
        .ok_or_else(|| AuthError::Adapter("invalid time timestamp".to_string()))
}

pub fn time_to_chrono_option(value: Option<OffsetDateTime>) -> AuthResult<Option<DateTime<Utc>>> {
    value.map(time_to_chrono).transpose()
}
