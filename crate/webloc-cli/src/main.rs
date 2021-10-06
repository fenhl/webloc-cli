#![deny(rust_2018_idioms, unused, unused_crate_dependencies, unused_import_braces, unused_lifetimes, unused_qualifications, warnings)]
#![forbid(unsafe_code)]

use {
    std::{
        fmt,
        io::{
            self,
            prelude::*,
            stdin,
            stdout,
        },
        path::PathBuf,
    },
    derive_more::From,
    structopt::StructOpt,
    url::Url,
    webloc::Webloc,
};

#[derive(StructOpt)]
enum Args {
    /// Output the URL contained in a webloc file.
    Read {
        /// Which webloc file to read. Defaults to stdin.
        #[structopt(parse(from_os_str))]
        path: Option<PathBuf>,
    },
    /// Store a URL in a webloc file.
    Save {
        /// Where to save the webloc. Defaults to stdout.
        #[structopt(parse(from_os_str))]
        path: Option<PathBuf>,
        /// Which URL to save. Read from stdin if omitted.
        url: Option<Url>,
        /// Write the webloc as a human-readable XML file rather than the more compact binary format.
        #[structopt(short, long)]
        xml: bool,
    },
}

#[derive(From)]
enum Error {
    Io(io::Error),
    Plist(plist::Error),
    Url(url::ParseError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "I/O error: {}", e),
            Self::Plist(e) => write!(f, "error reading/writing webloc plist file: {}", e),
            Self::Url(e) => write!(f, "invalid URL: {}", e),
        }
    }
}

#[wheel::main]
fn main(args: Args) -> Result<(), Error> {
    match args {
        Args::Read { path } => {
            let Webloc { url } = if let Some(path) = path {
                plist::from_file(path)?
            } else {
                let mut buf = Vec::default();
                stdin().read_to_end(&mut buf)?;
                plist::from_bytes(&buf)? // can't use plist::from_reader since stdin isn't Seek
            };
            println!("{}", url);
        }
        Args::Save { path, url, xml } => {
            let url = if let Some(url) = url {
                url
            } else {
                let mut buf = String::default();
                stdin().read_to_string(&mut buf)?;
                buf.parse()?
            };
            let webloc = Webloc { url };
            if let Some(path) = path {
                if xml {
                    plist::to_file_xml(path, &webloc)?;
                } else {
                    plist::to_file_binary(path, &webloc)?;
                }
            } else {
                if xml {
                    plist::to_writer_xml_with_options(stdout(), &webloc, &plist::XmlWriteOptions::default().indent_string("    "))?;
                } else {
                    plist::to_writer_binary(stdout(), &webloc)?;
                }
            }
        }
    }
    Ok(())
}
