package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

// Type node is a node in a graph.
type node struct {
	coord  [2]int
	height int
	edge   []*node
}

// constructor
func newNode(x, y, h int) *node {
	return &node{
		coord:  [2]int{x, y},
		height: h,
	}
}

// addEdge adds an edge to the node.
func (n *node) addEdge(e *node) {
	n.edge = append(n.edge, e)
}

// findAllPaths finds all paths from the node to the destination.
func (n *node) findAllPaths(dest *node, path []*node, paths *[][]*node) {
	max := shortestPath(paths)
	path = append(path, n)
	if max != nil && len(path) > len(max) { // We already know better
		return
	}
	if n == dest {
		*paths = append(*paths, path)
		fmt.Printf("\nFound path len %d", len(path))
		for _, p := range path {
			fmt.Printf("%v ", p.coord)
		}
		fmt.Println()
		return
	}
	for _, e := range n.edge {
		if !e.existsInPath(path) {
			e.findAllPaths(dest, path, paths)
		}
	}
}

// check if node exists in the path
func (n *node) existsInPath(path []*node) bool {
	for _, p := range path {
		if p == n {
			return true
		}
	}
	return false
}

// findNode with the given coordinates.
func (n *node) findNode(x, y int) *node {
	for _, n := range n.edge {
		if n.coord[0] == x && n.coord[1] == y {
			return n
		}
	}

	for _, n := range n.edge {
		return n.findNode(x, y)
	}
	return nil
}

// buildGraph builds a graph from the given matrix.
func buildGraph(matrix [][]int, start, end [2]int) (*node, *node) {
	var endNode *node
	var startNode *node
	// build nodes
	nodes := make([][]*node, len(matrix))
	for i := range nodes {
		nodes[i] = make([]*node, len(matrix[i]))
		for j := range nodes[i] {
			nodes[i][j] = newNode(i, j, matrix[i][j])
			if i == end[0] && j == end[1] {
				endNode = nodes[i][j]
				endNode.height = 'z'
			}
			if i == start[0] && j == start[1] {
				startNode = nodes[i][j]
				startNode.height = 'a'
			}
		}
	}
	// build edges
	for i := range nodes {
		for j := range nodes[i] {
			n := nodes[i][j]
			if i > 0 && n.height+1 >= nodes[i-1][j].height {
				n.addEdge(nodes[i-1][j])
			}
			if i < len(nodes)-1 && n.height+1 >= nodes[i+1][j].height {
				n.addEdge(nodes[i+1][j])
			}
			if j > 0 && n.height+1 >= nodes[i][j-1].height {
				n.addEdge(nodes[i][j-1])
			}
			if j < len(nodes[i])-1 && n.height+1 >= nodes[i][j+1].height {
				n.addEdge(nodes[i][j+1])
			}
		}
	}
	return startNode, endNode
}

// tranformMatrix of chars to ints.
func transformMatrix(matrix [][]string) [][]int {
	var m [][]int
	for _, row := range matrix {
		var r []int
		for _, c := range row {
			r = append(r, int(c[0]))
		}
		m = append(m, r)
	}

	return m
}

// find shortest path
func shortestPath(paths *[][]*node) []*node {
	var shortest []*node
	for _, path := range *paths {
		if shortest == nil || len(path) < len(shortest) {
			shortest = path
		}
	}
	if len(*paths) > 3 {
		*paths = [][]*node{shortest}
	}
	return shortest
}

func main() {
	// read matrix from stdin
	var matrix [][]string
	scanner := bufio.NewScanner(os.Stdin)
	rowN := 0
	var end [2]int
	var start [2]int
	for scanner.Scan() {
		row := strings.Split(scanner.Text(), "")
		matrix = append(matrix, row)
		for i, c := range row {
			if c == "S" {
				start = [2]int{rowN, i}
			}
			if c == "E" {
				end = [2]int{rowN, i}
			}
		}
		rowN++
	}
	intMatrix := transformMatrix(matrix)
	startNode, endNode := buildGraph(intMatrix, start, end)

	fmt.Printf("start: %v", startNode.coord)
	fmt.Printf("end: %v", endNode.coord)
	// find all paths
	var paths [][]*node
	startNode.findAllPaths(endNode, nil, &paths)
	fmt.Println(len(paths))
	fmt.Println(len(shortestPath(&paths)) - 1)
	for _, n := range shortestPath(&paths) {
		fmt.Printf("[%d,%d](%v) ", n.coord[0], n.coord[1], string(rune(n.height)))
	}
}
