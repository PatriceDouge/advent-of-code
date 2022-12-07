package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {

	filePath := os.Args[1]
	readFile, err := os.Open(filePath)

	if err != nil {
		fmt.Println(err)
	}
	fileScanner := bufio.NewScanner(readFile)
	fileScanner.Split(bufio.ScanLines)
	var fileLines []string

	for fileScanner.Scan() {
		fileLines = append(fileLines, fileScanner.Text())
	}

	readFile.Close()

	currentCal := 0
	maxCal := 0

	for _, num := range fileLines {

		// convert input string to int, if space - error
		cal, err := strconv.Atoi(num)

		currentCal += cal

		// if not empty, theres a space, check if currentCal > maxCal
		if err != nil {
			if currentCal > maxCal {
				maxCal = currentCal
			}
			currentCal = 0
		}

	}

	fmt.Println(maxCal)
}
