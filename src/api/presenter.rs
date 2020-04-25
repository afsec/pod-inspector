use crate::api::model;
use crate::api::view;
use crate::in_memory_db::State;

use tide::Request;

fn fib(n: usize) -> usize {
    if n == 0 || n == 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

pub async fn fibsum(req: Request<State>) -> tide::Response {
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
            tide::Response::new(200)
                .body_string(html)
                .set_header("Content-Type", "text/html; charset=utf-8")
        }
        Err(_) => {
            println!("Request not authenticated");
            tide::Response::new(307).set_header("Location", "/login")
        }
    }
}

pub async fn printenv(req: Request<State>) -> tide::Response {
    let command = "echo '$ env' ; env";
    match crate::auth::is_authenticated(req) {
        Ok(session) => match model::run_command(command) {
            Ok(stdout) => {
                let html = view::render_page(session, Some(stdout)).await;
                tide::Response::new(200)
                    .body_string(html)
                    .set_header("Content-Type", "text/html; charset=utf-8")
            }
            Err(stderr) => {
                let html = view::render_page(session, Some(stderr)).await;
                tide::Response::new(200)
                    .body_string(html)
                    .set_header("Content-Type", "text/html; charset=utf-8")
            }
        },
        Err(_) => {
            println!("Request not authenticated");
            tide::Response::new(307).set_header("Location", "/login")
        }
    }
}

pub async fn show_memory(req: Request<State>) -> tide::Response {
    let command = "echo '$ free -m' ; free -m";
    match crate::auth::is_authenticated(req) {
        Ok(session) => match model::run_command(command) {
            Ok(stdout) => {
                let html = view::render_page(session, Some(stdout)).await;
                tide::Response::new(200)
                    .body_string(html)
                    .set_header("Content-Type", "text/html; charset=utf-8")
            }
            Err(stderr) => {
                let html = view::render_page(session, Some(stderr)).await;
                tide::Response::new(200)
                    .body_string(html)
                    .set_header("Content-Type", "text/html; charset=utf-8")
            }
        },
        Err(_) => {
            println!("Request not authenticated");
            tide::Response::new(307).set_header("Location", "/login")
        }
    }
}

pub async fn cpuinfo(req: Request<State>) -> tide::Response {
    let command = "echo '$ cat /proc/cpuinfo' ; cat /proc/cpuinfo";
    match crate::auth::is_authenticated(req) {
        Ok(session) => match model::run_command(command) {
            Ok(stdout) => {
                let html = view::render_page(session, Some(stdout)).await;
                tide::Response::new(200)
                    .body_string(html)
                    .set_header("Content-Type", "text/html; charset=utf-8")
            }
            Err(stderr) => {
                let html = view::render_page(session, Some(stderr)).await;
                tide::Response::new(200)
                    .body_string(html)
                    .set_header("Content-Type", "text/html; charset=utf-8")
            }
        },
        Err(_) => {
            println!("Request not authenticated");
            tide::Response::new(307).set_header("Location", "/login")
        }
    }
}

pub async fn show_disk(req: Request<State>) -> tide::Response {
    let command = "echo '$ df -h' ; df -h";
    match crate::auth::is_authenticated(req) {
        Ok(session) => match model::run_command(command) {
            Ok(stdout) => {
                let html = view::render_page(session, Some(stdout)).await;
                tide::Response::new(200)
                    .body_string(html)
                    .set_header("Content-Type", "text/html; charset=utf-8")
            }
            Err(stderr) => {
                let html = view::render_page(session, Some(stderr)).await;
                tide::Response::new(200)
                    .body_string(html)
                    .set_header("Content-Type", "text/html; charset=utf-8")
            }
        },
        Err(_) => {
            println!("Request not authenticated");
            tide::Response::new(307).set_header("Location", "/login")
        }
    }
}

pub async fn show_processes(req: Request<State>) -> tide::Response {
    let command = "echo '$ ps -ef' ; ps -ef";
    match crate::auth::is_authenticated(req) {
        Ok(session) => match model::run_command(command) {
            Ok(stdout) => {
                let html = view::render_page(session, Some(stdout)).await;
                tide::Response::new(200)
                    .body_string(html)
                    .set_header("Content-Type", "text/html; charset=utf-8")
            }
            Err(stderr) => {
                let html = view::render_page(session, Some(stderr)).await;
                tide::Response::new(200)
                    .body_string(html)
                    .set_header("Content-Type", "text/html; charset=utf-8")
            }
        },
        Err(_) => {
            println!("Request not authenticated");
            tide::Response::new(307).set_header("Location", "/login")
        }
    }
}

