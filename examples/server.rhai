let server = tcp();
debug(server.bind("127.0.0.1:6969"));

loop {
	let client = server.accept();
	print(client.recv_line());
}
