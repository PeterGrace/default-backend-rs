use rocket::request::{FromRequest, Outcome, Request};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct AllHeaders(HashMap<String, String>);
impl AllHeaders {
    pub(crate) fn get(&self, key: String) -> Option<&String> {
        self.0.get(key.as_str())
    }
}
impl<'a, 'r> FromRequest<'a, 'r> for AllHeaders {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> Outcome<AllHeaders, ()> {
        let mut hash = HashMap::new();
        for header in request.headers().iter() {
            hash.insert(header.name().to_string(), header.value().to_string());
        }
        return Outcome::Success(AllHeaders(hash));
    }
}
