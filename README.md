# Inim


## The Inim API (rhai)

### Console

```
print(<smth>);
debug(<smth>);
```

### Files

```
let file = open(<path>, <options: r | w | a >); // Open file

let _ = file.read();          // Read the whole file
let _ = file.read(12);        // Read 12 characters
let _ = file.read('\n');      // Read until character(here '\n')

let _ = file.write(<string>); // Write string into the file

file.seek(<abs pos>);         // Goto absolute position in the file
file.step(<+/- offset>);      // Go back and forth by the offset

file.close();                 // Close the file
```

### Modules

```
import "<module name here>" as <alias> // Import a module as alias
```

### System

```
ls("<path>");    // Return a table of file in <path>
rm("<path>");    // Remove FILE at <path>

mkdir("<path>"); // Create directory at <path>
rmdir("<path>"); // Remove directory at <path>

time();          // Return time in seconds from UNIX_EPOCH
path();          // Returns current directory string
```

### Networking

```
let net = net(); // Create a network instance

// Server
if net.bind("<addr>:<port>") == "OK" { // Bind server to address
  // Success
} else {
  // Fail
}

loop {
  let client = net.accept(); // Wait for clients
  print(client.read_line()); // Read whole line from client
}
```