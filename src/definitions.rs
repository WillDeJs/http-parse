

/// Header names
pub const H_ACCEPT: &str = "Accept";
pub const H_ACCEPT_CHARSET: &str = "Accept-Charset";
pub const H_ACCEPT_LANGAUGE: &str = "Accept-Language";
pub const H_ACCEPT_RANGES: &str = "Accept-Ranges";
pub const H_AGE : &str = "Age";
pub const H_ALLOW : &str = "Allow";
pub const H_AUTHORIZATON : &str = "Authorization";
pub const H_CACHE_CONTROL : &str = "Cache-Control";
pub const H_CONNECTION : &str = "Connection";

pub const H_CONTENT_ENCODING : &str = "Content-Encoding";
pub const H_CONTENT_LANGUAGE : &str = "Content-Language";
pub const H_CONTENT_LENGTH : &str = "Content-Length";
pub const H_CONTENT_LOCATION : &str = "Content-Location";
pub const H_CONTENT_MD5 : &str = "Content-MD5";
pub const H_CONTENT_RANGE : &str = "Content-Range";
pub const H_CONTENT_TYPE : &str = "Content-Type";

pub const H_DATE : &str = "Date";

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

pub const H_TRAILER : &str = "Trailer";
pub const H_TRANSFER_ENCODING : &str = "Transfer-Encoding";
pub const H_UPGRADE : &str = "Upgrade";
pub const H_USER_AGENT : &str = "User-Agent";
pub const H_VARY : &str = "Vary";
pub const H_VIA : &str = "Via";
pub const H_WARNING : &str = "Warning";
pub const H_WWW_AUTHENTICATE : &str = "WWW-Authenticate";


/// STATUS CODES
const  S_CONTINUE : usize = 100;                        //  ; Section 10.1.1: Continue
const  S_SWITCHING_PROTOCOLS : usize =101;              //  ; Section 10.1.2: Switching Protocols
const  S_OK : usize =200;                               //  ; Section 10.2.1: OK
const  S_CREATED : usize =201;                          //  ; Section 10.2.2: Created
const  S_ACCEPTED : usize = 202;                        //  ; Section 10.2.3: Accepted
const  S_NON_AUTHORITATIVE : usize = 203;               //  ; Section 10.2.4: Non-Authoritative Information
const  S_NO_CONTENT : usize = 204;                      //  ; Section 10.2.5: No Content
const  S_RESET_CONTENT : usize = 205;                   //  ; Section 10.2.6: Reset Content
const  S_PARTIAL_CONTENT : usize = 206;                 //  ; Section 10.2.7: Partial Content
const  S_MULTIPLE_CHOICES : usize = 300;                //  ; Section 10.3.1: Multiple Choices
const  S_MOVED_PERMANENTLY : usize = 301;               //  ; Section 10.3.2: Moved Permanently
const  S_FOUND : usize = 302;                           //  ; Section 10.3.3: Found
const  S_SEE_OTHER : usize = 303;                       //  ; Section 10.3.4: See Other
const  S_NOT_MODIFIED : usize = 304;                    //  ; Section 10.3.5: Not Modified
const  S_USE_PROXY : usize = 305;                       //  ; Section 10.3.6: Use Proxy
const  S_TEMPORARY_REDIRECT : usize = 307;              //  ; Section 10.3.8: Temporary Redirect
const  S_BAD_REQUEST : usize = 400;                     //  ; Section 10.4.1: Bad Request
const  S_UNAUTHORIZED : usize = 401;                    //  ; Section 10.4.2: Unauthorized
const  S_PAYMENT_REQUIRED : usize = 402;                //  ; Section 10.4.3: Payment Required
const  S_FORBIDDEN : usize = 403;                       //  ; Section 10.4.4: Forbidden
const  S_NOT_FOUND : usize = 404;                       //  ; Section 10.4.5: Not Found
const  S_METHOD_NOT_ALLOWED : usize = 405;              //  ; Section 10.4.6: Method Not Allowed
const  S_NOT_ACCEPTABLE : usize = 406;                  //  ; Section 10.4.7: Not Acceptable
const  S_PROXY_AUTHENTICATION_REQUIRED : usize = 407;   //  ; Section 10.4.8: Proxy Authentication Required
const  S_REQUEST_TIME_OUT : usize = 408;                //  ; Section 10.4.9: Request Time-out
const  S_CONFLICT : usize = 409;                        //  ; Section 10.4.10: Conflict
const  S_GONE: usize = 410;                             //  ; Section 10.4.11: Gone
const  S_LENGTH_REQUIRED : usize = 411;                 //  ; Section 10.4.12: Length Required
const  S_PRECONDITION_FAILED : usize = 412;             //  ; Section 10.4.13: Precondition Failed
const  S_REQUEST_ENTITY_TOO_LARGE : usize = 413;        //  ; Section 10.4.14: Request Entity Too Large
const  S_REQUEST_URI_TOO_LARGE : usize = 414;           //  ; Section 10.4.15: Request-URI Too Large
const  S_UNSUPPORTED_MEDIA_TYPE : usize = 415;          //  ; Section 10.4.16: Unsupported Media Type
const  S_REQUESTED_RANGE_NOT_SATISFIABLE : usize = 416; //  ; Section 10.4.17: Requested range not satisfiable
const  S_EXPECTATION_FAILED : usize = 417;              //  ; Section 10.4.18: Expectation Failed
const  S_INTERNAL_SERVER_ERROR : usize = 500;           //  ; Section 10.5.1: Internal Server Error
const  S_NOT_IMPLEMENTED : usize = 501;                 //  ; Section 10.5.2: Not Implemented
const  S_BAD_GATEWAY : usize = 502;                     //  ; Section 10.5.3: Bad Gateway
const  S_SERVICE_UNAVAILABLE : usize = 503;             //  ; Section 10.5.4: Service Unavailable
const  S_GATEWAY_TIME_OUT : usize = 504;                //  ; Section 10.5.5: Gateway Time-out
const  S_HTTP_VERSION_NOT_SUPPORTED : usize = 505;      //  ; Section 10.5.6: HTTP Version not supported


