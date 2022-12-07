package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strings"
)

// Dir struct holds name of directory and its subdirectories, parent directory and integer
// that represents sum of all files in this directory
type Dir struct {
	name     string
	parent   *Dir
	children []*Dir
	sum      int
}

// Constructor of Dir struct
func newDir(name string, parent *Dir) *Dir {
	return &Dir{name: name, parent: parent}
}

// Add child directory to parent directory
func (d *Dir) addChild(child *Dir) {
	d.children = append(d.children, child)
}

// Sum all the sum integers of all directories in the tree
func (dir *Dir) sumAll() int {
	sum := dir.sum
	for _, child := range dir.children {
		sum += child.sumAll()
	}
	return sum
}

// Traverse tree and print all directories and their sum
func printAll(dir *Dir, depth int) {
	if dir == nil {
		return
	}
	fmt.Printf("---%s%s%d\n", strings.Repeat(" ", depth), dir.name, dir.sumAll())
	for _, child := range dir.children {
		printAll(child, depth+1)
	}
}

// Parent returns the parent directory of the tree
func (d Dir) Parent() *Dir {
	for d.parent != nil {
		d = *d.parent
	}

	return &d
}

// Traverse tree and print all directories and their sumAll if the sumAll < 10000
func sumAllFiltered(dir *Dir, depth int) int {
	if dir == nil {
		return 0
	}
	sum := 0
	if dir.sumAll() <= 100000 {
		// fmt.Printf("---%s%s%d\n", strings.Repeat(" ", depth), dir.name, dir.sumAll())
		sum += dir.sumAll()
	}
	for _, child := range dir.children {
		sum += sumAllFiltered(child, depth+1)
	}
	return sum
}

func main() {
	// Scanner that reads from stdin line by line
	var curdur *Dir
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		line := scanner.Text()
		switch {
		case strings.HasPrefix(line, "$ cd"):
			// If line starts with $ cd, then we are changing directory
			// We need to find directory that we are changing to
			// We take name from third word in line
			name := strings.Split(line, " ")[2]
			if name == ".." {
				// If we are changing to parent directory, then we just change curdir to parent directory
				curdur = curdur.parent
			} else {
				dir := newDir(name, curdur)
				if curdur != nil {
					// If we are changing to child directory, then we add child directory to parent directory
					curdur.addChild(dir)
				}
				curdur = dir
			}
		case strings.HasPrefix(line, "$ ls"):
			// If line starts with $ ls, then we are listing all directories in current directory
			// We don't need to do anything
		case strings.HasPrefix(line, "dir"):
			// If line starts with dir, then it's a child directory, just skip
		default:
			// Parse first word of line as integer and add it to sum of current directory
			if curdur == nil {
				// should never happen
				panic("curdir is nil" + line)
			}
			var sum int
			fmt.Sscanf(line, "%d", &sum)
			curdur.sum += sum
		}
	}
	curdur = curdur.Parent()
	sum := curdur.sumAll()
	const fs = 70000000
	const need = 30000000
	freespace := fs - sum
	needdelete := need - freespace

	sizes := curdur.ChildSizes()
	sort.Ints(sizes)

	for i := len(sizes) - 1; i > 0; i-- {
		if sizes[i] < needdelete {
			fmt.Println(sizes[i+1])
			return
		}
	}
}

func (d *Dir) ChildSizes() []int {
	var sizes []int
	sizes = append(sizes, d.sumAll())
	for _, child := range d.children {
		sizes = append(sizes, child.ChildSizes()...)
	}
	return sizes
}
