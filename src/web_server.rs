#![warn(clippy::all)]

use async_std::task;

pub fn run(bind: String) {
        use crate::in_memory_db::State;
        use tide::Server;
        task::block_on(async {
            let mut app = Server::with_state(State::new());
            app.at("/").get(crate::api::presenter::index_page);
            app.at("/login").get(crate::auth::show_login);
            app.at("/login").post(crate::auth::submit_login);
            app.at("/logout").get(crate::auth::logout);
            app.at("/uptime").get(crate::api::presenter::show_uptime);
            app.at("/processes").get(crate::api::presenter::show_processes);
            app.at("/interfaces").get(crate::api::presenter::show_interfaces);
            app.at("/netstat").get(crate::api::presenter::show_netstat);
            app.at("/disk").get(crate::api::presenter::show_disk);
            app.at("/printenv").get(crate::api::presenter::printenv);
            app.at("/memory").get(crate::api::presenter::show_memory);
            app.at("/disk").get(crate::api::presenter::show_disk);
            app.at("/cpuinfo").get(crate::api::presenter::cpuinfo);
            app.at("/show-lsof").get(crate::api::presenter::show_lsof);
            app.at("/show-root").get(crate::api::presenter::show_root);
            app.at("/show-pwd").get(crate::api::presenter::show_pwd);
            app.at("/cpuinfo").get(crate::api::presenter::cpuinfo);
            app.at("/404").get(crate::auth::show_404);
            app.at("/404").post(crate::auth::show_404);
            app.at("/fib").get(crate::api::presenter::fibsum);

            println!("Listening at: http://{}", &bind);
            let result = app.listen(bind).await;
            dbg!(&result);
        })
}
