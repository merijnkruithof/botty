# Pegasus
Pegasus is a Habbo load tester. It's able to spawn many bots concurrently.

Roadmap:
- [X] Authentication
- [X] Ping pong
- [X] API call to add and remove auth tickets
- [X] Enter rooms
- [ ] API authentication
- [ ] Updating look
- [ ] Sending friend requests
- [ ] Creating rooms
- [ ] Divide and conquer (create rooms & spawn bots to it)
- [ ] Proxy support

### Usage
There's currently no application that implements the API. Compile Pegasus Server and run commands like this:

```bash
curl -XPOST "http://localhost:666/api/sessions/add" --data '{"auth_ticket": "aardbei"}' -H "Content-Type: application/json" -v
Note: Unnecessary use of -X or --request, POST is already inferred.
*   Trying 127.0.0.1:666...
* Connected to localhost (127.0.0.1) port 666 (#0)
> POST /api/sessions/add HTTP/1.1
> Host: localhost:666
> User-Agent: curl/8.1.2
> Accept: */*
> Content-Type: application/json
> Content-Length: 26
> 
< HTTP/1.1 200 OK
< content-length: 0
< date: Thu, 14 Mar 2024 05:20:00 GMT
< 
* Connection #0 to host localhost left intact
```

Look at `src/main.rs` for all API requests.