package main

import (
	"bufio"
	"log"
	"os"
	"strconv"
)

func main() {
	file, err := os.Open("../input.txt")
	if err != nil {
		log.Fatalln("Couldn't open file for reading", err.Error())
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	largest := 0
	temp := 0

	for scanner.Scan() {
		if scanner.Text() == "" {
			if temp > largest {
				largest = temp
			}
			temp = 0
		} else {
			num, err := strconv.Atoi(scanner.Text())
			if err != nil {
				log.Fatalln("Couldn't parse text to number", err.Error())
			}
			temp += num
		}
	}

	log.Println("Higest is", largest)
}
