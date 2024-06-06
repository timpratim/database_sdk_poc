package main

// Import the generated bindings
import (
	"fmt"

	"github.com/timpratim/database_sdk_poc/database_sdk_poc"
)

func main() {
	// Entry point for the surrealdb ffi library
	surrealdb := database_sdk_poc.NewSurrealdb()
	defer surrealdb.Destroy()
	surrealdb.Create("Joh")
	persons := surrealdb.Select()

	// defer person.Destroy()
	fmt.Printf("Created person: %+v\n", persons)
}
