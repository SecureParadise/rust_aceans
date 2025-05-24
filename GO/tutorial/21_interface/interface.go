package main

import "fmt"

// interface name c0nvention "name+er"
// logger,storeer,
type paymenter interface {
	pay(amount float32)
	refund(amount float32, account string)
}
type payment struct {
	// gateWay stripe
	gateWay paymenter
}

// voilating open close principle
// should be open for extension and close for modification
func (p payment) makePayment(amount float32, account string) {
	// razorPayPaymentGateWay := razorPay{}
	// stripePayPaymentGateWay := stripe{}

	// razorPayPaymentGateWay.pay(amount)
	// stripePayPaymentGateWay.pay(amount)
	p.gateWay.pay(amount)
	p.gateWay.refund(amount, account)
}

type razorPay struct{}

func (r razorPay) pay(amount float32) {
	// logic to pay
	fmt.Println("Making payment using razoropay", amount)
}

type stripe struct{}

func (s stripe) pay(amount float32) {
	fmt.Println("Making payment using stripe", amount)
}

type fakePayment struct{}

func (f fakePayment) pay(amount float32) {
	fmt.Println("PAyment using fakepayment", amount)
}

// now integrating paypal
type payPal struct{}

func (p payPal) pay(amount float32) {
	fmt.Println("Payment using paypal", amount)
}
func (p payPal) refund(amount float32, account string) {
	fmt.Printf("refund %f to %s", amount, account)
}
func main() {
	// stripePayPaymentGw := stripe{}
	// new_stripe_payment := payment{
	// 	gateWay: stripePayPaymentGw,
	// }
	// new_stripe_payment.makePayment(222)
	// razorPayPaymentGw := razorPay{}
	// new_razor_payment := payment{
	// 	gateWay: razorPayPaymentGw,
	// }
	// new_razor_payment.makePayment(222)

	// fakeGw := fakePayment{}
	// new_fake_payment := payment{
	// 	gateWay: fakeGw,
	// }
	// new_fake_payment.makePayment(999)

	muskPay := payPal{}
	new_musky := payment{
		gateWay: muskPay,
	}
	new_musky.makePayment(708, "yoda")
}
