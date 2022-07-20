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

    if let Some(_e) = err.find::<ConversionError>() {
        code = StatusCode::BAD_REQUEST;
        message = "Bad Request".to_string();
    } else {
        // We should have expected this... Just log and say its a 500
        eprintln!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION".to_string();
    }

    let json = warp::reply::json(&ErrorMessage {
        code: code.as_u16(),
        message,
        data: json!(null),
    });

    Ok(warp::reply::with_status(json, code))
}