/// STATUS MESSAGES
const  M_CONTINUE : &str = "Continue";              
const  M_SWITCHING_PROTOCOLS : &str = "Switching Protocols";                
const  M_OK : &str = "OK";              
const  M_CREATED : &str = "Created";                
const  M_ACCEPTED : &str = "Accepted";              
const  M_NON_AUTHORITATIVE : &str = "Non-Authoritative Information";                
const  M_NO_CONTENT : &str = "No Content";              
const  M_RESET_CONTENT : &str = "Reset Content";                
const  M_PARTIAL_CONTENT : &str = "Partial Content";                
const  M_MULTIPLE_CHOICES : &str = "Multiple Choices";              
const  M_MOVED_PERMANENTLY : &str = "Moved Permanently";                
const  M_FOUND : &str = "Found";                
const  M_SEE_OTHER : &str = "See Other";                
const  M_NOT_MODIFIED : &str = "Not Modified";              
const  M_USE_PROXY : &str = "Use Proxy";                
const  M_TEMPORARY_REDIRECT : &str = "Temporary Redirect";              
const  M_BAD_REQUEST : &str = "Bad Request";                
const  M_UNAUTHORIZED : &str = "Unauthorized";              
const  M_PAYMENT_REQUIRED : &str = "Payment Required";              
const  M_FORBIDDEN : &str = "Forbidden";                
const  M_NOT_FOUND : &str = "Not Found";                
const  M_METHOD_NOT_ALLOWED : &str = "Method Not Allowed";              
const  M_NOT_ACCEPTABLE : &str = "Not Acceptable";              
const  M_PROXY_AUTHENTICATION_REQUIRED : &str = "Proxy Authentication Required";                
const  M_REQUEST_TIME_OUT : &str = "Request Time-out";              
const  M_CONFLICT : &str = " Conflict";             
const  M_GONE : &str = " Gone";             
const  M_LENGTH_REQUIRED : &str = " Length Required";               
const  M_PRECONDITION_FAILED : &str = " Precondition Failed";               
const  M_REQUEST_ENTITY_TOO_LARGE : &str = " Request Entity Too Large";             
const  M_REQUEST_URI_TOO_LARGE : &str = " Request-URI Too Large";               
const  M_UNSUPPORTED_MEDIA_TYPE : &str = " Unsupported Media Type";             
const  M_REQUESTED_RANGE_NOT_SATISFIABLE : &str = " Requested range not satisfiable";               
const  M_EXPECTATION_FAILED : &str = " Expectation Failed";             
const  M_INTERNAL_SERVER_ERROR : &str = "Internal Server Error";                
const  M_NOT_IMPLEMENTED : &str = "Not Implemented";                
const  M_BAD_GATEWAY : &str = "Bad Gateway";                
const  M_SERVICE_UNAVAILABLE : &str = "Service Unavailable";                
const  M_GATEWAY_TIME_OUT : &str = "Gateway Time-out";              
const  M_HTTP_VERSION_NOT_SUPPORTED : &str = "HTTP Version not supported";              