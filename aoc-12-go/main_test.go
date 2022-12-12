package main

import (
	"fmt"
	"testing"
)

func TestBuildGraph(t *testing.T) {
	testMatrix := [][]string{
		{"a", "b", "d"},
		{"d", "z", "e"},
		{"a", "a", "e"},
	}

	intMatrix := transformMatrix(testMatrix)

	n, end := buildGraph(intMatrix, [2]int{0, 0}, [2]int{2, 2})

	var path []*node
	var paths [][]*node

	n.findAllPaths(end, path, &paths)
	for _, p := range paths {
		for _, n := range p {
			fmt.Printf("[%d,%d](%v) ", n.coord[0], n.coord[1], string(rune(n.height)))
		}
		println()
	}
}
