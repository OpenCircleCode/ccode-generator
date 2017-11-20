
extern crate futures;
extern crate hyper;
extern crate tokio_core;

use std::io::{self, Write};
use self::futures::{Future, Stream};
use self::hyper::Client;

pub fn image(url : String) {

	let uri = url.parse::<hyper::Uri>().unwrap();

	if uri.scheme() != Some("http") {
		println!("This example only works with 'http' URLs.");
		return;
	};

	let mut core = tokio_core::reactor::Core::new().unwrap();

	let handle = core.handle();
	let client = Client::configure()
		.no_proto()
		.build(&handle);

	

	let work = client.get(uri).and_then(|res| {
		println!("Response: {}", res.status());
		println!("Headers: \n{}", res.headers());
		let body = res.body();
		println!("Body: \n{:?}", body);

		body.for_each(|chunk| {
			io::stdout().write_all(&chunk).map_err(From::from)
		})
	}).map(|_| {
		println!("\n\nDone.");
	});

	core.run(work).unwrap();
}

