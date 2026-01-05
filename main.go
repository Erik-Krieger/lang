package main

import (
	"flag"
	"fmt"
	"os"
	"strings"
)

const (
	FILE_EXTENSION string = ".lang"
)

var emitIntermediateLanguage *bool

func init() {
	emitIntermediateLanguage = flag.Bool("il", false, "")
	flag.Parse()
}

func main() {
	nonFlagArgs := flag.Args()

	if len(nonFlagArgs) == 0 || nonFlagArgs[0] == "" {
		fmt.Println("No input file specified.")
		os.Exit(1)
	}

	fileName := nonFlagArgs[0]

	if !strings.HasSuffix(fileName, FILE_EXTENSION) {
		fmt.Println("Invalid input file.")
		os.Exit(1)
	}

	fileInfo, err := os.Stat(fileName)
	if err != nil {
		fmt.Println("The specified path doesn't exist.")
		os.Exit(1)
	}

	if fileInfo.IsDir() {
		fmt.Println("The specified path is a directory.")
		os.Exit(1)
	}

	tokens, err := parseFile(fileName)
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	err = tokenize(tokens)
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	fmt.Println("All done.")
}
