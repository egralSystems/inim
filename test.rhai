let client = tcp();
debug(client.connect("127.0.0.1:6969"));
client.send_string("Hello world!");