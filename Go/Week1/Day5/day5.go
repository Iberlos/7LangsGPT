package main

import (
	"fmt"
	"math/rand"
	"sync"
)

type Grid [10][10]struct{ threat int }

func MaxInt(a, b int) int {
	if a > b {
		return a
	} else {
		return b
	}
}

func StandardSearchGrid(grid *Grid) {
	fmt.Println("***Standard Search Started***")
	for x, column := range grid {
		for y, cell := range column {
			fmt.Printf("Scaning cell (%d,%d)...", x, y)
			switch {
			case cell.threat == 0:
				fmt.Println("It is an empty room.")
			case cell.threat == 1:
				fmt.Println("You see something... Its a Goblin!")
			case cell.threat == 2:
				fmt.Println("You see something... Its a Wolf!")
			case cell.threat == 3:
				fmt.Println("You see something... Its a Orc!")
			case cell.threat == 4:
				fmt.Println("You see something... Its a Troll!")
			case cell.threat == 5:
				{
					fmt.Println("You see something... Oh no! Its a Dragon! RUN!")
					return
				}
			}
		}
	}
	fmt.Println("Search complete")
}

func ColumnSearch(column [10]struct{ threat int }, column_index int, message_channel chan string, dragon_channel chan bool, done chan struct{}, wg *sync.WaitGroup) {
	//Closing pattern 1
	defer wg.Done()
	for y, cell := range column {
		//Closing pattern 2
		select {
		case <-done:
			return
		default:
		}
		//Message generation
		message := fmt.Sprintf("Scaning cell (%d,%d)...", column_index, y)
		switch {
		case cell.threat == 0:
			message = message + "It is an empty room."
		case cell.threat == 1:
			message = message + "You see something... Its a Goblin!"
		case cell.threat == 2:
			message = message + "You see something... Its a Wolf!"
		case cell.threat == 3:
			message = message + "You see something... Its a Orc!"
		case cell.threat == 4:
			message = message + "You see something... Its a Troll!"
		case cell.threat == 5:
			{
				message = message + "You see something... Oh no! Its a Dragon! RUN!"
				dragon_channel <- true
			}
		}
		message_channel <- message
	}
}

func PrintMessages(message_channel chan string, done chan struct{}, wg *sync.WaitGroup) {
	defer wg.Done()
Loop:
	for {
		select {
		case msg := <-message_channel:
			fmt.Println(msg)
		case <-done:
			for msg := range message_channel {
				fmt.Println(msg)
			}
			break Loop
		}
	}
}

func main() {
	//Exercise 1
	stamina := 0.0
	if stamina >= 50 {
		fmt.Println("You can use a special attack!")
	} else if stamina >= 20 {
		fmt.Println("You can only attack normally!")
	} else {
		fmt.Println("You are too tired to fight!")
	}

	//Exercise 2
	max_stamina := 100.0
	stamina_percent := 100 * stamina / max_stamina
	switch {
	case stamina_percent >= 75:
		fmt.Println("Your stamina is high")
	case stamina_percent >= 50:
		fmt.Println("Your stamina is medium")
	case stamina_percent >= 25:
		fmt.Println("Your stamina is low")
	default:
		fmt.Println("Your stamina is too low to perform this action")
	}

	//Exercise 3
	party := []struct {
		name string
		hp   int
		max  int
	}{
		{"Astarion", 32, 40},
		{"Shadowheart", 20, 45},
		{"Wyll", 0, 50},
		{"Lae'zel", 47, 50},
	}

	for _, character := range party {
		health_percent := 100.0 * character.hp / character.max
		switch {
		case health_percent >= 75:
			fmt.Printf("%s is Healthy.\n", character.name)
		case health_percent >= 50:
			fmt.Printf("%s is Hurt.\n", character.name)
		case health_percent >= 20:
			fmt.Printf("%s is Wounded.\n", character.name)
		case health_percent >= 1:
			fmt.Printf("%s is Critical.\n", character.name)
		default:
			fmt.Printf("%s is Dead.\n", character.name)
		}
	}

	//Exercise 4
	items := []struct {
		name   string
		weight float64
	}{
		{"Longsword", 8.0},
		{"Potion", 0.5},
		{"Shield", 12.0},
		{"Herbs", 1.5},
	}

	total_weight := 0.0
	for _, item := range items {
		total_weight += item.weight
	}

	if total_weight <= 20 {
		fmt.Println("You feel light.")
	} else if total_weight <= 40 {
		fmt.Println("You feel burdened.")
	} else if total_weight <= 60 {
		fmt.Println("You feel very heavy.")
	} else {
		fmt.Println("You cannot move.")
	}

	//Exercise 5
	grid := Grid{}

	for x := range grid {
		for y := range grid[x] {
			grid[x][y].threat = rand.Intn(6)
		}
	}

	StandardSearchGrid(&grid)

	//Go Beyond
	var wg sync.WaitGroup
	message_channel := make(chan string)
	dragon_channel := make(chan bool)
	done := make(chan struct{})
	fmt.Println("***Column Search Started***")
	go PrintMessages(message_channel, done, &wg)
	for x := range grid {
		wg.Add(1)
		go ColumnSearch(grid[x], x, message_channel, dragon_channel, done, &wg)
	}
Loop:
	for {
		fmt.Println(".")
		if <-dragon_channel {
			close(done)
			break Loop
		}
	}
	wg.Wait()
	select {
	case <-done:
		fmt.Println("***Search Ended Early***")
	default:
		fmt.Println("***Search Ended***")
	}
}
