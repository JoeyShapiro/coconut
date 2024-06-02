package main

import (
	"errors"
	"fmt"
	"io"
	"net"
)

const (
	CONN_HOST   = "localhost"
	CONN_PORT   = "42069"
	PKT_VERSION = 1
)

var (
	users          map[uint8]UserConn
	maxConnections uint8
)

func main() {
	fmt.Println("Hello, World!")
	maxConnections = 8

	users = make(map[uint8]UserConn, maxConnections)

	address := CONN_HOST + ":" + CONN_PORT
	ln, err := net.Listen("tcp", address)
	fmt.Println("Listening on", address)
	if err != nil {
		panic(err)
	}

	for {
		conn, err := ln.Accept()
		if err != nil {
			// handle error
			fmt.Println("Error accepting:", err.Error())
			continue
		}
		go handleConnection(conn)
	}
}

func handleConnection(conn net.Conn) {
	var id uint8
	// handle connection
	fmt.Printf("Connection from %s\n", conn.RemoteAddr())
	// conn.Write([]byte("Hello, World!"))
	for {
		buf := make([]byte, 4096)
		n, err := conn.Read(buf)
		if err == io.EOF {
			fmt.Println("Connection closed by client")
			if id != 0 {
				RemoveUser(id)
				fmt.Printf("user with id %d has left\n", id) // server only cares about the id
			}
			break
		} else if err != nil {
			fmt.Println("Error reading:", err.Error())
			break
		}
		// fmt.Printf("Received %d Bytes\n", n)

		p, err := Packet{}.Parse(buf[:n])
		// this will handle the length and all that stuff for each packet type
		// if something inm the packet is invalid, it will return an error at the right time
		if err != nil {
			fmt.Println("Error parsing packet:", err.Error())
			fmt.Printf("%+v\n", buf[:n])
			conn.Write([]byte{PKT_VERSION, PKT_FUCKOFF})
			break
		}

		if p.Type == PKT_GREETINGS {
			var data []byte
			user := UserConn{Conn: &conn, User: string(p.Data)}
			id, err = InsertUser(user)
			if err != nil {
				data = []byte{PKT_VERSION, PKT_GREETINGS, 0}
			} else {
				data = []byte{PKT_VERSION, PKT_GREETINGS, id}
			}

			fmt.Printf("%s has joined with the id %d\n", user.User, id)

			conn.Write(data)
		} else if p.Type == PKT_SAMPLE {
			if len(p.Data) != 2048 {
				fmt.Println("Invalid data length", len(p.Data))
			}

			data := []byte{PKT_VERSION, PKT_OK} // TODO maybe send the length or something, but ok should be good enough
			conn.Write(data)
		}
	}
	conn.Close()
	fmt.Printf("Connection closed with %s\n", conn.RemoteAddr())
}

const (
	PKT_FUCKOFF = iota
	PKT_GREETINGS
	PKT_SAMPLE
	PKT_OK
)

type UserConn struct {
	Conn *net.Conn
	User string
}

func InsertUser(user UserConn) (uint8, error) {
	// we will always skip 0. that is reserved
	for i := 1; i < int(maxConnections+1); i++ {
		if _, ok := users[uint8(i)]; !ok {
			users[uint8(i)] = user
			return uint8(i), nil
		}
	}

	return 0, errors.New("could not insert user")
}

func RemoveUser(id uint8) error {
	if _, ok := users[id]; !ok {
		return errors.New("user does not exist")
	}

	delete(users, id)
	return nil
}

type Packet struct {
	Version uint8
	Type    uint8
	ID      uint8
	Data    []byte
}

func (Packet) Parse(data []byte) (p Packet, err error) {
	if len(data) < 2 {
		err = errors.New("packet too short")
		return
	}

	p.Version = data[0]
	if p.Version != PKT_VERSION {
		err = errors.New("version mismatch")
		return
	}

	p.Type = data[1]
	p.ID = data[2]
	p.Data = data[3:] // TODO pretty sure this is right

	return
}
