#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub hash: String,
    pub previous_hash: String,
    pub timestamp: i64,
    pub data: String,
    pub nonce: u64
}

// libp2p client yaratish
#[tokio::main]
aysnc fn main() {
    pretty_env_logger::init();

    info!("Peer ID: {}", PEER_ID.clone());
    let (respomse_sender, mut respomse_rcv) = mpsc::unbounded_channel();
    let auth_keys = Keypair::<X25519Spec>::new()
        .into_authentic(&KEYS)
        .expect("Autentifikatsiya kalitlarini yaratish")

    let transp = TokioTcpConfig::new()
        .upgrade(upgrade::Version::V1)
        .authenticate(NoiseConfig::xx(auth_keys).into_authentic)
        .multiplex(mplex::MpexConfig::new())
        .boxed();
}