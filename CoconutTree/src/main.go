package main

import (
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
	conn.Write([]byte("Hello, World!"))
	for {
		buf := make([]byte, 1024)
		n, err := conn.Read(buf)
		if err != nil {
			// handle error
			fmt.Println("Error reading:", err.Error())
			break
		}
		fmt.Printf("Received (%d): %v\n", n, buf[:n])
		conn.Write([]byte{PKT_GREETINGS, 1})
	}
	conn.Close()
	fmt.Printf("Connection closed with %s\n", conn.RemoteAddr())
}

const (
	PKT_FUCKOFF = iota
	PKT_GREETINGS
)
