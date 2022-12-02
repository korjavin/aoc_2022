package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
)

type top3 struct {
	sum [3]int // 0>1>2
}

func (t top3) String() string {
	return fmt.Sprintf("%d %d %d", t.sum[0], t.sum[1], t.sum[2])
}

func (t top3) Sum() int {
	return t.sum[0] + t.sum[1] + t.sum[2]
}

func (t *top3) CompareAndAdd(i int) {
	for idx := 0; idx < 3; idx++ {
		if t.sum[idx] < i {
			for j := 2; j > idx; j-- {
				// fmt.Printf("idx, j, t = %d, %d, %s\n", idx, j, t)
				t.sum[j] = t.sum[j-1]
			}
			t.sum[idx] = i
			return
		}
	}
}

func main() {
	current := 0
	t3 := top3{}

	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			t3.CompareAndAdd(current)
			current = 0
			continue
		}

		number, err := strconv.Atoi(line)
		if err != nil {
			log.Fatalf("Can't parse %q: %v", line, err)
		}
		current += number
	}
	t3.CompareAndAdd(current)

	fmt.Printf("%s, %d", t3, t3.Sum())
}
