package main

import (
	"fmt"
	"math/rand"
	"sync"
)

type Cell struct{ threat int }
type Grid [10][10]Cell

// Worker goroutine
func ColumnSearch(
	column [10]Cell,
	col int,
	messageCh chan<- string,
	dragonFound chan<- struct{},
	done <-chan struct{},
	wg *sync.WaitGroup,
) {
	defer wg.Done()

	for y, cell := range column {

		// Check for cancellation before work
		select {
		case <-done:
			return
		default:
		}

		msg := fmt.Sprintf("Scanning (%d,%d)... ", col, y)

		switch cell.threat {
		case 0:
			msg += "Empty."
		case 1:
			msg += "Goblin."
		case 2:
			msg += "Wolf."
		case 3:
			msg += "Orc."
		case 4:
			msg += "Troll."

		case 5:
			msg += "DRAGON! RUN!"

			// Notify main (non-blocking)
			select {
			case dragonFound <- struct{}{}:
				// dragon reported
			default:
				// already reported by another goroutine
			}

			// Send final dragon message
			messageCh <- msg
			return
		}

		// Normal scan message
		messageCh <- msg
	}
}

func main() {
	grid := Grid{}
	for x := range grid {
		for y := range grid[x] {
			grid[x][y].threat = rand.Intn(6)
		}
	}

	// GUARANTEE a dragon exists (optional)
	grid[rand.Intn(10)][rand.Intn(10)].threat = 5

	messageCh := make(chan string)
	done := make(chan struct{})
	dragonFound := make(chan struct{}, 1)
	var wg sync.WaitGroup

	fmt.Println("*** Column Search Started ***")

	// --- PRINTER GOROUTINE (prevents deadlocks) ---
	var printWg sync.WaitGroup
	printWg.Add(1)
	go func() {
		defer printWg.Done()
		for msg := range messageCh {
			fmt.Println(msg)
		}
	}()

	// --- START WORKER GOROUTINES ---
	for x := range grid {
		wg.Add(1)
		go ColumnSearch(grid[x], x, messageCh, dragonFound, done, &wg)
	}

	// --- WAIT FOR DRAGON SIGNAL ---
	<-dragonFound
	fmt.Println("*** Dragon found — canceling workers! ***")

	// --- CANCEL WORKERS ---
	close(done)

	// --- WAIT FOR ALL WORKERS ---
	wg.Wait()

	// --- CLOSE MESSAGE CHANNEL ---
	close(messageCh)

	// --- WAIT FOR PRINTER TO FINISH ---
	printWg.Wait()

	fmt.Println("*** Search ended cleanly. No deadlock. ***")
}
