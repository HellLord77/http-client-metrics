use std::fmt::Display;
use std::string::ToString;

use http::Response;
use http::StatusCode;
use http::header;
use metrics::Label;

use super::macros::define_http_response_status_code;
use super::macros::match_http_response_status_code;

const ERROR_TYPE: &str = "error.type";
const HTTP_RESPONSE_STATUS_CODE: &str = "http.response.status_code";

define_http_response_status_code! {
    CONTINUE,
    SWITCHING_PROTOCOLS,
    PROCESSING,
    EARLY_HINTS,

    OK,
    CREATED,
    ACCEPTED,
    NON_AUTHORITATIVE_INFORMATION,
    NO_CONTENT,
    RESET_CONTENT,
    PARTIAL_CONTENT,
    MULTI_STATUS,
    ALREADY_REPORTED,

    IM_USED,

    MULTIPLE_CHOICES,
    MOVED_PERMANENTLY,
    FOUND,
    SEE_OTHER,
    NOT_MODIFIED,
    USE_PROXY,
    TEMPORARY_REDIRECT,
    PERMANENT_REDIRECT,

    BAD_REQUEST,
    UNAUTHORIZED,
    PAYMENT_REQUIRED,
    FORBIDDEN,
    NOT_FOUND,
    METHOD_NOT_ALLOWED,
    NOT_ACCEPTABLE,
    PROXY_AUTHENTICATION_REQUIRED,
    REQUEST_TIMEOUT,
    CONFLICT,
    GONE,
    LENGTH_REQUIRED,
    PRECONDITION_FAILED,
    PAYLOAD_TOO_LARGE,
    URI_TOO_LONG,
    UNSUPPORTED_MEDIA_TYPE,
    RANGE_NOT_SATISFIABLE,
    EXPECTATION_FAILED,
    IM_A_TEAPOT,

    MISDIRECTED_REQUEST,
    UNPROCESSABLE_ENTITY,

    TOO_EARLY,

    UPGRADE_REQUIRED,

    PRECONDITION_REQUIRED,
    TOO_MANY_REQUESTS,

    REQUEST_HEADER_FIELDS_TOO_LARGE,

    UNAVAILABLE_FOR_LEGAL_REASONS,

    INTERNAL_SERVER_ERROR,
    NOT_IMPLEMENTED,
    BAD_GATEWAY,
    SERVICE_UNAVAILABLE,
    GATEWAY_TIMEOUT,
    HTTP_VERSION_NOT_SUPPORTED,
    VARIANT_ALSO_NEGOTIATES,
    INSUFFICIENT_STORAGE,
    LOOP_DETECTED,

    NOT_EXTENDED,
    NETWORK_AUTHENTICATION_REQUIRED,
}

pub(super) struct ResponseMetrics {
    http_client_response_body_size: u64,

    error_type: Option<String>,
    http_response_status_code: Option<StatusCode>,
}

impl ResponseMetrics {
    #[inline]
    pub(super) fn new<ResBody, Error>(res: &Result<Response<ResBody>, Error>) -> Self
    where
        Error: Display,
    {
        let http_client_response_body_size;

        let error_type;
        let http_response_status_code;

        match res {
            Ok(res) => {
                http_client_response_body_size = res
                    .headers()
                    .get(header::CONTENT_LENGTH)
                    .and_then(|header_value| header_value.to_str().ok())
                    .and_then(|string| string.parse().ok())
                    .unwrap_or(0);

                error_type = None;
                http_response_status_code = Some(res.status());
            }
            Err(err) => {
                http_client_response_body_size = 0;

                error_type = Some(err.to_string());
                http_response_status_code = None;
            }
        };

        Self {
            http_client_response_body_size,
            error_type,
            http_response_status_code,
        }
    }

    #[inline]
    pub(super) fn http_client_response_body_size(&self) -> u64 {
        self.http_client_response_body_size
    }

    pub(super) fn labels(self) -> impl Iterator<Item = Label> {
        [
            self.error_type
                .map(|error_type| Label::new(ERROR_TYPE, error_type)),
            self.http_response_status_code
                .as_ref()
                .map(http_response_status_code),
        ]
        .into_iter()
        .flatten()
    }
}

fn http_response_status_code(http_response_status_code: &StatusCode) -> Label {
    match_http_response_status_code! {
        *http_response_status_code,
        Label::new(HTTP_RESPONSE_STATUS_CODE, http_response_status_code.as_u16().to_string()),

        CONTINUE,
        SWITCHING_PROTOCOLS,
        PROCESSING,
        EARLY_HINTS,

        OK,
        CREATED,
        ACCEPTED,
        NON_AUTHORITATIVE_INFORMATION,
        NO_CONTENT,
        RESET_CONTENT,
        PARTIAL_CONTENT,
        MULTI_STATUS,
        ALREADY_REPORTED,

        IM_USED,

        MULTIPLE_CHOICES,
        MOVED_PERMANENTLY,
        FOUND,
        SEE_OTHER,
        NOT_MODIFIED,
        USE_PROXY,
        TEMPORARY_REDIRECT,
        PERMANENT_REDIRECT,

        BAD_REQUEST,
        UNAUTHORIZED,
        PAYMENT_REQUIRED,
        FORBIDDEN,
        NOT_FOUND,
        METHOD_NOT_ALLOWED,
        NOT_ACCEPTABLE,
        PROXY_AUTHENTICATION_REQUIRED,
        REQUEST_TIMEOUT,
        CONFLICT,
        GONE,
        LENGTH_REQUIRED,
        PRECONDITION_FAILED,
        PAYLOAD_TOO_LARGE,
        URI_TOO_LONG,
        UNSUPPORTED_MEDIA_TYPE,
        RANGE_NOT_SATISFIABLE,
        EXPECTATION_FAILED,
        IM_A_TEAPOT,

        MISDIRECTED_REQUEST,
        UNPROCESSABLE_ENTITY,

        TOO_EARLY,

        UPGRADE_REQUIRED,

        PRECONDITION_REQUIRED,
        TOO_MANY_REQUESTS,

        REQUEST_HEADER_FIELDS_TOO_LARGE,

        UNAVAILABLE_FOR_LEGAL_REASONS,

        INTERNAL_SERVER_ERROR,
        NOT_IMPLEMENTED,
        BAD_GATEWAY,
        SERVICE_UNAVAILABLE,
        GATEWAY_TIMEOUT,
        HTTP_VERSION_NOT_SUPPORTED,
        VARIANT_ALSO_NEGOTIATES,
        INSUFFICIENT_STORAGE,
        LOOP_DETECTED,

        NOT_EXTENDED,
        NETWORK_AUTHENTICATION_REQUIRED,
    }
}
