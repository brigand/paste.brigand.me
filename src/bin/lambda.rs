use lambda_http::{lambda, Body, Request, Response};
use lambda_runtime::{error::HandlerError, Context};

fn main() {
    lambda!(router)
}

type Res = Result<Response<Body>, Response<Body>>;

fn not_found() -> Res {
    Err(Response::builder()
        .status(http::StatusCode::NOT_FOUND)
        .body(Body::Text("404 Not Found".into()))
        .unwrap())
}

fn index_route(_request: &Request) -> Res {
    not_found()?;

    Ok(Response::builder()
        .body(Body::Text("Hello, world!".into()))
        .unwrap())
}

fn router(request: Request, _ctx: Context) -> Result<Response<Body>, HandlerError> {
    match index_route(&request) {
        Ok(res) => Ok(res),
        Err(res) => Ok(res),
    }
}
