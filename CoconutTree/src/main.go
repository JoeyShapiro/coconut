package main

import (
	"errors"
	"fmt"
	"net"
)

const (
	CONN_HOST   = "localhost"
	CONN_PORT   = "42069"
	PKT_VERSION = 1
)

func main() {
	fmt.Println("Hello, World!")
	address := CONN_HOST + ":" + CONN_PORT
	ln, err := net.Listen("tcp", address)
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
	// handle connection
	fmt.Printf("Connection from %s\n", conn.RemoteAddr())
	// conn.Write([]byte("Hello, World!"))
	for {
		buf := make([]byte, 4096)
		n, err := conn.Read(buf)
		if err != nil {
			fmt.Println("Error reading:", err.Error())
			break
		}
		fmt.Printf("Received %d Bytes\n", n)

		p, err := Packet{}.Parse(buf[:n])
		if err != nil {
			fmt.Println("Error parsing packet:", err.Error())
			conn.Write([]byte{PKT_VERSION, PKT_FUCKOFF})
			break
		}
		// fmt.Printf("%+v\n", p)

		if p.Type == PKT_GREETINGS {
			data := []byte{PKT_VERSION, PKT_GREETINGS, p.ID}
			conn.Write(data)
		} else if p.Type == PKT_SAMPLE {
			fmt.Println(len(p.Data), p.Data[:10])
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
