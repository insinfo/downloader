#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::process;

use clap::{clap_app, crate_version};
use downloader::download::{DefaultEventsHandler, ftp_download, gen_filename, http_download, prep_headers, request_headers_from_server};
use downloader::utils;
use failure::{format_err, Fallible};
use downloader::core::{Config, HttpDownload};

/*mod models;//importa o modulo "models"
use std::mem::size_of_val as sizeOf;
use std::convert::From;

/*use models::pessoa;//importa modulo/struct pessoa
use models::pessoa::CalculaIdade;//importa modulo/trait CalculaIdade*/
use models::pessoa::{Pessoa,CalculaIdade};*/

fn main() {
    /*let p = Pessoa{ name: String::from("Isaque"), idade: 36 };
    println!("{}", p.description());

    let numero = 42;
    let recebe_numero = numero;
    let nome = String::from("Isaque");

    println!("{} ", nome);
    println!("{} ", tamanho_string(&nome));
    println!("{} ", nome);*/

    match run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("error: {}", e);
            process::exit(1);
        }
    }
}




/*fn run() -> Fallible<()> {
    let args = clap_app!(Duma =>
    (version: crate_version!())
    (author: "Matt Gathu <mattgathu@gmail.com>")
    (about: "A minimal file downloader")
    (@arg quiet: -q --quiet "quiet (no output)")
    (@arg continue: -c --continue "resume getting a partially-downloaded file")
    (@arg singlethread: -s --singlethread "download using only a single thread")
    (@arg headers: -H --headers "prints the headers sent by the HTTP server")
    (@arg FILE: -O --output +takes_value "write documents to FILE")
    (@arg AGENT: -U --useragent +takes_value "identify as AGENT instead of Duma/VERSION")
    (@arg SECONDS: -T --timeout +takes_value "set all timeout values to SECONDS")
    (@arg NUM_CONNECTIONS: -n --num_connections +takes_value "maximum number of concurrent connections (default is 8)")
    (@arg URL: +required +takes_value "url to download")
    )
        .get_matches_safe().unwrap_or_else(|e| e.exit());

    let url = utils::parse_url(
        args.value_of("URL")
            .ok_or_else(|| format_err!("missing URL argument"))?,
    )?;
    let quiet_mode = args.is_present("quiet");
    let file_name = args.value_of("FILE");

    match url.scheme() {
        "ftp" => ftp_download(url, quiet_mode, file_name),
        "http" | "https" => http_download(url, &args, crate_version!()),
        _ => utils::gen_error(format!("unsupported url scheme '{}'", url.scheme())),
    }
}*/
fn run() -> Fallible<()> {
    let url = utils::parse_url("https://designscad.com/wp-content/uploads/2017/01/landscape_2d_bmp_graphics__pattern_texture_material__graphics_42845.jpg")?;
    let resume_download = false;
    let concurrent_download =false;
    let user_agent = &format!("Duma/{}","0.1.0");
    let timeout = 30u64;
    let headers = request_headers_from_server(&url, timeout, &user_agent)?;

    let fname = gen_filename(&url, None, Some(&headers));

    let headers = prep_headers(&fname, resume_download, &user_agent)?;


    let conf = Config {
        user_agent: user_agent.clone(),
        resume: false,
        headers,
        file: fname.clone(),
        timeout: timeout.clone(),
        concurrent: false,
        max_retries: 100,
        num_workers: 8usize,
        bytes_on_disk: None,//Some(0u64),
        chunk_offsets:  None,//Some(vec![]),
        chunk_size: 512000,
    };

    println!("conf: {:?}",conf);
    let mut client = HttpDownload::new(url.clone(), conf.clone());
    let quiet_mode = false;
    let events_handler =
        DefaultEventsHandler::new(&fname, resume_download, concurrent_download, quiet_mode)?;
    client.events_hook(events_handler).download()?;

    Ok(())
}
/*fn tamanho_string( str: &String) -> i32{
    return  str.len() as i32;
}*/