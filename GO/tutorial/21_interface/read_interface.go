package main

import "fmt"

///////////////////////////////////////////////////////////
// INTERFACE DEFINITION
///////////////////////////////////////////////////////////

// Naming convention: interface names typically end with "-er" (e.g., Logger, Reader, Paymenter)
type Paymenter interface {
	pay(amount float32)                    // Method to perform a payment
	refund(amount float32, account string) // Method to perform a refund
}

///////////////////////////////////////////////////////////
// STRUCT THAT USES THE INTERFACE
///////////////////////////////////////////////////////////

// `payment` struct uses a gateway that follows the Paymenter interface
type Payment struct {
	gateway Paymenter
}

// makePayment calls the payment and refund methods of the assigned payment gateway.
// This follows the **Open/Closed Principle**: open to new payment gateways, no need to modify this code.
func (p Payment) makePayment(amount float32, account string) {
	p.gateway.pay(amount)
	p.gateway.refund(amount, account)
}

///////////////////////////////////////////////////////////
// IMPLEMENTATIONS OF THE PAYMENTER INTERFACE
///////////////////////////////////////////////////////////

// RazorPay implementation
type RazorPay struct{}

func (r RazorPay) pay(amount float32) {
	fmt.Println("Making payment using RazorPay:", amount)
}

func (r RazorPay) refund(amount float32, account string) {
	fmt.Printf("Refunding ₹%.2f to %s using RazorPay\n", amount, account)
}

// Stripe implementation
type Stripe struct{}

func (s Stripe) pay(amount float32) {
	fmt.Println("Making payment using Stripe:", amount)
}

func (s Stripe) refund(amount float32, account string) {
	fmt.Printf("Refunding ₹%.2f to %s using Stripe\n", amount, account)
}

// FakePayment for testing or mocking
type FakePayment struct{}

func (f FakePayment) pay(amount float32) {
	fmt.Println("Fake payment of ₹", amount)
}

func (f FakePayment) refund(amount float32, account string) {
	fmt.Printf("Pretending to refund ₹%.2f to %s (fake)\n", amount, account)
}

// PayPal implementation
type PayPal struct{}

func (p PayPal) pay(amount float32) {
	fmt.Println("Making payment using PayPal:", amount)
}

func (p PayPal) refund(amount float32, account string) {
	fmt.Printf("Refunding ₹%.2f to %s using PayPal\n", amount, account)
}

///////////////////////////////////////////////////////////
// MAIN FUNCTION
///////////////////////////////////////////////////////////

func main() {
	// You can plug in any payment gateway that implements the Paymenter interface

	// Using PayPal
	paypalGateway := PayPal{}
	paymentViaPaypal := Payment{
		gateway: paypalGateway,
	}
	paymentViaPaypal.makePayment(708, "yoda")

	// Using Stripe
	stripeGateway := Stripe{}
	paymentViaStripe := Payment{
		gateway: stripeGateway,
	}
	paymentViaStripe.makePayment(500, "tony")

	// Using RazorPay
	razorGateway := RazorPay{}
	paymentViaRazor := Payment{
		gateway: razorGateway,
	}
	paymentViaRazor.makePayment(300, "steve")

	// Using FakePayment (for testing)
	fakeGateway := FakePayment{}
	testPayment := Payment{
		gateway: fakeGateway,
	}
	testPayment.makePayment(0, "test_account")
}
