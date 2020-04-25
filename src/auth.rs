use crate::in_memory_db::{Database, NewSession, Session, State};
use cookie::{Cookie, SameSite};
use http_types::{headers::CONTENT_TYPE, StatusCode};
use mime::TEXT_HTML_UTF_8;
use std::time::{SystemTime, UNIX_EPOCH};
use tide::Request;

#[derive(Debug, serde::Deserialize)]
struct LoginForm {
    username: Option<String>,
    password: Option<String>,
}

fn new_cookie_session(req: &Request<State>, username: String) -> Cookie<'static> {
    use time::strptime;
    // Session ID: first part(epoch+time -> uuidv1) + second part(random uuidv4)

    // Get current epoch time to generate uuidv1
    let start = SystemTime::now();
    let full_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let full_epoch_string = format!("{:?}", full_epoch).replace("s", "");
    let full_epoch_vec: Vec<&str> = full_epoch_string.split('.').collect();

    let epoch_secs = full_epoch_vec[0].parse::<u64>().unwrap();
    let epoch_milis = full_epoch_vec[1].parse::<u32>().unwrap();

    // Generate uuid v1
    let context = uuid::v1::Context::new(42);
    let ts = uuid::v1::Timestamp::from_unix(&context, epoch_secs, epoch_milis);
    let uuidv1 = uuid::Uuid::new_v1(ts, &[1, 2, 3, 4, 5, 6]).expect("failed to generate UUID");

    // Generate uuid v4
    let uuidv4 = uuid::Uuid::new_v4();

    // Merge uuiv1 + uuv4 to create a good enough Session ID (Maybe include some aditional information like IP address and/or source port)
    let session_token = format!("{}-{}", uuidv1, uuidv4);

    // Calculate expiration time of sessions
    let now = epoch_secs;
    // let time_to_expire = now + 40; // 40 seconds
    let time_to_expire = now + 172_800; // 48 hours in seconds

    // Save Cookie into backend
    let new_user = NewSession {
        session_token: session_token.to_string(),
        expires_at: now + 172_800, // 48 hours in seconds
        username,
    };

    Database::add_session(req.state(), new_user);
    let mut cookie_session = Cookie::new("session_id", session_token);
    cookie_session.set_http_only(true);
    cookie_session.set_same_site(SameSite::Strict);
    // TODO: Maybe set Cookie with `Secure` parameter (in the future commits)

    let time_to_expire = format!("{:?}", time_to_expire);
    dbg!(&time_to_expire);
    let time_to_expire = strptime(&time_to_expire, "%s").unwrap();
    dbg!(&time_to_expire);
    cookie_session.set_expires(time_to_expire);
    cookie_session
}

fn get_cookie_from_request(req: &Request<State>, cookie_name: &str) -> Option<String> {
    let cookie_str = match req.cookie(cookie_name) {
        Some(cookie) => cookie.to_string(),
        None => return None,
    };
    let cookie_vec: Vec<&str> = cookie_str.split('=').collect();
    // let cookie_key = cookie_vec[0];
    let cookie_value = cookie_vec[1];
    Some(cookie_value.to_string())
}

pub fn is_authenticated(req: Request<State>) -> Result<Session, ()> {
    dbg!(&req);
    let cookie = match get_cookie_from_request(&req, "session_id") {
        Some(cookie_value) => cookie_value,
        None => {
            return Err(());
        }
    };
    let start = SystemTime::now();
    let full_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let full_epoch_string = format!("{:?}", full_epoch).replace("s", "");
    let full_epoch_vec: Vec<&str> = full_epoch_string.split('.').collect();

    let now_epoch_secs = full_epoch_vec[0].parse::<u64>().unwrap();
    dbg!(&now_epoch_secs);
    // * Implements expiration time at backend by invalidate a session
    match Database::list_sessions(req.state()).iter().find(|session| {
        session.get_session_token() == cookie && session.get_expires_at() > now_epoch_secs
    }) {
        Some(current_session) => Ok(current_session.clone()),
        None => Err(()),
    }
}

pub async fn show_login(req: Request<State>) -> tide::Result {
    dbg!(&req);
    let html = include_str!("../static_html/login.html").to_string();
    Ok(tide::Response::new(StatusCode::Ok)
        .body_string(html)
        .set_header(CONTENT_TYPE, TEXT_HTML_UTF_8))
}

fn is_valid_auth(username: String, password: String) -> bool {
    // TODO: Some data driven authetication. Ex: ENV, Toml, SQL
    use std::env;
    if env::var("POD_INSPECTOR_USERNAME").is_ok() && env::var("POD_INSPECTOR_PASSWORD").is_ok() {
        username == env::var("POD_INSPECTOR_USERNAME").unwrap()
            && password == env::var("POD_INSPECTOR_PASSWORD").unwrap()
    } else {
        username == "admin" && password == "9admin9"
            || username == "root" && password == "root"
            || username == "user" && password == "qwASzx"
    }
}

pub async fn submit_login(mut req: Request<State>) -> tide::Result {
    dbg!(&req);
    let query: LoginForm = req
        .body_form()
        .await
        .expect("unable to deserialize LoginForm request");
    let mut username = String::new();
    let mut password = String::new();
    if query.username.is_some() && query.password.is_some() {
        if let Some(user) = query.username {
            username = user;
        }
        if let Some(pass) = query.password {
            password = pass;
        }
        if is_valid_auth(username.clone(), password) {
            let cookie = new_cookie_session(&req, username);
            // * If login ok, create response with a redirect to /
            let html = r#"<head>
            <meta http-equiv="refresh" content="0; url=/">
            </head>"#
                .to_string();
            let mut res = tide::Response::new(StatusCode::TemporaryRedirect)
                .body_string(html)
                .set_header(CONTENT_TYPE, TEXT_HTML_UTF_8);
            res.set_cookie(cookie);
            Ok(res)
        } else {
            println!("Username and/or Passsword not found");
            let html = r#"<head>
            <meta http-equiv="refresh" content="0; url=/404">
            </head>"#
                .to_string();
            Ok(tide::Response::new(StatusCode::TemporaryRedirect)
                .body_string(html)
                .set_header(CONTENT_TYPE, TEXT_HTML_UTF_8))
        }
    } else {
        println!("Invalid Form");
        let html = r#"<head>
        <meta http-equiv="refresh" content="0; url=/404">
        </head>"#
            .to_string();
        Ok(tide::Response::new(StatusCode::TemporaryRedirect)
            .body_string(html)
            .set_header(CONTENT_TYPE, TEXT_HTML_UTF_8))
    }
}

pub async fn show_404(req: Request<State>) -> tide::Result {
    dbg!(&req);
    let html = include_str!("../static_html/404.html").to_string();
    Ok(tide::Response::new(StatusCode::NotFound)
        .body_string(html)
        .set_header(CONTENT_TYPE, TEXT_HTML_UTF_8))
}

pub async fn logout(req: Request<State>) -> tide::Result {
    dbg!(&req);
    // * Check get session_token cookie
    let cookie = match get_cookie_from_request(&req, "session_id") {
        Some(cookie_value) => cookie_value,
        None => String::from(""),
    };
    // * Remove session from mutable state
    Database::destroy_session(req.state(), cookie);
    let html = r#"<head>
    <meta http-equiv="refresh" content="0; url=/">
    </head>"#
        .to_string();
    let mut res = tide::Response::new(StatusCode::TemporaryRedirect)
        .body_string(html)
        .set_header(CONTENT_TYPE, TEXT_HTML_UTF_8);
        res.remove_cookie(Cookie::named("session_id"));
        Ok(res)
}
