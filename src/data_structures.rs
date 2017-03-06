#[derive(RustcDecodable, RustcEncodable)]
pub struct EventSeverRequest<T> {
    pub method: String,
    pub parameters: T,
}

#[derive(RustcDecodable, RustcEncodable)]
#[allow(non_snake_case)]
pub struct GetChunkOfEventsRequest {
    pub afterSequenceNumber: i64,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct EventServerResult<T> {
    pub result: T,
}

#[derive(RustcDecodable, RustcEncodable)]
#[allow(non_snake_case)]
pub struct Event {
    pub sequenceNumber: i64,
    pub eventType: String,
    pub time: String,
    pub body: String,
}

#[derive(RustcDecodable, RustcEncodable)]
#[allow(non_snake_case)]
pub struct SequenceNumberContainer {
    pub sequenceNumber: i64,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct MessageEventBody {
    pub message: String,
}

#[derive(RustcDecodable, RustcEncodable)]
#[allow(non_snake_case)]
pub struct HueRelayEventBody {
    pub url: String,
    pub content: String,
}
