use clap::Clap;
use data_encoding::BASE32;
use ed25519_dalek::{Keypair, PublicKey};
use env_logger::Target;
use hex;
use log::{debug, info, LevelFilter};
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

/// Onion-vanity allows you to generate vanity onion addresses.
#[derive(Clap)]
#[clap(
    name = "Onion Vanity",
    version = "1.0",
    author = "Aurèle Oulès <aurele@oules.com>"
)]
struct Opts {
    /// The pattern the onion address must start with
    #[clap(short, long)]
    pattern: String,
    /// Number of threads
    #[clap(short, long, default_value = "4")]
    threads: i64,

    #[clap(short, long)]
    verbose: bool,
}

fn compute_checksum(pk: &PublicKey) -> Vec<u8> {
    let mut vec = Vec::new();

    vec.extend_from_slice(".onion checksum".as_bytes());
    vec.extend_from_slice(&pk.to_bytes());
    vec.push(0x03);

    let mut hasher = Sha256::new();
    hasher.update(vec);
    let result = hasher.finalize();

    result.to_vec()
}

fn pub_to_base32(pk: &PublicKey) -> String {
    BASE32.encode(&pk.to_bytes()).to_lowercase()
}

fn compute_address(pk: &PublicKey) -> String {
    let checksum = compute_checksum(pk);

    let mut address = Vec::new();
    address.extend_from_slice(&pk.to_bytes());
    address.extend_from_slice(&checksum[..2]);
    address.push(0x03);

    format!("{}.onion", BASE32.encode(&address).to_lowercase())
}

fn mine_address(threads: i64, pattern: String) -> (Keypair, String) {
    let done = Arc::new((AtomicBool::new(false), Mutex::new(None)));

    fn work(pattern: String, done: Arc<(AtomicBool, Mutex<Option<Keypair>>)>) {
        let mut csprng = OsRng {};
        let (is_done, lock) = &*done;

        while !is_done.load(Ordering::Relaxed) {
            let keypair: Keypair = Keypair::generate(&mut csprng);
            let addr = pub_to_base32(&keypair.public);

            if addr.starts_with(&pattern) {
                *lock.lock().unwrap() = Some(keypair);
                is_done.store(true, Ordering::Release);
            }
        }
    }

    debug!("Starting {} threads.", threads);
    for i in 0..threads {
        let done = Arc::clone(&done);
        let pattern = pattern.clone();
        thread::spawn(move || {
            debug!("Started thread {}", i);

            work(pattern, done);
        });
    }

    let d = Arc::clone(&done);
    work(pattern, done);

    let b =
        d.1.lock()
            .unwrap()
            .as_ref()
            .map(|f| (f.to_bytes()))
            .clone()
            .unwrap();

    let key = Keypair::from_bytes(&b).unwrap();
    let addr = compute_address(&key.public);
    (key, addr)
}

fn main() {
    let opts: Opts = Opts::parse();

    env_logger::builder()
        .target(Target::Stdout)
        .filter_level(if opts.verbose {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        })
        .init();
    
    info!("Started mining address: {}", opts.pattern);
    let (keypair, addr) = mine_address(opts.threads, opts.pattern);

    info!("Found onion address: {}", addr);
    info!("Private key: {}",
        hex::encode(keypair.secret.to_bytes())
    );    
}
