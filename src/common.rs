use clap::{Arg, ArgAction, Command};
use hbb_common::{
    anyhow::{Context, Result}, log, ResultType
};
use ini::Ini;
use sodiumoxide::crypto::sign;
use std::{
    ffi::OsString, io::{prelude::*, Read}, net::SocketAddr, time::{Instant, SystemTime}
};

pub(crate) trait CycleLoop {
    async fn cycle(&self) -> ResultType<()>;
}

#[allow(dead_code)]
pub(crate) fn get_expired_time() -> Instant {
    let now = Instant::now();
    now.checked_sub(std::time::Duration::from_secs(3600))
        .unwrap_or(now)
}

#[allow(dead_code)]
pub(crate) fn test_if_valid_server(host: &str, name: &str) -> ResultType<SocketAddr> {
    use std::net::ToSocketAddrs;
    let res = if host.contains(':') {
        host.to_socket_addrs()?.next().context("")
    } else {
        format!("{}:{}", host, 0)
            .to_socket_addrs()?
            .next()
            .context("")
    };
    if res.is_err() {
        log::error!("Invalid {} {}: {:?}", name, host, res);
    }
    res
}

#[allow(dead_code)]
pub(crate) fn get_servers(s: &str, tag: &str) -> Vec<String> {
    let servers: Vec<String> = s
        .split(',')
        .filter(|x| !x.is_empty() && test_if_valid_server(x, tag).is_ok())
        .map(|x| x.to_owned())
        .collect();
    log::info!("{}={:?}", tag, servers);
    servers
}

#[allow(dead_code)]
#[inline]
fn arg_name(name: &str) -> String {
    name.to_uppercase().replace('_', "-")
}

lazy_static::lazy_static!(
    static ref RENDEZVOUS_PORT: OsString = unsafe { OsString::from_encoded_bytes_unchecked(hbb_common::config::RENDEZVOUS_PORT.to_string().into_bytes()) };
    static ref RMEM: OsString = unsafe { OsString::from_encoded_bytes_unchecked(hbb_common::config::RMEM.to_string().into_bytes()) };
);
#[allow(dead_code)]
pub fn init_args(name: &'static str, about: &'static str) {
    let rendezvous_port = RENDEZVOUS_PORT.as_os_str();
    let rmem = RMEM.as_os_str();
    let matches = Command::new(name)
        .version(crate::version::VERSION)
        .author("Purslane Ltd. <info@rustdesk.com>")
        .about(about)
        .arg(Arg::new("config").short('c').long("config").action(ArgAction::Set).value_name("FILE").help("Sets a custom config file"))
        .arg(Arg::new("port").short('p').long("port").action(ArgAction::Set).value_name("NUMBER").value_parser(clap::value_parser!(u32).range(1..65536)).default_value(rendezvous_port).help("Sets the listening port"))
        .arg(Arg::new("serial").short('s').long("serial").action(ArgAction::Set).value_name("NUMBER").default_value("0").help("Sets configure update serial number"))
        .arg(Arg::new("rendezvous-servers").short('R').long("rendezvous-servers").action(ArgAction::Set).value_name("HOSTS").help("Sets rendezvous servers, separated by comma"))
        .arg(Arg::new("software-url").short('u').long("software-url").action(ArgAction::Set).value_name("URL").help("Sets download url of RustDesk software of newest version"))
        // .arg(Arg::new("relay-servers").short('r').long("relay-servers").action(ArgAction::Set).value_name("HOSTS").help("Sets the default relay servers, separated by comma"))
        .arg(Arg::new("rmem").short('M').long("rmem").action(ArgAction::Set).value_name("NUMBER").default_value(rmem).help("Sets UDP recv buffer size, set system rmem_max first, e.g., sudo sysctl -w net.core.rmem_max=52428800. vi /etc/sysctl.conf, net.core.rmem_max=52428800, sudo sysctl –p"))
        .arg(Arg::new("mask").long("mask").action(ArgAction::Set).value_name("MASK").help("Determine if the connection comes from LAN, e.g. 192.168.0.0/16'"))
        .get_matches();
    if let Ok(v) = Ini::load_from_file(".env") {
        if let Some(section) = v.section(None::<String>) {
            section
                .iter()
                .for_each(|(k, v)| std::env::set_var(arg_name(k), v));
        }
    }
    if let Some(config) = matches.get_one::<String>("config") {
        if let Ok(v) = Ini::load_from_file(config) {
            if let Some(section) = v.section(None::<String>) {
                section
                    .iter()
                    .for_each(|(k, v)| std::env::set_var(arg_name(k), v));
            }
        }
    }
    let keys = matches.ids();
    for k in keys {
        if k.as_str() == "port" {
            if let Some(_v) = matches.get_one::<u32>(k.as_str()) {
                std::env::set_var(arg_name(k.as_str()), _v.to_string());
            }
        } else {
            if let Some(_v) = matches.get_one::<String>(k.as_str()) {
                std::env::set_var(arg_name(k.as_str()), _v.clone());
            }
        }
    }
}

