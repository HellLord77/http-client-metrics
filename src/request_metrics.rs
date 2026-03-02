use http::Method;
use http::Request;
use http::Uri;
use http::Version;
use http::uri::Scheme;
use http_body::Body;
use metrics::Label;

use super::macros::define_http_request_method;
use super::macros::define_url_scheme;
use super::macros::match_network_protocol_version;
use super::macros::match_url_scheme;

const HTTP_REQUEST_METHOD: &str = "http.request.method";
const SERVER_ADDRESS: &str = "server.address";
const SERVER_PORT: &str = "server.port";
const NETWORK_PROTOCOL_NAME: &str = "network.protocol.name";
const NETWORK_PROTOCOL_VERSION: &str = "network.protocol.version";
const URL_SCHEME: &str = "url.scheme";

define_http_request_method! {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,

    PATCH,
    QUERY,
    _OTHER,
}

const NETWORK_PROTOCOL_NAME_LABEL: Label = Label::from_static_parts(NETWORK_PROTOCOL_NAME, "http");

const NETWORK_PROTOCOL_VERSION_LABEL_09: Label =
    Label::from_static_parts(NETWORK_PROTOCOL_VERSION, "0.9");
const NETWORK_PROTOCOL_VERSION_LABEL_10: Label =
    Label::from_static_parts(NETWORK_PROTOCOL_VERSION, "1.0");
const NETWORK_PROTOCOL_VERSION_LABEL_11: Label =
    Label::from_static_parts(NETWORK_PROTOCOL_VERSION, "1.1");
const NETWORK_PROTOCOL_VERSION_LABEL_2: Label =
    Label::from_static_parts(NETWORK_PROTOCOL_VERSION, "2");
const NETWORK_PROTOCOL_VERSION_LABEL_3: Label =
    Label::from_static_parts(NETWORK_PROTOCOL_VERSION, "3");

define_url_scheme!(HTTP, HTTPS);

pub(super) struct RequestMetrics {
    http_client_request_body_size: u64,

    http_request_method: Label,
    server_address: Option<String>,
    server_port: Option<u16>,
    network_protocol_version: Version,
    url_scheme: Option<Label>,
}

impl RequestMetrics {
    #[inline]
    pub(super) fn new<ReqBody>(req: &Request<ReqBody>) -> Self
    where
        ReqBody: Body,
    {
        Self {
            http_client_request_body_size: req.body().size_hint().lower(),
            http_request_method: http_request_method(req.method()),
            server_address: req.uri().host().map(ToString::to_string),
            server_port: port_or_known_default(req.uri()),
            network_protocol_version: req.version(),
            url_scheme: req.uri().scheme().and_then(url_scheme),
        }
    }

    #[inline]
    pub(super) fn http_client_request_body_size(&self) -> u64 {
        self.http_client_request_body_size
    }

    pub(super) fn labels(self) -> impl Iterator<Item = Label> {
        [
            Some(self.http_request_method),
            self.server_address
                .map(|server_address| Label::new(SERVER_ADDRESS, server_address)),
            self.server_port
                .map(|server_port| Label::new(SERVER_PORT, server_port.to_string())),
            Some(NETWORK_PROTOCOL_NAME_LABEL),
            network_protocol_version(&self.network_protocol_version),
            self.url_scheme,
        ]
        .into_iter()
        .flatten()
    }
}

fn http_request_method(http_request_method: &Method) -> Label {
    match *http_request_method {
        Method::GET => HTTP_REQUEST_METHOD_LABEL_GET,
        Method::HEAD => HTTP_REQUEST_METHOD_LABEL_HEAD,
        Method::POST => HTTP_REQUEST_METHOD_LABEL_POST,
        Method::PUT => HTTP_REQUEST_METHOD_LABEL_PUT,
        Method::DELETE => HTTP_REQUEST_METHOD_LABEL_DELETE,
        Method::CONNECT => HTTP_REQUEST_METHOD_LABEL_CONNECT,
        Method::OPTIONS => HTTP_REQUEST_METHOD_LABEL_OPTIONS,
        Method::TRACE => HTTP_REQUEST_METHOD_LABEL_TRACE,

        Method::PATCH => HTTP_REQUEST_METHOD_LABEL_PATCH,
        _ if http_request_method == "QUERY" => HTTP_REQUEST_METHOD_LABEL_QUERY,
        _ => HTTP_REQUEST_METHOD_LABEL__OTHER,
    }
}

fn network_protocol_version(network_protocol_version: &Version) -> Option<Label> {
    match_network_protocol_version!(*network_protocol_version => _09, _10, _11, _2, _3)
}

fn url_scheme(url_scheme: &Scheme) -> Option<Label> {
    match_url_scheme!(url_scheme => HTTP, HTTPS)
}

#[inline]
fn port_or_known_default(uri: &Uri) -> Option<u16> {
    uri.port_u16()
        .or_else(|| uri.scheme_str().and_then(default_port))
}

#[inline]
fn default_port(scheme: &str) -> Option<u16> {
    match scheme {
        "http" | "ws" => Some(80),
        "https" | "wss" => Some(443),
        "ftp" => Some(21),
        _ => None,
    }
}
