package main

import "fmt"

// enumerated type
type OrderStatus int

const (
	Received OrderStatus = iota
	Confiremed
	Prepared
	Delivered
)

type Order_Status string

const (
	Leliya Order_Status = "Leliya"
	Pakka               = "Pakka"
	Tayar               = "Tayar"
	Pahuch              = "Pahuch"
)

// func changeOrderStatus(status string) {
// 	fmt.Println("changing oreder status to", status)
// }
func changeOrderStatus(status OrderStatus) {
	fmt.Println("changing oreder status to", status)
}
func change_OrderStatus(status Order_Status) {
	fmt.Println("changing oreder status to", status)
}

func main() {

	changeOrderStatus(Prepared)
	change_OrderStatus(Leliya)
}
