# Simple UDP Server/Client (SUS)

SUS is a command-line tool written in Rust for sending and receiving data over UDP. It supports sending text messages, as well as transferring files between computers on the same network.

## Usage

```
sus [-h] [-m <MESSAGE> <ADDRESS> | -f <PATH> <ADDRESS> | -s <PORT>]
```

- `-h`: Displays the help message and usage instructions.
- `-m <MESSAGE> <ADDRESS>`: Sends the specified message to the given address.
- `-f <PATH> <ADDRESS>`: Sends the file located at the specified path to the given address.
- `-s <PORT>`: Starts a listener on the specified port to receive messages or files. Port 4123 is reserved for sending.

## Examples

### Sending a Message

sus -m "Hello, World!" 192.168.1.100:4123

### Sending a File

sus -f "/path/to/file.txt" 192.168.1.100:4123

### Receiving Messages or Files (Starting server)

sus -s "4123"

## How it Works

The application uses the `std::net::UdpSocket` to establish UDP communication. When sending a message or file, it first sends a header containing metadata about the data being sent (type and size). The receiver acknowledges the header, and then the actual data is transmitted.

For file transfers, the file is read into memory before being sent. If a file with the same name already exists in the download directory, the application will automatically rename the incoming file with a numerical suffix to avoid overwriting.

## Dependencies

- `std` (Rust standard library)
- `dirs_2` (for locating the user's download directory)

## License

This project is licensed under the [MIT License](LICENSE).
