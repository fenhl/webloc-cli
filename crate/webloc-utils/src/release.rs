#![deny(rust_2018_idioms, unused, unused_crate_dependencies, unused_import_braces, unused_lifetimes, unused_qualifications, warnings)]
#![forbid(unsafe_code)]

use {
    std::{
        cmp::Ordering::*,
        future::Future,
        io::{
            self,
            prelude::*,
        },
        process::{
            ExitStatus,
            Stdio,
        },
        time::Duration,
    },
    async_trait::async_trait,
    itertools::Itertools as _,
    reqwest::{
        Body,
        Client,
        StatusCode,
    },
    semver::Version,
    serde::Deserialize,
    serde_json::json,
    tokio::{
        fs,
        process::Command,
    },
};

#[async_trait]
trait CommandOutputExt {
    async fn check(&mut self, name: &'static str) -> Result<ExitStatus, Error>;
}

#[async_trait]
impl CommandOutputExt for Command {
    async fn check(&mut self, name: &'static str) -> Result<ExitStatus, Error> {
        let status = self.status().await?;
        if status.success() {
            Ok(status)
        } else {
            Err(Error::CommandExit(name, status))
        }
    }
}

struct Repo {
    user: String,
    name: String,
}

impl Repo {
    fn new(user: impl ToString, name: impl ToString) -> Repo {
        Repo {
            user: user.to_string(),
            name: name.to_string(),
        }
    }

    async fn latest_release(&self, client: &Client) -> reqwest::Result<Option<Release>> {
        let response = client.get(&format!("https://api.github.com/repos/{}/{}/releases/latest", self.user, self.name))
            .send().await?;
        if response.status() == StatusCode::NOT_FOUND { return Ok(None) } // no releases yet
        Ok(Some(
            response.error_for_status()?
                .json::<Release>().await?
        ))
    }

    async fn create_release(&self, client: &Client, name: String, tag_name: String, body: String) -> reqwest::Result<Release> {
        Ok(
            client.post(&format!("https://api.github.com/repos/{}/{}/releases", self.user, self.name))
                .json(&json!({
                    "body": body,
                    "draft": true,
                    "name": name,
                    "tag_name": tag_name
                }))
                .send().await?
                .error_for_status()?
                .json::<Release>().await?
        )
    }

    async fn publish_release(&self, client: &Client, release: Release) -> reqwest::Result<Release> {
        Ok(
            client.patch(&format!("https://api.github.com/repos/{}/{}/releases/{}", self.user, self.name, release.id))
                .json(&json!({"draft": false}))
                .send().await?
                .error_for_status()?
                .json::<Release>().await?
        )
    }

    fn release_attach<'a>(&self, client: &'a Client, release: &'a Release, name: &'a str, content_type: &'static str, body: impl Into<Body> + 'a) -> impl Future<Output = reqwest::Result<()>> + 'a {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(reqwest::header::CONTENT_TYPE, reqwest::header::HeaderValue::from_static(content_type));
        async move {
            client.post(&release.upload_url.replace("{?name,label}", ""))
                .query(&[("name", name)])
                .headers(headers)
                .body(body)
                .send().await?
                .error_for_status()?;
            Ok(())
        }
    }
}

#[derive(Deserialize)]
struct Release {
    id: u64,
    tag_name: String,
    upload_url: String,
}

impl Release {
    fn version(&self) -> Result<Version, semver::Error> {
        self.tag_name[1..].parse()
    }
}

async fn check_cli_version(package: &str, version: &Version) {
    let cli_output = String::from_utf8(Command::new("cargo").arg("run").arg(format!("--package={}", package)).arg("--").arg("--version").stdout(Stdio::piped()).output().await.expect("failed to run CLI with --version").stdout).expect("CLI version output is invalid UTF-8");
    let (cli_name, cli_version) = cli_output.trim_end().split(' ').collect_tuple().expect("no space in CLI version output");
    assert_eq!(cli_name, package);
    assert_eq!(*version, cli_version.parse().expect("failed to parse CLI version"));
}

