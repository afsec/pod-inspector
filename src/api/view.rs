use crate::in_memory_db::Session;
use std::env;
use crate::{VERSION,PROGRAM_NAME};

pub async fn render_page(session: Session, output_command: Option<String>) -> String {
    let hostname = match env::var("HOSTNAME") {
        Ok(val) => val,
        Err(_) => "None".to_string(),
    };

    let output_command = match output_command {
        Some(str) => str,
        None => "".to_string(),
    };
    let html = format!(
        r#"
<head>
    <title>{} - v.{}</title>
    <style>
        * {{
            margin: 0;
            padding: 0;
            background-color: #111;
            color: #fff;
        }}
        #main {{
            font-family: monospace;
            margin-left: auto;
            margin-right: auto;
        }}
        #menu {{
            margin-left: auto;
            margin-right: auto;
            width: 600px;
        }}
        .header {{
            margin-left: auto;
            margin-right: auto;
            text-align: center;
        }}
        a {{
            color: #0074d9;
        }}
        p.option:hover {{
            background-color: #4d4200;
        }}
        #cmd-output{{
            margin-top: 10px;
            margin-left: 10px;
        }}
        #footer {{
            text-align: center;
        }}
    </style>
</head>
<body>
    <div id="main">
        <h1 class="header">Pod Inspector</h1>
        <h2 class="header">hostname:[{}]</h2>
        <div id="menu">
            <p class="option"><a href="/">/</a> This page</p>
            <p class="option"><a href="/printenv">/printenv</a> Print Environment Variables</p>
            <p class="option"><a href="/memory">/memory</a> Show free memory</p>
            <p class="option"><a href="/disk">/disk</a> Show disk space</p>
            <p class="option"><a href="/interfaces">/interfaces</a> Print network interfaces info</p>
            <p class="option"><a href="/processes">/processes</a> Print processes list</p>
            <p class="option"><a href="/cpuinfo">/cpuinfo</a> Show CPU info</p>
            <p class="option"><a href="/netstat">/netstat</a> Print sockets info (TCP/UDP/Unix)</p>
            <p class="option"><a href="/show-lsof">/show-lsof</a> Show list of opened files</p>
            <p class="option"><a href="/show-root">/show-root</a> List files at root filesystem (/)</p>
            <p class="option"><a href="/show-pwd">/show-pwd</a> List files at current working directory (pwd)</p>
            <p class="option"><a href="/uptime">/uptime</a> Print uptime</p>
            <p class="option"><a href="/fib">/fib</a> CPU test using Fibonacci (Recommend value: 42)</p>
            <br/>
            <p class="option"><a href="/logout">Logout({})</a></p>

        </div>
            <hr/>
                <pre id="cmd-output">{}</pre>
            <br/>
            <hr/>
            <p id="footer">Powered by <a href="https://www.rust-lang.org/">Rust Programming Language</a><br/>
            Copyright 2020 - MIT Licensed by <a href="https://github.com/afsec/">afsec</a>.
            <p>
    </div>
</body>
"#,
        PROGRAM_NAME,
        VERSION,
        hostname,
        session.get_username(),
        output_command
    );
    html
}
