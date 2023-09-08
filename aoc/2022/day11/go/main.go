package main

import (
	"fmt"
	"math/big"
	"os"
	"regexp"
	"strconv"
	"strings"
)

type Monkey struct {
	Name           string
	TestAgainst    uint64
	ThrowTo        [2]uint64
	InspectedCount uint64
	WorryLevels    []big.Int
	Operations     [2]string
}

// Not working correctly; use rust code for correct implementation
func main() {
	data, err := os.ReadFile("../input.txt")
	if err != nil {
		panic("Error reading file")
	}

	var r = regexp.MustCompile(`\d+|\+|\*|(\d+|old)$`)
	var lines = strings.SplitN(string(data), "\n", -1)
	var monkeys = make([]Monkey, 0)
	var limit int = 10000

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
			InspectedCount: uint64(len(worryLevels)),
			WorryLevels:    stringArrayToBigIntArray(worryLevels),
			Operations:     [2]string{operations[0], operations[1]},
			ThrowTo:        [2]uint64{toUint32(caseTrue[0]), toUint32(caseFalse[0])},
		})
	}

	for i := 1; i <= limit; i++ {
		for i, monkey := range monkeys {

			for _, w := range monkey.WorryLevels {
				var val big.Int

				v, err := strconv.Atoi(monkey.Operations[1])
				if err != nil {
					val = w
				} else {
					val = *big.NewInt(int64(v))
				}

				var worry big.Int

				switch monkey.Operations[0] {
				case "*":
					worry.Set(val.Mul(&val, &w))
				case "+":
					worry.Set(val.Add(&val, &w))
				default:
					panic("Unknown operator")
				}

				// worry.Div(&worry, big.NewInt(3))

				var worryCopy big.Int
				worryCopy.Set(&worry)

				// fmt.Println("Worry:", worry.Int64())

				if worryCopy.Rem(&worryCopy, big.NewInt(int64(monkey.TestAgainst))).Cmp(big.NewInt(0)) == 0 {
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

		if i == limit /* || i%100 == 0 || i%20 == 0 */ {
			for m, monkey := range monkeys {
				monkeys[m].InspectedCount -= uint64(len(monkey.WorryLevels))
				// fmt.Println("Monkey", monkey.Name, "has", monkey.InspectedCount, "inspected values at", i)
			}
		}
	}

	var inspectedCounts []uint64

	for _, v := range monkeys {
		inspectedCounts = append(inspectedCounts, v.InspectedCount)
	}

	fmt.Println("Inspected counts:", inspectedCounts)
	fmt.Println("part_02:", productOfMaxTwo(inspectedCounts))
}

func productOfMaxTwo[T uint64](values []T) T {
	var max T = 0
	var secondMax T = 0

	for _, value := range values {
		if value > max {
			secondMax = max
			max = value
		} else if value > secondMax {
			secondMax = value
		}
	}

	println("Max two:", max, secondMax)
	return max * secondMax
}

func toUint32(value string) uint64 {
	num, err := strconv.Atoi(value)
	if err != nil {
		panic("Couldn't parse number")
	}
	return uint64(num)
}

func stringArrayToBigIntArray(arr []string) []big.Int {
	var returnArray = []big.Int{}
	for _, v := range arr {
		returnArray = append(returnArray, *big.NewInt(int64(toUint32(v))))
	}
	return returnArray
}
