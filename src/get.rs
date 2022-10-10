use futures::{Future, Stream};
use ipfs_api::IpfsClient;
use std::io::{self, Write};

let client = IpfsClient::default();

pub fn get_pkg() -> impl Future<Item = (), Error = io::Error> {
    let req = client
    .get("/test/file.json")
    .concat2()
    .map(|res| {
        let out = io::stdout();
        let mut out = out.lock();

        out.write_all(&res).unwrap();
    })
    .map_err(|e| eprintln!("{}", e));
    Ok(())
}
