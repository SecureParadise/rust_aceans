package main

import (
	// "fmt"
	"os"
)

func main() {
	// read file
	f, err := os.Open("example.txt")
	if err != nil {
		// log the error
		// panic
		panic(err)
	}
	defer f.Close()
	buf := make([]byte, 10)
	d, err := f.Read(buf)
	if err != nil {
		panic(err)
	}
	println("data", d, buf)
	// filee_info, erroror := f.Stat()
	// if erroror != nil {
	// 	// log the error
	// 	// panic
	// 	panic(erroror)
	// }
	// fmt.Println("File Name :->", filee_info.Name())
	// fmt.Println("File IsDir :->", filee_info.IsDir())
	// fmt.Println("File Size:->", filee_info.Size())
	// fmt.Println("File Mode:->", filee_info.Mode())
	// fmt.Println("File ModTime:->", filee_info.ModTime())
}
