use axohtml::{dom::DOMTree, html};

pub fn create() -> DOMTree<String> {
    html!(
        <html lang="en">
        <head>
            <title>"WebAuthn-rs Tutorial"</title>
            <meta charset="UTF-8" />
            <meta name="viewport" content="width=device-width, initial-scale=1" />
        </head>
        <body>
            <p>"Welcome to the WebAuthn Server!"</p>

            <div>
                <input type="text" id="username" placeholder="Enter your username here" />
            </div>

            <div>
                <p id="flash_message"></p>
            </div>

        </body>
        </html>
    )
}
