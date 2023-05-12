package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

type Monkey struct {
	Name           string
	TestAgainst    uint32
	ThrowTo        [2]uint32
	InspectedCount uint32
	WorryLevels    []uint32
	Operations     [2]string
}

func main() {
	data, err := os.ReadFile("../_input.txt")
	if err != nil {
		panic("Error reading file")
	}

	var r = regexp.MustCompile(`\d+|\+|\*|(\d+|old)$`)
	var lines = strings.SplitN(string(data), "\n", -1)
	var monkeys = make([]Monkey, 0)

	for i := 0; i < len(lines)/7; i++ {
		name := r.FindAllString(lines[i*7+0], -1)
		worryLevels := r.FindAllString(lines[i*7+1], -1)
		operations := r.FindAllString(lines[i*7+2], -1)
		testAgainst := r.FindAllString(lines[i*7+3], -1)
		caseTrue := r.FindAllString(lines[i*7+4], -1)
		caseFalse := r.FindAllString(lines[i*7+5], -1)

		monkeys = append(monkeys, Monkey{
			Name:           name[0],
			TestAgainst:    toUint32(testAgainst[0]),
			InspectedCount: uint32(len(worryLevels)),
			WorryLevels:    stringArrayToUint32Array(worryLevels),
			Operations:     [2]string{operations[0], operations[1]},
			ThrowTo:        [2]uint32{toUint32(caseTrue[0]), toUint32(caseFalse[0])},
		})
	}

	for i := 1; i <= 20; i++ {
		for i := 0; i < len(monkeys); i++ {
			monkey := monkeys[i]

			for _, w := range monkey.WorryLevels {
				v, err := strconv.Atoi(monkey.Operations[1])
				if err != nil {
					v = int(w)
				}

				var worry uint32

				switch monkey.Operations[0] {
				case "*":
					worry = uint32(v) * w
				case "+":
					worry = uint32(v) + w
				default:
					panic("Unknown operator")
				}

				worry /= 3

				// fmt.Println("Worry:", worry)

				if worry%monkey.TestAgainst == 0 {
					monkeys[monkey.ThrowTo[0]].WorryLevels = append(monkeys[monkey.ThrowTo[0]].WorryLevels, worry)
					monkeys[monkey.ThrowTo[0]].InspectedCount += 1
					// println("Throwing to monkey", monkey.ThrowTo[0])
				} else {
					monkeys[monkey.ThrowTo[1]].WorryLevels = append(monkeys[monkey.ThrowTo[1]].WorryLevels, worry)
					monkeys[monkey.ThrowTo[1]].InspectedCount += 1
					// println("Throwing to monkey", monkey.ThrowTo[1])
				}

				monkeys[i].WorryLevels = monkeys[i].WorryLevels[1:]
			}
		}

		if i == 20 {
			for i, monkey := range monkeys {
				monkeys[i].InspectedCount -= uint32(len(monkey.WorryLevels))
			}
		}
	}

	var inspectedCounts []uint32

	for _, v := range monkeys {
		inspectedCounts = append(inspectedCounts, v.InspectedCount)
	}

	fmt.Println("part_01:", productOfMaxTwo(inspectedCounts))
}

func productOfMaxTwo(values []uint32) uint32 {
	var max uint32 = 0
	var secondMax uint32 = 0

	for _, value := range values {
		if value > max {
			secondMax = max
			max = value
		} else if value > secondMax {
			secondMax = value
		}
	}

	return max * secondMax
}

func toUint32(value string) uint32 {
	num, err := strconv.Atoi(value)
	if err != nil {
		panic("Couldn't parse number")
	}
	return uint32(num)
}

func stringArrayToUint32Array(arr []string) []uint32 {
	var returnArray = []uint32{}
	for _, v := range arr {
		returnArray = append(returnArray, toUint32(v))
	}
	return returnArray
}
