#[derive(Clone, PartialEq, Debug)]
pub struct StatusCode(pub(crate) usize, pub(crate) &'static str);

pub const DEFAULT_HTTP_PORT: u16 = 80;
pub const DEFAULT_HTTPS_PORT: u16 = 443;

impl StatusCode {
    pub const CONTINUE: Self = Self(S_CONTINUE, M_CONTINUE);
    pub const SWITCHING_PROTOCOLS: Self = Self(S_SWITCHING_PROTOCOLS, M_SWITCHING_PROTOCOLS);
    pub const OK: Self = Self(S_OK, M_OK);
    pub const CREATED: Self = Self(S_CREATED, M_CREATED);
    pub const ACCEPTED: Self = Self(S_ACCEPTED, M_ACCEPTED);
    pub const NON_AUTHORITATIVE: Self = Self(S_NON_AUTHORITATIVE, M_NON_AUTHORITATIVE);
    pub const NO_CONTENT: Self = Self(S_NO_CONTENT, M_NO_CONTENT);
    pub const RESET_CONTENT: Self = Self(S_RESET_CONTENT, M_RESET_CONTENT);
    pub const PARTIAL_CONTENT: Self = Self(S_PARTIAL_CONTENT, M_PARTIAL_CONTENT);
    pub const MULTIPLE_CHOICES: Self = Self(S_MULTIPLE_CHOICES, M_MULTIPLE_CHOICES);
    pub const MOVED_PERMANENTLY: Self = Self(S_MOVED_PERMANENTLY, M_MOVED_PERMANENTLY);
    pub const FOUND: Self = Self(S_FOUND, M_FOUND);
    pub const SEE_OTHER: Self = Self(S_SEE_OTHER, M_SEE_OTHER);
    pub const NOT_MODIFIED: Self = Self(S_NOT_MODIFIED, M_NOT_MODIFIED);
    pub const USE_PROXY: Self = Self(S_USE_PROXY, M_USE_PROXY);
    pub const TEMPORARY_REDIRECT: Self = Self(S_TEMPORARY_REDIRECT, M_TEMPORARY_REDIRECT);
    pub const BAD_REQUEST: Self = Self(S_BAD_REQUEST, M_BAD_REQUEST);
    pub const UNAUTHORIZED: Self = Self(S_UNAUTHORIZED, M_UNAUTHORIZED);
    pub const PAYMENT_REQUIRED: Self = Self(S_PAYMENT_REQUIRED, M_PAYMENT_REQUIRED);
    pub const FORBIDDEN: Self = Self(S_FORBIDDEN, M_FORBIDDEN);
    pub const NOT_FOUND: Self = Self(S_NOT_FOUND, M_NOT_FOUND);
    pub const METHOD_NOT_ALLOWED: Self = Self(S_METHOD_NOT_ALLOWED, M_METHOD_NOT_ALLOWED);
    pub const NOT_ACCEPTABLE: Self = Self(S_NOT_ACCEPTABLE, M_NOT_ACCEPTABLE);
    pub const PROXY_AUTHENTICATION_REQUIRED: Self = Self(
        S_PROXY_AUTHENTICATION_REQUIRED,
        M_PROXY_AUTHENTICATION_REQUIRED,
    );
    pub const REQUEST_TIME_OUT: Self = Self(S_REQUEST_TIME_OUT, M_REQUEST_TIME_OUT);
    pub const CONFLICT: Self = Self(S_CONFLICT, M_CONFLICT);
    pub const GONE: Self = Self(S_GONE, M_GONE);
    pub const LENGTH_REQUIRED: Self = Self(S_LENGTH_REQUIRED, M_LENGTH_REQUIRED);
    pub const PRECONDITION_FAILED: Self = Self(S_PRECONDITION_FAILED, M_PRECONDITION_FAILED);
    pub const REQUEST_ENTITY_TOO_LARGE: Self =
        Self(S_REQUEST_ENTITY_TOO_LARGE, M_REQUEST_ENTITY_TOO_LARGE);
    pub const REQUEST_URI_TOO_LARGE: Self = Self(S_REQUEST_URI_TOO_LARGE, M_REQUEST_URI_TOO_LARGE);
    pub const UNSUPPORTED_MEDIA_TYPE: Self =
        Self(S_UNSUPPORTED_MEDIA_TYPE, M_UNSUPPORTED_MEDIA_TYPE);
    pub const REQUESTED_RANGE_NOT_SATISFIABLE: Self = Self(
        S_REQUESTED_RANGE_NOT_SATISFIABLE,
        M_REQUESTED_RANGE_NOT_SATISFIABLE,
    );
    pub const EXPECTATION_FAILED: Self = Self(S_EXPECTATION_FAILED, M_EXPECTATION_FAILED);
    pub const INTERNAL_SERVER_ERROR: Self = Self(S_INTERNAL_SERVER_ERROR, M_INTERNAL_SERVER_ERROR);
    pub const NOT_IMPLEMENTED: Self = Self(S_NOT_IMPLEMENTED, M_NOT_IMPLEMENTED);
    pub const BAD_GATEWAY: Self = Self(S_BAD_GATEWAY, M_BAD_GATEWAY);
    pub const SERVICE_UNAVAILABLE: Self = Self(S_SERVICE_UNAVAILABLE, M_SERVICE_UNAVAILABLE);
    pub const GATEWAY_TIME_OUT: Self = Self(S_GATEWAY_TIME_OUT, M_GATEWAY_TIME_OUT);
    pub const HTTP_VERSION_NOT_SUPPORTED: Self =
        Self(S_HTTP_VERSION_NOT_SUPPORTED, M_HTTP_VERSION_NOT_SUPPORTED);
}

impl PartialEq<usize> for StatusCode {
    fn eq(&self, other: &usize) -> bool {
        &self.0 == other
    }
}
impl PartialEq<StatusCode> for usize {
    fn eq(&self, other: &StatusCode) -> bool {
        self == &other.0
    }
}

pub const H_ACCEPT: &str = "Accept";
pub const H_ACCEPT_CHARSET: &str = "Accept-Charset";
pub const H_ACCEPT_LANGUAGE: &str = "Accept-Language";
pub const H_ACCEPT_ENCODING: &str = "Accept-Encoding";
pub const H_ACCEPT_RANGES: &str = "Accept-Ranges";
pub const H_AGE: &str = "Age";
pub const H_ALLOW: &str = "Allow";
pub const H_AUTHORIZATION: &str = "Authorization";
pub const H_CACHE_CONTROL: &str = "Cache-Control";
pub const H_CONNECTION: &str = "Connection";
pub const H_CONTENT_ENCODING: &str = "Content-Encoding";
pub const H_CONTENT_LANGUAGE: &str = "Content-Language";
pub const H_CONTENT_LENGTH: &str = "Content-Length";
pub const H_CONTENT_LOCATION: &str = "Content-Location";
pub const H_CONTENT_MD5: &str = "Content-MD5";
pub const H_CONTENT_RANGE: &str = "Content-Range";
pub const H_CONTENT_TYPE: &str = "Content-Type";
pub const H_DATE: &str = "Date";
pub const H_ETAG: &str = "ETag";
pub const H_EXPECT: &str = "Expect";
pub const H_EXPIRES: &str = "Expires";
pub const H_FROM: &str = "From";
pub const H_HOST: &str = "Host";
pub const H_IF_MATCH: &str = "If-Match";
pub const H_IF_MODIFIED_SINCE: &str = "if-Modified-Since";
pub const H_IF_NONE_MATCH: &str = "If-None-Match";
pub const H_IF_RANGE: &str = "If-Range";
pub const H_IF_UNMODIFIED_SINCE: &str = "If-Unmodified-Since";
pub const H_LAST_MODIFIED: &str = "Last-Modified";
pub const H_LOCATION: &str = "Location";
pub const H_MAX_FORWARDS: &str = "Max-Forwards";
pub const H_PRAGMA: &str = "Pragma";
pub const H_PROXY_AUTHENTICATE: &str = "Proxy-Authenticate";
pub const H_PROXY_AUTHORIZATION: &str = "Proxy-Authorization";
pub const H_RANGE: &str = "Range";
pub const H_REFERER: &str = "Referer";
pub const H_SERVER: &str = "Server";
pub const H_TE: &str = "TE";
pub const H_TRAILER: &str = "Trailer";
pub const H_TRANSFER_ENCODING: &str = "Transfer-Encoding";
pub const H_UPGRADE: &str = "Upgrade";
pub const H_USER_AGENT: &str = "User-Agent";
pub const H_VARY: &str = "Vary";
pub const H_VIA: &str = "Via";
pub const H_WARNING: &str = "Warning";
pub const H_WWW_AUTHENTICATE: &str = "WWW-Authenticate";

pub const S_CONTINUE: usize = 100;
pub const S_SWITCHING_PROTOCOLS: usize = 101;
pub const S_OK: usize = 200;
pub const S_CREATED: usize = 201;
pub const S_ACCEPTED: usize = 202;
pub const S_NON_AUTHORITATIVE: usize = 203;
pub const S_NO_CONTENT: usize = 204;
pub const S_RESET_CONTENT: usize = 205;
pub const S_PARTIAL_CONTENT: usize = 206;
pub const S_MULTIPLE_CHOICES: usize = 300;
pub const S_MOVED_PERMANENTLY: usize = 301;
pub const S_FOUND: usize = 302;
pub const S_SEE_OTHER: usize = 303;
pub const S_NOT_MODIFIED: usize = 304;
pub const S_USE_PROXY: usize = 305;
pub const S_TEMPORARY_REDIRECT: usize = 307;
pub const S_BAD_REQUEST: usize = 400;
pub const S_UNAUTHORIZED: usize = 401;
pub const S_PAYMENT_REQUIRED: usize = 402;
pub const S_FORBIDDEN: usize = 403;
pub const S_NOT_FOUND: usize = 404;
pub const S_METHOD_NOT_ALLOWED: usize = 405;
pub const S_NOT_ACCEPTABLE: usize = 406;
pub const S_PROXY_AUTHENTICATION_REQUIRED: usize = 407;
pub const S_REQUEST_TIME_OUT: usize = 408;
pub const S_CONFLICT: usize = 409;
pub const S_GONE: usize = 410;
pub const S_LENGTH_REQUIRED: usize = 411;
pub const S_PRECONDITION_FAILED: usize = 412;
pub const S_REQUEST_ENTITY_TOO_LARGE: usize = 413;
pub const S_REQUEST_URI_TOO_LARGE: usize = 414;
pub const S_UNSUPPORTED_MEDIA_TYPE: usize = 415;
pub const S_REQUESTED_RANGE_NOT_SATISFIABLE: usize = 416;
pub const S_EXPECTATION_FAILED: usize = 417;
pub const S_INTERNAL_SERVER_ERROR: usize = 500;
pub const S_NOT_IMPLEMENTED: usize = 501;
pub const S_BAD_GATEWAY: usize = 502;
pub const S_SERVICE_UNAVAILABLE: usize = 503;
pub const S_GATEWAY_TIME_OUT: usize = 504;
pub const S_HTTP_VERSION_NOT_SUPPORTED: usize = 505;

pub const M_CONTINUE: &str = "Continue";
pub const M_SWITCHING_PROTOCOLS: &str = "Switching Protocols";
pub const M_OK: &str = "OK";
pub const M_CREATED: &str = "Created";
pub const M_ACCEPTED: &str = "Accepted";
pub const M_NON_AUTHORITATIVE: &str = "Non-Authoritative Information";
pub const M_NO_CONTENT: &str = "No Content";
pub const M_RESET_CONTENT: &str = "Reset Content";
pub const M_PARTIAL_CONTENT: &str = "Partial Content";
pub const M_MULTIPLE_CHOICES: &str = "Multiple Choices";
pub const M_MOVED_PERMANENTLY: &str = "Moved Permanently";
pub const M_FOUND: &str = "Found";
pub const M_SEE_OTHER: &str = "See Other";
pub const M_NOT_MODIFIED: &str = "Not Modified";
pub const M_USE_PROXY: &str = "Use Proxy";
pub const M_TEMPORARY_REDIRECT: &str = "Temporary Redirect";
pub const M_BAD_REQUEST: &str = "Bad Request";
pub const M_UNAUTHORIZED: &str = "Unauthorized";
pub const M_PAYMENT_REQUIRED: &str = "Payment Required";
pub const M_FORBIDDEN: &str = "Forbidden";
pub const M_NOT_FOUND: &str = "Not Found";
pub const M_METHOD_NOT_ALLOWED: &str = "Method Not Allowed";
pub const M_NOT_ACCEPTABLE: &str = "Not Acceptable";
pub const M_PROXY_AUTHENTICATION_REQUIRED: &str = "Proxy Authentication Required";
pub const M_REQUEST_TIME_OUT: &str = "Request Time-out";
pub const M_CONFLICT: &str = " Conflict";
pub const M_GONE: &str = " Gone";
pub const M_LENGTH_REQUIRED: &str = " Length Required";
pub const M_PRECONDITION_FAILED: &str = " Precondition Failed";
pub const M_REQUEST_ENTITY_TOO_LARGE: &str = " Request Entity Too Large";
pub const M_REQUEST_URI_TOO_LARGE: &str = " Request-URI Too Large";
pub const M_UNSUPPORTED_MEDIA_TYPE: &str = " Unsupported Media Type";
pub const M_REQUESTED_RANGE_NOT_SATISFIABLE: &str = " Requested range not satisfiable";
pub const M_EXPECTATION_FAILED: &str = " Expectation Failed";
pub const M_INTERNAL_SERVER_ERROR: &str = "Internal Server Error";
pub const M_NOT_IMPLEMENTED: &str = "Not Implemented";
pub const M_BAD_GATEWAY: &str = "Bad Gateway";
pub const M_SERVICE_UNAVAILABLE: &str = "Service Unavailable";
pub const M_GATEWAY_TIME_OUT: &str = "Gateway Time-out";
pub const M_HTTP_VERSION_NOT_SUPPORTED: &str = "HTTP Version not supported";

/// Web Specific MIME (media type) file extensions
pub const MIME_EXT_AAC: &str = ".aac";
pub const MIME_EXT_ABW: &str = ".abw";
pub const MIME_EXT_APNG: &str = ".apng";
pub const MIME_EXT_ARC: &str = ".arc";
pub const MIME_EXT_AVIF: &str = ".avif";
pub const MIME_EXT_AVI: &str = ".avi";
pub const MIME_EXT_AZW: &str = ".azw";
pub const MIME_EXT_BIN: &str = ".bin";
pub const MIME_EXT_BMP: &str = ".bmp";
pub const MIME_EXT_BZ: &str = ".bz";
pub const MIME_EXT_BZ2: &str = ".bz2";
pub const MIME_EXT_CDA: &str = ".cda";
pub const MIME_EXT_CSH: &str = ".csh";
pub const MIME_EXT_CSS: &str = ".css";
pub const MIME_EXT_CSV: &str = ".csv";
pub const MIME_EXT_DOC: &str = ".doc";
pub const MIME_EXT_DOCX: &str = ".docx";
pub const MIME_EXT_EOT: &str = ".eot";
pub const MIME_EXT_EPUB: &str = ".epub";
pub const MIME_EXT_GZ: &str = ".gz";
pub const MIME_EXT_GIF: &str = ".gif";
pub const MIME_EXT_HTM: &str = ".htm";
pub const MIME_EXT_HTML: &str = ".html";
pub const MIME_EXT_ICO: &str = ".ico";
pub const MIME_EXT_ICS: &str = ".ics";
pub const MIME_EXT_JAR: &str = ".jar";
pub const MIME_EXT_JPEG: &str = ".jpeg";
pub const MIME_EXT_JPG: &str = ".jpg";
pub const MIME_EXT_JS: &str = ".js";
pub const MIME_EXT_JSON: &str = ".json";
pub const MIME_EXT_JSONLD: &str = ".jsonld";
pub const MIME_EXT_MID: &str = ".mid";
pub const MIME_EXT_MIDI: &str = ".midi";
pub const MIME_EXT_MJS: &str = ".mjs";
pub const MIME_EXT_MP3: &str = ".mp3";
pub const MIME_EXT_MP4: &str = ".mp4";
pub const MIME_EXT_MPEG: &str = ".mpeg";
pub const MIME_EXT_MPKG: &str = ".mpkg";
pub const MIME_EXT_ODP: &str = ".odp";
pub const MIME_EXT_ODS: &str = ".ods";
pub const MIME_EXT_ODT: &str = ".odt";
pub const MIME_EXT_OGA: &str = ".oga";
pub const MIME_EXT_OGV: &str = ".ogv";
pub const MIME_EXT_OGX: &str = ".ogx";
pub const MIME_EXT_OPUS: &str = ".opus";
pub const MIME_EXT_OTF: &str = ".otf";
pub const MIME_EXT_PNG: &str = ".png";
pub const MIME_EXT_PDF: &str = ".pdf";
pub const MIME_EXT_PHP: &str = ".php";
pub const MIME_EXT_PPT: &str = ".ppt";
pub const MIME_EXT_PPTX: &str = ".pptx";
pub const MIME_EXT_RAR: &str = ".rar";
pub const MIME_EXT_RTF: &str = ".rtf";
pub const MIME_EXT_SH: &str = ".sh";
pub const MIME_EXT_SVG: &str = ".svg";
pub const MIME_EXT_TAR: &str = ".tar";
pub const MIME_EXT_TIF: &str = ".tif";
pub const MIME_EXT_TIFF: &str = ".tiff";
pub const MIME_EXT_TS: &str = ".ts";
pub const MIME_EXT_TTF: &str = ".ttf";
pub const MIME_EXT_TXT: &str = ".txt";
pub const MIME_EXT_VSD: &str = ".vsd";
pub const MIME_EXT_WAV: &str = ".wav";
pub const MIME_EXT_WEBA: &str = ".weba";
pub const MIME_EXT_WEBM: &str = ".webm";
pub const MIME_EXT_WEBP: &str = ".webp";
pub const MIME_EXT_WOFF: &str = ".woff";
pub const MIME_EXT_WOFF2: &str = ".woff2";
pub const MIME_EXT_XHTML: &str = ".xhtml";
pub const MIME_EXT_XLS: &str = ".xls";
pub const MIME_EXT_XLSX: &str = ".xlsx";
pub const MIME_EXT_XML: &str = ".xml";
pub const MIME_EXT_XUL: &str = ".xul";
pub const MIME_EXT_ZIP: &str = ".zip";
pub const MIME_EXT_3GP: &str = ".3gp";
pub const MIME_EXT_3G2: &str = ".3g2";
pub const MIME_EXT_7Z: &str = ".7z";

/// Web Specific MIME TYPEs (media types)
pub const MIME_TYPE_AAC: &str = "audio/aac";
pub const MIME_TYPE_ABW: &str = "application/x-abiword";
pub const MIME_TYPE_APNG: &str = "image/apng";
pub const MIME_TYPE_ARC: &str = "application/x-freearc";
pub const MIME_TYPE_AVIF: &str = "image/avif";
pub const MIME_TYPE_AVI: &str = "video/x-msvideo";
pub const MIME_TYPE_AZW: &str = "application/vnd.amazon.ebook";
pub const MIME_TYPE_BIN: &str = "application/octet-stream";
pub const MIME_TYPE_BMP: &str = "image/bmp";
pub const MIME_TYPE_BZ: &str = "application/x-bzip";
pub const MIME_TYPE_BZ2: &str = "application/x-bzip2";
pub const MIME_TYPE_CDA: &str = "application/x-cdf";
pub const MIME_TYPE_CSH: &str = "application/x-csh";
pub const MIME_TYPE_CSS: &str = "text/css";
pub const MIME_TYPE_CSV: &str = "text/csv";
pub const MIME_TYPE_DOC: &str = "application/msword";
pub const MIME_TYPE_DOCX: &str =
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document";
pub const MIME_TYPE_EOT: &str = "application/vnd.ms-fontobject";
pub const MIME_TYPE_EPUB: &str = "application/epub+zip";
pub const MIME_TYPE_GZ: &str = "application/gzip";
pub const MIME_TYPE_GZ_X: &str = "application/x-gzip";
pub const MIME_TYPE_GIF: &str = "image/gif";
pub const MIME_TYPE_HTML: &str = "text/html";
pub const MIME_TYPE_ICO: &str = "image/vnd.microsoft.icon";
pub const MIME_TYPE_ICS: &str = "text/calendar";
pub const MIME_TYPE_JAR: &str = "application/java-archive";
pub const MIME_TYPE_JPG: &str = "image/jpeg";
pub const MIME_TYPE_JS: &str = "text/javascript";
pub const MIME_TYPE_JSON: &str = "application/json";
pub const MIME_TYPE_JSONLD: &str = "application/ld+json";
pub const MIME_TYPE_MID: &str = "audio/midi";
pub const MIME_TYPE_MIDI: &str = "audio/x-midi";
pub const MIME_TYPE_MJS: &str = "text/javascript";
pub const MIME_TYPE_MP3: &str = "audio/mpeg";
pub const MIME_TYPE_MP4: &str = "video/mp4";
pub const MIME_TYPE_MPEG: &str = "video/mpeg";
pub const MIME_TYPE_MPKG: &str = "application/vnd.apple.installer+xml";
pub const MIME_TYPE_ODP: &str = "application/vnd.oasis.opendocument.presentation";
pub const MIME_TYPE_ODS: &str = "application/vnd.oasis.opendocument.spreadsheet";
pub const MIME_TYPE_ODT: &str = "application/vnd.oasis.opendocument.text";
pub const MIME_TYPE_OGA: &str = "audio/ogg";
pub const MIME_TYPE_OGV: &str = "video/ogg";
pub const MIME_TYPE_OGX: &str = "application/ogg";
pub const MIME_TYPE_OPUS: &str = "audio/ogg";
pub const MIME_TYPE_OTF: &str = "font/otf";
pub const MIME_TYPE_PNG: &str = "image/png";
pub const MIME_TYPE_PDF: &str = "application/pdf";
pub const MIME_TYPE_PHP: &str = "application/x-httpd-php";
pub const MIME_TYPE_PPT: &str = "application/vnd.ms-powerpoint";
pub const MIME_TYPE_PPTX: &str =
    "application/vnd.openxmlformats-officedocument.presentationml.presentation";
pub const MIME_TYPE_RAR: &str = "application/vnd.rar";
pub const MIME_TYPE_RTF: &str = "application/rtf";
pub const MIME_TYPE_SH: &str = "application/x-sh";
pub const MIME_TYPE_SVG: &str = "image/svg+xml";
pub const MIME_TYPE_TAR: &str = "application/x-tar";
pub const MIME_TYPE_TIFF: &str = "image/tiff";
pub const MIME_TYPE_TS: &str = "video/mp2t";
pub const MIME_TYPE_TTF: &str = "font/ttf";
pub const MIME_TYPE_TXT: &str = "text/plain";
pub const MIME_TYPE_VSD: &str = "application/vnd.visio";
pub const MIME_TYPE_WAV: &str = "audio/wav";
pub const MIME_TYPE_WEBA: &str = "audio/webm";
pub const MIME_TYPE_WEBM: &str = "video/webm";
pub const MIME_TYPE_WEBP: &str = "image/webp";
pub const MIME_TYPE_WOFF: &str = "font/woff";
pub const MIME_TYPE_WOFF2: &str = "font/woff2";
pub const MIME_TYPE_XHTML: &str = "application/xhtml+xml";
pub const MIME_TYPE_XLS: &str = "application/vnd.ms-excel";
pub const MIME_TYPE_XLSX: &str =
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet";
pub const MIME_TYPE_XML: &str = "application/xml";
pub const MIME_TYPE_XMLTEX: &str = "text/xml ";
pub const MIME_TYPE_XUL: &str = "application/vnd.mozilla.xul+xml";
pub const MIME_TYPE_ZIP: &str = "application/zip";
pub const MIME_TYPE_ZIP_X: &str = "application/x-zip-compressed";
pub const MIME_TYPE_3GP: &str = "video/3gpp";
pub const MIME_TYPE_3GP_AUDIO: &str = "audio/3gpp";
pub const MIME_TYPE_3G2: &str = "video/3gpp2";
pub const MIME_TYPE_3G2_AUDIO: &str = "audio/3gpp2";
pub const MIME_TYPE_7Z: &str = "application/x-7z-compressed";

pub const MINE_MULTIPART_FORM: &str = "multipart/form-data";
pub const MINE_URLENCODED_FORM: &str = "application/x-www-form-urlencoded";
