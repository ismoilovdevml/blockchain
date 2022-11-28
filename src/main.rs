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

    let mut behavior = RecipeBehaviour {
        floodsub: Floodsub::new(PEER_ID.clone()),
        mdns: TokioMdns::new().expect("mdns yaratish"),
        respomse_sender,
    };

    behavior.floodsub.subscribe(TOPIC.clone());


    let mut swarm = SwarmBuilder::new(transp, behavior, PEER_ID.clone())
        .executor(Box::new(|fut|){
            tokio::spawn(fut);
        })
        .build();

    let mut stdin = tokio::io::BufReader::new(tokio::io::stdin()).lines();

    Swarm::listen_on(
        &mut swarm,
        "/ip2/0.0.0.0/tcp/0"
            .parce()
            .expect("local socket olish mumkin")
    )
    .expect("swarm boshlash mumkin");

    loop {
        let evt = {
        tokio::slect! {
            line = stdin.next_line() => Some(EventType::Input(line.expect("qatorni olish").expect("stdin dan satrni o'qiy oladi"))),
            event = swarm.next() => {
                info!("Ishlamaydigan Swarm: {:?}", event);
                None
            },
            response = response_rcv.recv() => Some(EventType::response(response.expect("javob mavjud"))),
        }
      };
    }
}