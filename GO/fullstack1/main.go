package main

import (
	"fmt"
	"log"

	"github.com/gofiber/fiber/v2"
)

type Todo struct {
	ID        int    `json:"id"`
	Completed bool   `json:"completed"`
	Body      string `json:"body"`
}

func main() {
	fmt.Println("hello world")
	app := fiber.New()

	todos := []Todo{}
	app.Get("/", func(c *fiber.Ctx) error {
		return c.Status(200).JSON(fiber.Map{"msg": "HELLO"})
	})
	app.Post("/api/todos", func(c *fiber.Ctx) error {
		todo := &Todo{} //{id:0,completed:false,Body: }
		if err := c.BodyParser(todo); err != nil {
			return err
		}
		if todo.Body == "" {
			return c.Status(400).JSON(fiber.Map("error":"Todo Body is required"))
		}
		toto.ID = len(todos)+1
		todos = append(todos, *todo)
		return c.Sta
	})
	log.Fatal(app.Listen(":4000"))
}
