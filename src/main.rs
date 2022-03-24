mod flights_start_end;
mod req_response;

use warp::Filter;

#[tokio::main]
async fn main() {
    let routes = warp::post().and(warp::body::json()).map(
        |start_and_end_req: req_response::StartAndDestinationRequest| {
            // println!("{:#?}", start_and_end_req);

            let pairs = start_and_end_req
                .stop_list
                .iter()
                .map(|pair| (pair.0.as_str(), pair.1.as_str()))
                .collect::<Vec<_>>();

            let resp = match flights_start_end::construct_start_end(&pairs) {
                Some(pair) => req_response::StartAndDestinationResponse::Success {
                    start_and_end: pair,
                },
                None => req_response::StartAndDestinationResponse::Failure {
                    reason: "Indecisive to get the travel data.".into(),
                },
            };

            warp::reply::json(&resp)
        },
    );

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
