//! This is a simple example to demonstrate the usage of this library.

const _RESPONSE: &str = r#"<!doctype html>
<html>
<head>
    <title>Example Domain</title>

    <meta charset="utf-8" />
    <meta http-equiv="Content-type" content="text/html; charset=utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <style type="text/css">
    body {
        background-color: #f0f0f2;
        margin: 0;
        padding: 0;
        font-family: -apple-system, system-ui, BlinkMacSystemFont, "Segoe UI", "Open Sans", "Helvetica Neue", Helvetica, Arial, sans-serif;

    }
    div {
        width: 600px;
        margin: 5em auto;
        padding: 2em;
        background-color: #fdfdff;
        border-radius: 0.5em;
        box-shadow: 2px 3px 7px 2px rgba(0,0,0,0.02);
    }
    a:link, a:visited {
        color: #38488f;
        text-decoration: none;
    }
    @media (max-width: 700px) {
        div {
            margin: 0 auto;
            width: auto;
        }
    }
    </style>
</head>

<body>
<div>
    <h1>Example Domain</h1>
    <p>This domain is for use in illustrative examples in documents. You may use this
    domain in literature without prior coordination or asking for permission.</p>
    <p><a href="https://www.iana.org/domains/example">More information...</a></p>
</div>
</body>
</html>"#;

fn main() -> Result<(), bitreq::Error> {
    // TODO: For this request object to be useful we probably need to
    // either make the `Request` fields all public or make the
    // `ParsedRequest` and associated types public.
    //
    // Either option is a reasonably invasive change in design so
    // needs thorough consideration.

    // FIXME: Found to be broken while importing from minireq.

    // let request = bitreq::get("http://example.com");

    // // Do what you need to do to send the request to the server.
    // let response = Response::from_bytes(RESPONSE);

    // let body = response.as_str()?;
    // println!("{}", body);

    Ok(())
}
