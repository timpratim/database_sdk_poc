package main

// Import the generated bindings
import (
    "github.com/timpratim/database_sdk_poc"
    "fmt"
)

func main() {
    id := database_sdk_poc.NewRecordId("person", "1")
    person := database_sdk_poc.Create(id, "John", "Doe")
    fmt.Printf("Created person: %+v\n", person)
}
