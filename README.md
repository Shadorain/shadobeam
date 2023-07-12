# Shadobeam

## Description

> Shadobeam is a blazingly fast simple implementation of a beaconing implant
> written in Rust.

### Task

- Client (shadobeam_implant)
    - "Checks in" to server for tasking with a variable heartbeat interval with jitter.
    - If tasking is available it will execute said task on the client machine.
    - Tasks will simply be shellcode for now.
- Server (shadobeam_implant)
    - Sits on a task queue.
    - When the client "checks in", the server will distribute any tasking it has to the client.
    - Provided new tasking via `shadobeam_interface`.
    - Should support multiple interface and implant connections.
- Interface (shadobeam_interface)
    - TODO: TUI and/or GUI interface for adding more tasking.
    - TODO: Scriptable tasking.
    - Connect to server and provide any tasks.
- Comms will be done using a `gRPC` protocol over `TCP`.

## License

GPLv3: http://www.gnu.org/licenses/
