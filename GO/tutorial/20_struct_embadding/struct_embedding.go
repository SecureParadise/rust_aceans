package main

import (
	"fmt"
	// "reflect"
)

type customer struct {
	name  string
	phone string
}

type order struct {
	id     string
	amt    float32
	status string
	customer
}

func main() {
	// phone := 9810234567
	// fmt.Println(reflect.TypeOf(phone))
	// newCustomer := customer{
	// 	name:  "jonny",
	// 	phone: "981023456",
	// }
	flipkart_order := order{
		id:     "1",
		amt:    32.0,
		status: "received",
		// customer: newCustomer,
		customer: customer{
			name:  "jonny",
			phone: "981023456"},
	}
	fmt.Println(flipkart_order)
	fmt.Println(flipkart_order.customer)

}
