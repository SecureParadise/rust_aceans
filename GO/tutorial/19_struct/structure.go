package main

import (
	"fmt"
	"time"
)

// order struct
type order struct {
	id        int
	amt       float32
	status    string
	createdAt time.Time
}

// constructor
// its like hack ,there is no any constructor in golang
func newOrder(id int, amt float32, status string) *order {
	// initial setup here ...
	my_order := order{
		id:     id,
		amt:    amt,
		status: status,
	}
	return &my_order
}

// function to change status
// byconvention use struct first letter
// receiver type
// to change value of struct use *
func (o *order) changeStatus(status string) {
	o.status = status
}

// to get value * is optional
func (o order) getAmount() float32 {
	return o.amt
}

func main() {
	// var daraz_order order
	// flipkart_ortder :={
	// 	1,32,"ready_to_ship",time.Now()
	// }
	flipkart_order := order{
		id:        1,
		amt:       32.0,
		status:    "ready_to_ship",
		createdAt: time.Now(), // assign current time
	}
	fmt.Println(flipkart_order.createdAt)
	fmt.Println(flipkart_order)

	amazon_order := order{
		id:        2,
		amt:       3500,
		status:    "process",
		createdAt: time.Now(),
	}

	fmt.Println("amzon", amazon_order)
	amazon_order.changeStatus("delivered")
	fmt.Println("amzon", amazon_order)
	fmt.Println(amazon_order.getAmount())
}
