use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn newsletter_form(flash_messages: IncomingFlashMessages) -> HttpResponse {
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        writeln!(msg_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">

<head>
  <meta http-equiv="content-type" content="text/html; charset=utf-8">
  <title>Publish Newsletter Issue</title>
</head>

<body>
  {msg_html}
  <form action="/admin/newsletter" method="post">
    <label>
      Title:<br/>
      <input
      type="text"
      placeholder="Enter the issue title"
      name="title"
      >
    </label>

    <label>Plain text content: <br/>
      <textarea
      placeholder="Enter the content in plain text"
      name="plain_content"
      rows="20"
      cols="50"
      />
    </label>

    <label>HTML text content: <br/>
      <textarea
      placeholder="Enter the content in HTML format"
      name="html_content"
      rows="20"
      cols="50"
      />
    </label>

    <button type="submit">Publish</button>
  </form>
  <p><a href="/admin/dashboard">>&lt;- Back</a></p>
</body>
</html>"#
        ))
}
