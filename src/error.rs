use serde::Serialize;
use serde_json::{json, Value};
use warp::{hyper::StatusCode, reject::Reject};

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
    data: Value,
}

#[derive(Debug)]
pub struct ConversionError;
impl Reject for ConversionError {}

pub async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    let code;
    let message;

    if err.find::<ConversionError>().is_some()
        || err.find::<warp::reject::InvalidQuery>().is_some()
        || err.find::<warp::reject::InvalidHeader>().is_some()
    {
        code = StatusCode::BAD_REQUEST;
        message = "Bad Request";
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "Method Not Allowed";
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message: message.into(),
        data: json!(null),
    });

    Ok(warp::reply::with_status(json, code))
}
