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

let _ = file.read_<str/blob>();          // Read the whole file as a string or blob
let _ = file.read_<str/blob>(12);        // Read 12 characters
let _ = file.read_<str/blob>('\n');      // Read until character(here '\n')

let _ = file.write(<string>); // Write string into the file
let _ = file.write(<blob>); // Write blob to file

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
  
  // The read/write interface is the same as for the files
}
```


## The Inim interface <Rust side>

### Overriding default Inim interfaces
The interfaces are done with generics. If you want to override the defaults you need to place your interface implementations int these fields.
`
  let mut inim = <Inim<ConsoleIf, FileIf, SysIf, NetIf>>::new();
  // Replace them if you want. These interfaces are shown below.
`

### Console If
`
  pub trait Console: Clone + 'static {
      fn print(text: &str);
      fn debug(text: &str);
  }
`
### File If
```
  pub trait File: Clone + 'static {
      fn open(path: &str, options: &str) -> Self;
      fn close(&mut self);

      fn seek(&mut self, offset: usize);
      fn step(&mut self, step: i64);

      fn read_blob_all(&mut self) -> Vec<u8>;
      fn read_blob_amount(&mut self, amount: i64) -> Vec<u8>;

      fn read_string_all(&mut self) -> String;
      fn read_char(&mut self) -> char;

      fn read_string_amount(&mut self, amount: usize) -> String {
          let mut output = String::new();

          for _ in 0..amount {
              output.push(self.read_char());
          }

          output
      }

      fn read_string_until(&mut self, stop: char) -> String {
          let mut output = String::new();

          loop {
              let ch = self.read_char();

              if ch == stop {
                  break;
              }

              output.push(ch);
          }

          output
      }

      fn write_string(&mut self, text: &str);
      fn write_blob(&mut self, blob: Vec<u8>);
  }
```

### Sys If
```
  pub trait Sys: Clone + 'static {
      fn ls(path: &str) -> Vec<Dynamic>;
      fn mkdir(path: &str) -> bool;
      fn rm(path: &str) -> bool;
      fn rmdir(path: &str) -> bool;
      fn time() -> f64;
      fn path() -> String;
  }
```

### Net If
```
  pub trait Net: Clone + 'static {
      fn tcp() -> Self; // Create TCP socket

      fn addr(&mut self) -> String;

      fn bind(&mut self, addr: &str) -> String; // Start server
      fn connect(&mut self, addr: &str) -> String; // Connect to server

      fn set_timeout(&mut self, timeout: i64); // Set recv timeout

      fn accept(&mut self) -> Self; // Wait for connections

      fn send_string(&mut self, msg: &str) -> String; // Send string
      fn recv_string(&mut self, char_count: i64) -> String; // Receive a string
      fn recv_line(&mut self) -> String;

      fn send_blob(&mut self, msg: Vec<u8>) -> String; // Send bytes
      fn recv_blob_amount(&mut self, byte_count: i64) -> Vec<u8>; // Receive a blob
      fn recv_blob(&mut self) -> Vec<u8>;

      fn close(&mut self); // Close socket
  }
```