#[allow(dead_code)]
#[inline]
pub fn get_arg(name: &str) -> String {
    get_arg_or(name, "".to_owned())
}

#[allow(dead_code)]
#[inline]
pub fn get_arg_or(name: &str, default: String) -> String {
    std::env::var(arg_name(name)).unwrap_or(default)
}

#[allow(dead_code)]
#[inline]
pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|x| x.as_secs())
        .unwrap_or_default()
}

pub fn gen_sk(wait: u64) -> (String, Option<sign::SecretKey>) {
    let sk_file = "id_ed25519";
    if wait > 0 && !std::path::Path::new(sk_file).exists() {
        std::thread::sleep(std::time::Duration::from_millis(wait));
    }
    if let Ok(mut file) = std::fs::File::open(sk_file) {
        let mut contents = String::new();
        if file.read_to_string(&mut contents).is_ok() {
            let contents = contents.trim();
            let sk = base64::decode(contents).unwrap_or_default();
            if sk.len() == sign::SECRETKEYBYTES {
                let mut tmp = [0u8; sign::SECRETKEYBYTES];
                tmp[..].copy_from_slice(&sk);
                let pk = base64::encode(&tmp[sign::SECRETKEYBYTES / 2..]);
                log::info!("Private key comes from {}", sk_file);
                return (pk, Some(sign::SecretKey(tmp)));
            } else {
                // don't use log here, since it is async
                println!("Fatal error: malformed private key in {sk_file}.");
                std::process::exit(1);
            }
        }
    } else {
        let gen_func = || {
            let (tmp, sk) = sign::gen_keypair();
            (base64::encode(tmp), sk)
        };
        let (mut pk, mut sk) = gen_func();
        for _ in 0..300 {
            if !pk.contains('/') && !pk.contains(':') {
                break;
            }
            (pk, sk) = gen_func();
        }
        let pub_file = format!("{sk_file}.pub");
        if let Ok(mut f) = std::fs::File::create(&pub_file) {
            f.write_all(pk.as_bytes()).ok();
            if let Ok(mut f) = std::fs::File::create(sk_file) {
                let s = base64::encode(&sk);
                if f.write_all(s.as_bytes()).is_ok() {
                    log::info!("Private/public key written to {}/{}", sk_file, pub_file);
                    log::debug!("Public key: {}", pk);
                    return (pk, Some(sk));
                }
            }
        }
    }
    ("".to_owned(), None)
}

#[cfg(unix)]
pub async fn listen_signal() -> Result<()> {
    use hbb_common::tokio;
    use hbb_common::tokio::signal::unix::{signal, SignalKind};

    tokio::spawn(async {
        let mut s = signal(SignalKind::terminate())?;
        let terminate = s.recv();
        let mut s = signal(SignalKind::interrupt())?;
        let interrupt = s.recv();
        let mut s = signal(SignalKind::quit())?;
        let quit = s.recv();

        tokio::select! {
            _ = terminate => {
                log::info!("signal terminate");
            }
            _ = interrupt => {
                log::info!("signal interrupt");
            }
            _ = quit => {
                log::info!("signal quit");
            }
        }
        Ok(())
    })
    .await?
}

#[cfg(not(unix))]
pub async fn listen_signal() -> Result<()> {
    let () = std::future::pending().await;
    unreachable!();
}
