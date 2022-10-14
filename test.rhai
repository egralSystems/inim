import "greeter" as g;

print(g::greet("Bob"));

for directory in ls(".") {
	print(directory);
}

let google = tcp();
debug(google.connect("176.9.48.143", 80));
// print(google.recv);