pub async fn show_root(req: Request<State>) -> tide::Response {
    let command = "echo '$ ls -lh /' ; ls -lh /";
    match crate::auth::is_authenticated(req) {
        Ok(session) => match model::run_command(command) {
            Ok(stdout) => {
                let html = view::render_page(session, Some(stdout)).await;
                tide::Response::new(200)
                    .body_string(html)
                    .set_header("Content-Type", "text/html; charset=utf-8")
            }
            Err(stderr) => {
                let html = view::render_page(session, Some(stderr)).await;
                tide::Response::new(200)
                    .body_string(html)
                    .set_header("Content-Type", "text/html; charset=utf-8")
            }
        },
        Err(_) => {
            println!("Request not authenticated");
            tide::Response::new(307).set_header("Location", "/login")
        }
    }
}

pub async fn show_pwd(req: Request<State>) -> tide::Response {
    let command = "echo '$ pwd' ; pwd ; echo '$ ls -lh' ; ls -lh";
    match crate::auth::is_authenticated(req) {
        Ok(session) => match model::run_command(command) {
            Ok(stdout) => {
                let html = view::render_page(session, Some(stdout)).await;
                tide::Response::new(200)
                    .body_string(html)
                    .set_header("Content-Type", "text/html; charset=utf-8")
            }
            Err(stderr) => {
                let html = view::render_page(session, Some(stderr)).await;
                tide::Response::new(200)
                    .body_string(html)
                    .set_header("Content-Type", "text/html; charset=utf-8")
            }
        },
        Err(_) => {
            println!("Request not authenticated");
            tide::Response::new(307).set_header("Location", "/login")
        }
    }
}

pub async fn show_uptime(req: Request<State>) -> tide::Response {
    let command = "echo '$ uptime' ; uptime";
    match crate::auth::is_authenticated(req) {
        Ok(session) => match model::run_command(command) {
            Ok(stdout) => {
                let html = view::render_page(session, Some(stdout)).await;
                tide::Response::new(200)
                    .body_string(html)
                    .set_header("Content-Type", "text/html; charset=utf-8")
            }
            Err(stderr) => {
                let html = view::render_page(session, Some(stderr)).await;
                tide::Response::new(200)
                    .body_string(html)
                    .set_header("Content-Type", "text/html; charset=utf-8")
            }
        },
        Err(_) => {
            println!("Request not authenticated");
            tide::Response::new(307).set_header("Location", "/login")
        }
    }
}

pub async fn show_lsof(req: Request<State>) -> tide::Response {
    let command = "echo '$ lsof' ; lsof";
    match crate::auth::is_authenticated(req) {
        Ok(session) => match model::run_command(command) {
            Ok(stdout) => {
                let html = view::render_page(session, Some(stdout)).await;
                tide::Response::new(200)
                    .body_string(html)
                    .set_header("Content-Type", "text/html; charset=utf-8")
            }
            Err(stderr) => {
                let html = view::render_page(session, Some(stderr)).await;
                tide::Response::new(200)
                    .body_string(html)
                    .set_header("Content-Type", "text/html; charset=utf-8")
            }
        },
        Err(_) => {
            println!("Request not authenticated");
            tide::Response::new(307).set_header("Location", "/login")
        }
    }
}

pub async fn show_interfaces(req: Request<State>) -> tide::Response {
    let command = "echo '$ ip link' ; ip link ;echo '$ ip addr' ;  ip addr";
    match crate::auth::is_authenticated(req) {
        Ok(session) => match model::run_command(command) {
            Ok(stdout) => {
                let html = view::render_page(session, Some(stdout)).await;
                tide::Response::new(200)
                    .body_string(html)
                    .set_header("Content-Type", "text/html; charset=utf-8")
            }
            Err(stderr) => {
                let html = view::render_page(session, Some(stderr)).await;
                tide::Response::new(200)
                    .body_string(html)
                    .set_header("Content-Type", "text/html; charset=utf-8")
            }
        },
        Err(_) => {
            println!("Request not authenticated");
            tide::Response::new(307).set_header("Location", "/login")
        }
    }
}

pub async fn show_netstat(req: Request<State>) -> tide::Response {
    let command = "echo '$ netstat -anp' ; netstat -anp";
    match crate::auth::is_authenticated(req) {
        Ok(session) => match model::run_command(command) {
            Ok(stdout) => {
                let html = view::render_page(session, Some(stdout)).await;
                tide::Response::new(200)
                    .body_string(html)
                    .set_header("Content-Type", "text/html; charset=utf-8")
            }
            Err(stderr) => {
                let html = view::render_page(session, Some(stderr)).await;
                tide::Response::new(200)
                    .body_string(html)
                    .set_header("Content-Type", "text/html; charset=utf-8")
            }
        },
        Err(_) => {
            println!("Request not authenticated");
            tide::Response::new(307).set_header("Location", "/login")
        }
    }
}

pub async fn index_page(req: Request<State>) -> tide::Response {
    match crate::auth::is_authenticated(req) {
        Ok(session) => {
            let html = view::render_page(session, None).await;
            tide::Response::new(200)
                .body_string(html)
                .set_header("Content-Type", "text/html; charset=utf-8")
        }
        Err(_) => {
            println!("Request not authenticated");
            tide::Response::new(307).set_header("Location", "/login")
        }
    }
}
