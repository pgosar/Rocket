use sha1::Digest;
use base64::engine::general_purpose;
use base64::Engine;

const WEBSOCKET_PREFIX: &str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

pub fn sec_websocket_key(client_key: String) -> String {
    let combined = client_key + WEBSOCKET_PREFIX;
    let mut sha1 = sha1::Sha1::new();
    sha1.update(combined);
    let hash = sha1.finalize();
    let my_key: String = general_purpose::STANDARD.encode(&hash[..]);
    my_key
}