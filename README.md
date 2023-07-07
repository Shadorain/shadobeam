# Shadobeam

## Description

> Shadobeam is a blazingly fast simple implementation of a beaconing implant
> with both a client and a server written in Rust.

### Task

- Client
	- "Checks in" to server for tasking with a variable heartbeat interval with jitter.
	- If tasking is available it will execute said task on the client machine.
		 - Task will simply be shellcode for now.
 - Server
	- Sits on a task queue.
	- When the client "checks in", the server will distribute any tasking it has to the client.
    - TODO: TUI and/or GUI interface for adding more tasking.
    - TODO: Scriptable tasking.
 - Comms will be done using a `TLV` protocol over `TCP`.

## License

GPLv3: http://www.gnu.org/licenses/
