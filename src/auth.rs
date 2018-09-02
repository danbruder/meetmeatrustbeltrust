//!
//! This example showcases the Github OAuth2 process for requesting access to the user's public repos and
//! email address.
//!
//! Before running it, you'll need to generate your own Github OAuth2 credentials.
//!
//! In order to run the example call:
//!
//! ```sh
//! GITHUB_CLIENT_ID=xxx GITHUB_CLIENT_SECRET=yyy cargo run --example github
//! ```
//!
//! ...and follow the instructions.
//!

extern crate actix_web;
extern crate base64;
extern crate oauth2;
extern crate rand;
extern crate url;
use oauth2::basic::BasicClient;
use oauth2::prelude::*;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl,
};
use std::env;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use url::Url;

fn main() {
    let github_client_id = ClientId::new(
        env::var("GITHUB_CLIENT_ID").expect("Missing the GITHUB_CLIENT_ID environment variable."),
    );
    let github_client_secret = ClientSecret::new(
        env::var("GITHUB_CLIENT_SECRET")
            .expect("Missing the GITHUB_CLIENT_SECRET environment variable."),
    );
    let auth_url = AuthUrl::new(
        Url::parse("https://github.com/login/oauth/authorize")
            .expect("Invalid authorization endpoint URL"),
    );
    let token_url = TokenUrl::new(
        Url::parse("https://github.com/login/oauth/access_token")
            .expect("Invalid token endpoint URL"),
    );

    // Set up the config for the Github OAuth2 process.
    let client = BasicClient::new(
            github_client_id,
            Some(github_client_secret),
            auth_url, Some(token_url)
        )
        // This example is requesting access to the user's public repos and email.
        .add_scope(Scope::new("public_repo".to_string()))
        .add_scope(Scope::new("user:email".to_string()))

        // This example will be running its own server at localhost:8080.
        // See below for the server implementation.
        .set_redirect_url(
            RedirectUrl::new(
                Url::parse("http://localhost:8080")
                    .expect("Invalid redirect URL")
            )
        );

    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, csrf_state) = client.authorize_url(CsrfToken::new_random);

    println!(
        "Open this URL in your browser:\n{}\n",
        authorize_url.to_string()
    );
}
