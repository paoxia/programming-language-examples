package main

import (
	"bufio"
	"errors"
	"fmt"
	"io"
	"os"
)

func main() {
	file, err := os.Open("./text")
	if err != nil {
		fmt.Println(err.Error())
		return
	}
	defer file.Close()

	fileReader := bufio.NewReader(file)
	for {
		readerStr, readError := fileReader.ReadString('\n')
		fmt.Printf("The readerStr is: %v", readerStr)
		if errors.Is(readError, io.EOF) {
			return
		}
	}
}
