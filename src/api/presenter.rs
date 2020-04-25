use crate::api::model;
use crate::api::view;
use crate::in_memory_db::State;

use tide::Request;

use http_types::{headers::CONTENT_TYPE, StatusCode};
use mime::TEXT_HTML_UTF_8;

fn fib(n: usize) -> usize {
    if n == 0 || n == 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

pub async fn fibsum(req: Request<State>) -> tide::Result {
    match crate::auth::is_authenticated(req) {
        Ok(session) => {
            use std::time::Instant;
            // let n: usize = req.param("n").unwrap_or(0);
            let n: usize = 42;
            // Start a stopwatch
            let start = Instant::now();
            // Compute the nth number in the fibonacci sequence
            let fib_n = fib(n);
            // Stop the stopwatch
            let duration = Instant::now().duration_since(start).as_secs();
            // Return the answer
            let command_output = format!(
                "The fib of {} is {}.\nIt was computed in {} secs.\n",
                n, fib_n, duration,
            );
            // render_page must receive current Session struct
            let html = view::render_page(session, Some(command_output)).await;
            Ok(tide::Response::new(StatusCode::Ok)
                .body_string(html)
                .set_header(CONTENT_TYPE, TEXT_HTML_UTF_8))
        }
        Err(_) => {
            println!("Request not authenticated");
            // Err(http_types::Error::from_str(StatusCode::BadRequest, "Bad Request"))
            Ok(tide::Response::new(StatusCode::TemporaryRedirect)
                .set_header("Location".parse().unwrap(), "/login"))
        }
    }
}

pub async fn printenv(req: Request<State>) -> tide::Result {
    match crate::auth::is_authenticated(req) {
        Ok(session) => match model::run_command("echo '$ env' ; env") {
            Ok(stdout) => {
                let html = view::render_page(session, Some(stdout)).await;
                Ok(tide::Response::new(StatusCode::Ok)
                    .body_string(html)
                    .set_header(CONTENT_TYPE, TEXT_HTML_UTF_8))
            }
            Err(stderr) => {
                let html = view::render_page(session, Some(stderr)).await;
                Ok(tide::Response::new(StatusCode::Ok)
                    .body_string(html)
                    .set_header(CONTENT_TYPE, TEXT_HTML_UTF_8))
            }
        },
        Err(_) => {
            println!("Request not authenticated");
            // Err(http_types::Error::from_str(StatusCode::BadRequest, "Bad Request"))
            Ok(tide::Response::new(StatusCode::TemporaryRedirect)
                .set_header("Location".parse().unwrap(), "/login"))
        }
    }
}

pub async fn show_memory(req: Request<State>) -> tide::Result {
    match crate::auth::is_authenticated(req) {
        Ok(session) => match model::run_command("echo '$ free -m' ; free -m") {
            Ok(stdout) => {
                let html = view::render_page(session, Some(stdout)).await;
                Ok(tide::Response::new(StatusCode::Ok)
                    .body_string(html)
                    .set_header(CONTENT_TYPE, TEXT_HTML_UTF_8))
            }
            Err(stderr) => {
                let html = view::render_page(session, Some(stderr)).await;
                Ok(tide::Response::new(StatusCode::Ok)
                    .body_string(html)
                    .set_header(CONTENT_TYPE, TEXT_HTML_UTF_8))
            }
        },
        Err(_) => {
            println!("Request not authenticated");
            // Err(http_types::Error::from_str(StatusCode::BadRequest, "Bad Request"))
            Ok(tide::Response::new(StatusCode::TemporaryRedirect)
                .set_header("Location".parse().unwrap(), "/login"))
        }
    }
}

pub async fn cpuinfo(req: Request<State>) -> tide::Result {
    match crate::auth::is_authenticated(req) {
        Ok(session) => match model::run_command("echo '$ cat /proc/cpuinfo' ; cat /proc/cpuinfo") {
            Ok(stdout) => {
                let html = view::render_page(session, Some(stdout)).await;
                Ok(tide::Response::new(StatusCode::Ok)
                    .body_string(html)
                    .set_header(CONTENT_TYPE, TEXT_HTML_UTF_8))
            }
            Err(stderr) => {
                let html = view::render_page(session, Some(stderr)).await;
                Ok(tide::Response::new(StatusCode::Ok)
                    .body_string(html)
                    .set_header(CONTENT_TYPE, TEXT_HTML_UTF_8))
            }
        },
        Err(_) => {
            println!("Request not authenticated");
            // Err(http_types::Error::from_str(StatusCode::BadRequest, "Bad Request"))
            Ok(tide::Response::new(StatusCode::TemporaryRedirect)
                .set_header("Location".parse().unwrap(), "/login"))
        }
    }
}

pub async fn show_disk(req: Request<State>) -> tide::Result {
    match crate::auth::is_authenticated(req) {
        Ok(session) => match model::run_command("echo '$ df -h' ; df -h") {
            Ok(stdout) => {
                let html = view::render_page(session, Some(stdout)).await;
                Ok(tide::Response::new(StatusCode::Ok)
                    .body_string(html)
                    .set_header(CONTENT_TYPE, TEXT_HTML_UTF_8))
            }
            Err(stderr) => {
                let html = view::render_page(session, Some(stderr)).await;
                Ok(tide::Response::new(StatusCode::Ok)
                    .body_string(html)
                    .set_header(CONTENT_TYPE, TEXT_HTML_UTF_8))
            }
        },
        Err(_) => {
            println!("Request not authenticated");
            // Err(http_types::Error::from_str(StatusCode::BadRequest, "Bad Request"))
            Ok(tide::Response::new(StatusCode::TemporaryRedirect)
                .set_header("Location".parse().unwrap(), "/login"))
        }
    }
}

pub async fn show_processes(req: Request<State>) -> tide::Result {
    match crate::auth::is_authenticated(req) {
        Ok(session) => match model::run_command("echo '$ ps -ef' ; ps -ef") {
            Ok(stdout) => {
                let html = view::render_page(session, Some(stdout)).await;
                Ok(tide::Response::new(StatusCode::Ok)
                    .body_string(html)
                    .set_header(CONTENT_TYPE, TEXT_HTML_UTF_8))
            }
            Err(stderr) => {
                let html = view::render_page(session, Some(stderr)).await;
                Ok(tide::Response::new(StatusCode::Ok)
                    .body_string(html)
                    .set_header(CONTENT_TYPE, TEXT_HTML_UTF_8))
            }
        },
        Err(_) => {
            println!("Request not authenticated");
            // Err(http_types::Error::from_str(StatusCode::BadRequest, "Bad Request"))
            Ok(tide::Response::new(StatusCode::TemporaryRedirect)
                .set_header("Location".parse().unwrap(), "/login"))
        }
    }
}

pub async fn show_root(req: Request<State>) -> tide::Result {
    match crate::auth::is_authenticated(req) {
        Ok(session) => match model::run_command("echo '$ ls -lh /' ; ls -lh /") {
            Ok(stdout) => {
                let html = view::render_page(session, Some(stdout)).await;
                Ok(tide::Response::new(StatusCode::Ok)
                    .body_string(html)
                    .set_header(CONTENT_TYPE, TEXT_HTML_UTF_8))
            }
            Err(stderr) => {
                let html = view::render_page(session, Some(stderr)).await;
                Ok(tide::Response::new(StatusCode::Ok)
                    .body_string(html)
                    .set_header(CONTENT_TYPE, TEXT_HTML_UTF_8))
            }
        },
        Err(_) => {
            println!("Request not authenticated");
            // Err(http_types::Error::from_str(StatusCode::BadRequest, "Bad Request"))
            Ok(tide::Response::new(StatusCode::TemporaryRedirect)
                .set_header("Location".parse().unwrap(), "/login"))
        }
    }
}

pub async fn show_pwd(req: Request<State>) -> tide::Result {
    match crate::auth::is_authenticated(req) {
        Ok(session) => match model::run_command("echo '$ pwd' ; pwd ; echo '$ ls -lh' ; ls -lh") {
            Ok(stdout) => {
                let html = view::render_page(session, Some(stdout)).await;
                Ok(tide::Response::new(StatusCode::Ok)
                    .body_string(html)
                    .set_header(CONTENT_TYPE, TEXT_HTML_UTF_8))
            }
            Err(stderr) => {
                let html = view::render_page(session, Some(stderr)).await;
                Ok(tide::Response::new(StatusCode::Ok)
                    .body_string(html)
                    .set_header(CONTENT_TYPE, TEXT_HTML_UTF_8))
            }
        },
        Err(_) => {
            println!("Request not authenticated");
            // Err(http_types::Error::from_str(StatusCode::BadRequest, "Bad Request"))
            Ok(tide::Response::new(StatusCode::TemporaryRedirect)
                .set_header("Location".parse().unwrap(), "/login"))
        }
    }
}

pub async fn show_uptime(req: Request<State>) -> tide::Result {
    match crate::auth::is_authenticated(req) {
        Ok(session) => match model::run_command("echo '$ uptime' ; uptime") {
            Ok(stdout) => {
                let html = view::render_page(session, Some(stdout)).await;
                Ok(tide::Response::new(StatusCode::Ok)
                    .body_string(html)
                    .set_header(CONTENT_TYPE, TEXT_HTML_UTF_8))
            }
            Err(stderr) => {
                let html = view::render_page(session, Some(stderr)).await;
                Ok(tide::Response::new(StatusCode::Ok)
                    .body_string(html)
                    .set_header(CONTENT_TYPE, TEXT_HTML_UTF_8))
            }
        },
        Err(_) => {
            println!("Request not authenticated");
            // Err(http_types::Error::from_str(StatusCode::BadRequest, "Bad Request"))
            Ok(tide::Response::new(StatusCode::TemporaryRedirect)
                .set_header("Location".parse().unwrap(), "/login"))
        }
    }
}

pub async fn show_lsof(req: Request<State>) -> tide::Result {
    match crate::auth::is_authenticated(req) {
        Ok(session) => match model::run_command("echo '$ lsof' ; lsof") {
            Ok(stdout) => {
                let html = view::render_page(session, Some(stdout)).await;
                Ok(tide::Response::new(StatusCode::Ok)
                    .body_string(html)
                    .set_header(CONTENT_TYPE, TEXT_HTML_UTF_8))
            }
            Err(stderr) => {
                let html = view::render_page(session, Some(stderr)).await;
                Ok(tide::Response::new(StatusCode::Ok)
                    .body_string(html)
                    .set_header(CONTENT_TYPE, TEXT_HTML_UTF_8))
            }
        },
        Err(_) => {
            println!("Request not authenticated");
            // Err(http_types::Error::from_str(StatusCode::BadRequest, "Bad Request"))
            Ok(tide::Response::new(StatusCode::TemporaryRedirect)
                .set_header("Location".parse().unwrap(), "/login"))
        }
    }
}

pub async fn show_interfaces(req: Request<State>) -> tide::Result {
    match crate::auth::is_authenticated(req) {
        Ok(session) => {
            match model::run_command("echo '$ ip link' ; ip link ;echo '$ ip addr' ;  ip addr") {
                Ok(stdout) => {
                    let html = view::render_page(session, Some(stdout)).await;
                    Ok(tide::Response::new(StatusCode::Ok)
                        .body_string(html)
                        .set_header(CONTENT_TYPE, TEXT_HTML_UTF_8))
                }
                Err(stderr) => {
                    let html = view::render_page(session, Some(stderr)).await;
                    Ok(tide::Response::new(StatusCode::Ok)
                        .body_string(html)
                        .set_header(CONTENT_TYPE, TEXT_HTML_UTF_8))
                }
            }
        }
        Err(_) => {
            println!("Request not authenticated");
            // Err(http_types::Error::from_str(StatusCode::BadRequest, "Bad Request"))
            Ok(tide::Response::new(StatusCode::TemporaryRedirect)
                .set_header("Location".parse().unwrap(), "/login"))
        }
    }
}

pub async fn show_netstat(req: Request<State>) -> tide::Result {
    match crate::auth::is_authenticated(req) {
        Ok(session) => match model::run_command("echo '$ netstat -anp' ; netstat -anp") {
            Ok(stdout) => {
                let html = view::render_page(session, Some(stdout)).await;
                Ok(tide::Response::new(StatusCode::Ok)
                    .body_string(html)
                    .set_header(CONTENT_TYPE, TEXT_HTML_UTF_8))
            }
            Err(stderr) => {
                let html = view::render_page(session, Some(stderr)).await;
                Ok(tide::Response::new(StatusCode::Ok)
                    .body_string(html)
                    .set_header(CONTENT_TYPE, TEXT_HTML_UTF_8))
            }
        },
        Err(_) => {
            println!("Request not authenticated");
            // Err(http_types::Error::from_str(StatusCode::BadRequest, "Bad Request"))
            Ok(tide::Response::new(StatusCode::TemporaryRedirect)
                .set_header("Location".parse().unwrap(), "/login"))
        }
    }
}

pub async fn index_page(req: Request<State>) -> tide::Result {
    match crate::auth::is_authenticated(req) {
        Ok(session) => {
            let html = view::render_page(session, None).await;
            Ok(tide::Response::new(StatusCode::Ok)
                .body_string(html)
                .set_header(CONTENT_TYPE, TEXT_HTML_UTF_8))
        }
        Err(_) => {
            println!("Request not authenticated");
            // Err(http_types::Error::from_str(StatusCode::BadRequest, "Bad Request"))
            Ok(tide::Response::new(StatusCode::TemporaryRedirect)
                .set_header("Location".parse().unwrap(), "/login"))
        }
    }
}
