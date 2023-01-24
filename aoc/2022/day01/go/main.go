package main

import (
	"bufio"
	"log"
	"os"
	"sort"
	"strconv"
)

func main() {
	file, err := os.Open("../input.txt")

	if err != nil {
		log.Fatalln("Couldn't open file for reading", err.Error())
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)
	sums := []int{0}

	for scanner.Scan() {
		if scanner.Text() == "" {
			sums = append(sums, 0)
		} else {
			num, err := strconv.Atoi(scanner.Text())
			if err != nil {
				log.Fatalln("Couldn't parse text to number", err.Error())
			}
			sums[len(sums)-1] += num
		}
	}

	sort.Slice(sums, func(i, j int) bool {
		return sums[i] > sums[j]
	})

	log.Println("Highest is", sums[0])
	sumOfTopThree := 0

	for i := 0; i < 3; i++ {
		sumOfTopThree += sums[i]
	}

	log.Println("Sum of top three", sumOfTopThree)
}
