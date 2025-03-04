use actix_web::dev::Payload;
#[cfg(not(feature = "nouid"))]
use actix_web::error;
use actix_web::Error;
use actix_web::FromRequest;
use actix_web::HttpRequest;

#[derive(Deserialize, Debug, Clone)]
pub struct UserId {
    pub user_id: String,
}

impl FromRequest for UserId {
    type Config = ();
    type Future = Result<Self, Error>;
    type Error = Error;

    #[cfg(not(feature = "nouid"))]
    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let user_id = req
            .headers()
            .get("x-forwarded-user-subject")
            .or_else(|| req.headers().get("x-auth-subject"));
        user_id
            .and_then(|id| id.to_str().ok())
            .map(|id| UserId {
                user_id: id.to_owned(),
            })
            .ok_or_else(|| error::ErrorForbidden("no user_id"))
    }

    #[cfg(feature = "nouid")]
    #[inline]
    fn from_request(_: &HttpRequest, _: &mut Payload) -> Self::Future {
        use std::env::var;
        let user_id = var("DPW_USER_ID").unwrap();
        Ok(UserId { user_id })
    }
}
