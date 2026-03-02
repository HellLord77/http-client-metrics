macro_rules! define_http_request_method {
    ($($variant:ident),+ $(,)?) => {
        ::pastey::paste! {
            $(
                const [<HTTP_REQUEST_METHOD_LABEL_ $variant>]: ::metrics::Label =
                    ::metrics::Label::from_static_parts(
                        HTTP_REQUEST_METHOD, ::core::stringify!($variant)
                    );
            )*
        }
    };
}

macro_rules! match_network_protocol_version {
    ($value:expr => $($variant:ident),+ $(,)?) => {
        ::pastey::paste! {
            match $value {
                $(
                    ::http::Version::[<HTTP $variant>] =>
                        ::core::option::Option::Some([<NETWORK_PROTOCOL_VERSION_LABEL $variant>]),
                )*
                _ => ::core::option::Option::None,
            }
        }
    };
}

macro_rules! define_url_scheme {
    ($($variant:ident),+ $(,)?) => {
        ::pastey::paste! {
            $(
                const [<URL_SCHEME_LABEL_ $variant>]: ::metrics::Label =
                    ::metrics::Label::from_static_parts(
                        URL_SCHEME, ::core::stringify!([<$variant:lower>])
                    );
            )*
        }
    };
}

macro_rules! match_url_scheme {
    ($value:expr => $($variant:ident),+ $(,)?) => {
        ::pastey::paste! {
            match $value.as_str() {
                $(
                    ::core::stringify!([<$variant:lower>]) =>
                        ::core::option::Option::Some([<URL_SCHEME_LABEL_ $variant>]),
                )*
                _ => ::core::option::Option::None,
            }
        }
    };
}

macro_rules! define_http_response_status_code {
    ($($variant:ident),+ $(,)?) => {
        ::pastey::paste! {
            $(
                const [<HTTP_RESPONSE_STATUS_CODE_LABEL_ $variant>]: ::metrics::Label =
                    ::metrics::Label::from_static_parts(
                        HTTP_RESPONSE_STATUS_CODE,
                        ::const_str::to_str!(::http::StatusCode::$variant.as_u16())
                    );
            )*
        }
    };
}

macro_rules! match_http_response_status_code {
    (
        $value:expr,
        $default:expr,
        $($variant:ident),+ $(,)?
    ) => {
        ::pastey::paste! {
            match $value {
                $(
                    ::http::StatusCode::$variant => [<HTTP_RESPONSE_STATUS_CODE_LABEL_ $variant>],
                )*
                _ => $default,
            }
        }
    };
}

pub(super) use define_http_request_method;
pub(super) use define_http_response_status_code;
pub(super) use define_url_scheme;
pub(super) use match_http_response_status_code;
pub(super) use match_network_protocol_version;
pub(super) use match_url_scheme;
