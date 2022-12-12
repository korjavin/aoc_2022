package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"sort"
	"strings"
)

// Type Monkey holds slice of items (integers) and to closure function throw_to and worrisome_level
type Monkey struct {
	items           []int
	throw_to        func(int) int
	worrisome_level func(int) int
}

// fabric for throw_to
func ThrowToFactory(div, left, right int) func(int) int {
	return func(i int) int {
		if i%div == 0 {
			return left
		} else {
			return right
		}
	}
}

// Enum operations
type operation int

const (
	PLUS operation = 1
	MULT           = 3
)

// helper operation from string
func operFromString(s string) operation {
	switch s {
	case "+":
		return PLUS
	case "*":
		return MULT
	default:
		panic("Unknown operation")
	}
}

// fabric for worrisome_level
func WorrisomeLevelFactory(level int, oper operation, operand int) func(int) int {
	return func(i int) int {
		switch oper {
		case PLUS:
			return (i + operand)
		case MULT:
			if operand == 0 {
				return (i * i)
			}
			return (i * operand)
		}
		panic("Unknown operation")
		return 0
	}
}

// constructor for Monkey
func NewMonkey(items []int, div, left, right int, oper operation, operand int) *Monkey {
	tr := ThrowToFactory(div, left, right)
	wl := WorrisomeLevelFactory(operand, oper, operand)
	return &Monkey{items, tr, wl}
}

// Monkey get item
func (m *Monkey) GetItem() int {
	if len(m.items) == 0 {
		return -1
	}
	item := m.items[0]
	m.items = m.items[1:]
	return item
}

// monkey add item
func (m *Monkey) AddItem(item int) {
	m.items = append(m.items, item)
}

// Monkey make a step
func (m *Monkey) Step() (int, int, int) {
	item := m.GetItem()
	if item == -1 {
		return -1, 0, 0
	}
	new_wl := m.worrisome_level(item)
	throw_to := m.throw_to(new_wl)
	return item, new_wl, throw_to
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)

	monkeys := map[int]*Monkey{}
	monkeyarr := []*Monkey{}

	var number int
	number = -1
	var items []int
	var div, left, right, operand int
	var operStr string
	var oper operation

	divisor := 1

	for scanner.Scan() {
		line := scanner.Text()
		switch {
		case strings.HasPrefix(line, "Monkey"):
			if number != -1 {
				mnk := NewMonkey(items, div, left, right, oper, operand)
				monkeys[number] = mnk
				monkeyarr = append(monkeyarr, mnk)
				items = []int{}
			}
			_, _ = fmt.Sscanf(line, "Monkey %d:", &number)
		case strings.HasPrefix(line, "  Starting items:"):
			line = strings.TrimPrefix(line, "  Starting items:")
			for _, s := range strings.Split(line, ", ") {
				var i int
				_, _ = fmt.Sscanf(s, "%d", &i)
				items = append(items, i)
			}
		case strings.HasPrefix(line, "  Operation:"):
			operand = 0
			_, _ = fmt.Sscanf(line, "  Operation: new = old %s %d", &operStr, &operand)
			oper = operFromString(operStr)
			log.Printf("Operation: line %s  %s %d", line, operStr, operand)
		case strings.HasPrefix(line, "  Test: divisible by"):
			_, _ = fmt.Sscanf(line, "  Test: divisible by %d", &div)
			divisor = divisor * div
		case strings.HasPrefix(line, "    If true: throw to monkey"):
			_, _ = fmt.Sscanf(line, "    If true: throw to monkey %d", &left)
		case strings.HasPrefix(line, "    If false: throw to monkey"):
			_, _ = fmt.Sscanf(line, "    If false: throw to monkey %d", &right)
		}
	}
	// last one
	mnk := NewMonkey(items, div, left, right, oper, operand)
	monkeys[number] = mnk
	monkeyarr = append(monkeyarr, mnk)
	counter := map[int]int{}

	for round := 0; round < 10000; round++ {
		for n, m := range monkeyarr {
			fmt.Printf("Monkey %d: %v\n", n, m.items)
			for range m.items {
				olditem, item, throw_to := m.Step()
				if olditem == -1 {
					break
				}
				counter[n] = counter[n] + 1
				log.Printf("Monkey %d: %d  %d -> Monkey %d", n, olditem, item, throw_to)
				monkeys[throw_to].AddItem(item % divisor)
			}
		}
	}
	for _, m := range monkeyarr {
		fmt.Printf("%v\n", m.items)
	}
	arr := []int{}
	for _, c := range counter {
		arr = append(arr, c)
	}
	sort.Ints(arr)
	fmt.Printf("%v\n", arr)
	fmt.Printf("score=%d\n", arr[len(arr)-2]*arr[len(arr)-1])
}
