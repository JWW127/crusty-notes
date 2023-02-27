// this will not just run you need to include module in main
// this is example is mostly from Programming_Rust_2_E

use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

fn actix_main() {
    // create new server
    let server = HttpServer::new(|| {
        // App let us create application instance
        App::new()
            .route("/", web::get().to(get_index)) // @ get route / call get_index
            .route("/gcd", web::post().to(post_gcd)) // @ post route /gcd call post_gcd
    });

    println!("serving on localhost:3000");
    //set up ser
    server
        .bind("127.0.0.1:3000") // bind to localhost:300
        .expect("error binding server")
        .run() // start this bad boy
        .expect("error running server");
}

fn get_index() -> HttpResponse {
    //actix provides a way to create content of "text/html" as a res body
    HttpResponse::Ok().content_type("text/html").body(
        r#"
            <!DOCTYPE html>
            <html lang="en" data-theme="dark">

            <head>
                <title>GCD</title>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <link href="https://unpkg.com/nes.css@latest/css/nes.min.css" rel="stylesheet" />
            </head>

            <div class="nes-container is-rounded is-dark">
            <title>GCD CALC</title>
                <form action="/gcd" method="post">
                    <input class="nes-input" type="text" name="n" placeholder="Enter First Number"/>
                    <input class="nes-input" type="text" name="m" placeholder="Eneter Second Number"/>
                    <button class="nes-btn is-primary" type="submit">Compute</button>
                </form>
            "#,
    )
}

// greatest common divisor
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

// this is called in our post route when we click submit
fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("computing GCD with zero is ... ");
    }

    let response = format!(
        "The GCD of {} and {} \
            is <b>{}</b>\n",
        form.n,
        form.m,
        gcd(form.n, form.m)
    );

    HttpResponse::Ok().content_type("text/html").body(response)
}

/*
This is kinda cool like a simple little web app basically
*/
