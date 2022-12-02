package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestCompareAndAdd(t *testing.T) {
	t3 := top3{}

	t3.CompareAndAdd(6)
	t.Log(t3)
	assert.Equal(t, 6, t3.Sum())

	t3.CompareAndAdd(4)
	t.Log(t3)
	assert.Equal(t, 10, t3.Sum())

	t3.CompareAndAdd(11)
	t.Log(t3)
	assert.Equal(t, 21, t3.Sum())

	t3.CompareAndAdd(24)
	t.Log(t3)
	assert.Equal(t, 41, t3.Sum())

	t3.CompareAndAdd(10)
	t.Log(t3)
	assert.Equal(t, 45, t3.Sum())
}
