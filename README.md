# http-server

To test server, run the application:

```bash
$ cargo run
```

and pipe a request "TEST" to the TCP socket (IP address + Port number = web socket) 
```bash
$ echo "TEST" | netcat 127.0.0.1 8080
```