async fn version() -> Version {
    let version = Version::parse(env!("CARGO_PKG_VERSION")).expect("failed to parse current version");
    assert_eq!(version, webloc::version());
    check_cli_version("webloc-cli", &version).await;
    version
}

async fn release_client() -> Result<Client, Error> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::AUTHORIZATION, reqwest::header::HeaderValue::from_str(&format!("token {}", fs::read_to_string("assets/release-token.txt").await?))?);
    Ok(Client::builder()
        .user_agent(concat!("webloc-release/", env!("CARGO_PKG_VERSION")))
        .default_headers(headers)
        .timeout(Duration::from_secs(30))
        .http2_prior_knowledge()
        .use_rustls_tls()
        .https_only(true)
        .build()?)
}

async fn write_release_notes() -> Result<String, Error> {
    eprintln!("editing release notes");
    let mut release_notes_file = tempfile::Builder::new()
        .prefix("webloc-cli-release-notes")
        .suffix(".md")
        .tempfile()?;
    Command::new("code").arg("--wait").arg(release_notes_file.path()).check("code").await?;
    let mut buf = String::default();
    release_notes_file.read_to_string(&mut buf)?;
    if buf.is_empty() { return Err(Error::EmptyReleaseNotes) }
    Ok(buf)
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)] InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
    #[error(transparent)] Io(#[from] io::Error),
    #[error(transparent)] Reqwest(#[from] reqwest::Error),
    #[error(transparent)] SemVer(#[from] semver::Error),
    #[error("command `{0}` exited with {1}")]
    CommandExit(&'static str, ExitStatus),
    #[error("aborting due to empty release notes")]
    EmptyReleaseNotes,
    #[error("local crate has the same version as latest release")]
    SameVersion,
    #[error("local crate has a lower version than latest release")]
    VersionRegression,
}

#[wheel::main(debug)]
async fn main() -> Result<(), Error> {
    eprintln!("creating reqwest client");
    let client = release_client().await?;
    //TODO make sure working dir is clean and on default branch and up to date with remote and remote is up to date
    let repo = Repo::new("fenhl", "webloc-cli");
    eprintln!("checking version");
    if let Some(latest_release) = repo.latest_release(&client).await? {
        let remote_version = latest_release.version()?;
        match version().await.cmp(&remote_version) {
            Less => return Err(Error::VersionRegression),
            Equal => return Err(Error::SameVersion),
            Greater => {}
        }
    }
    //TODO make sure Rust is up to date
    let release_notes = write_release_notes().await?;
    eprintln!("creating release");
    let release = repo.create_release(&client, format!("webloc {}", version().await), format!("v{}", version().await), release_notes).await?;
    eprintln!("building webloc CLI for x86_64");
    Command::new("cargo").arg("build").arg("--release").arg("--target=x86_64-apple-darwin").arg("--package=webloc-cli").env("MACOSX_DEPLOYMENT_TARGET", "10.7").check("cargo").await?;
    eprintln!("building webloc CLI for aarch64");
    Command::new("cargo").arg("build").arg("--release").arg("--target=aarch64-apple-darwin").arg("--package=webloc-cli").env("MACOSX_DEPLOYMENT_TARGET", "11.0").check("cargo").await?;
    eprintln!("creating Universal macOS binary");
    Command::new("lipo").arg("-create").arg("target/aarch64-apple-darwin/release/webloc").arg("target/x86_64-apple-darwin/release/webloc").arg("-output").arg("assets/webloc-macos").check("lipo").await?;
    eprintln!("uploading webloc CLI");
    repo.release_attach(&client, &release, "webloc", "application/x-mach-binary", fs::read("assets/webloc-macos").await?).await?;
    eprintln!("publishing release");
    repo.publish_release(&client, release).await?;
    Ok(())
}
