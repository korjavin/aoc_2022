package main

import (
	"fmt"
	"testing"
)

//
// func TestBuildGraph(t *testing.T) {
// 	testMatrix := [][]string{
// 		{"a", "b", "d"},
// 		{"d", "z", "e"},
// 		{"a", "a", "e"},
// 	}
//
// 	intMatrix := transformMatrix(testMatrix)
//
// 	n, end := buildGraph(intMatrix, [2]int{0, 0}, [2]int{2, 2})
//
// 	var path []*node
// 	var paths [][]*node
//
// 	n.findAllPaths(end, path, &paths)
// 	for _, p := range paths {
// 		for _, n := range p {
// 			fmt.Printf("[%d,%d](%v) ", n.coord[0], n.coord[1], string(rune(n.height)))
// 		}
// 		println()
// 	}
// }

// test queue
func TestQueue(t *testing.T) {
	q := newQueue(
		newNode(7, 0, 7), 7,
	)
	q.add(
		newNode(3, 0, 3), 3,
	)
	n := newNode(1, 1, 0)
	q.add(
		n, 1,
	)
	q.add(
		n, 2,
	)

	for {
		n := q.get()
		if n == nil {
			break
		}
		fmt.Printf("[%d,%d]:%d ", n.coord[0], n.coord[1], n.height)
	}
	println()
}
