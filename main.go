package main

// Import the generated bindings
import (
    "github.com/timpratim/database_sdk_poc/database_sdk_poc"
    "fmt"
)

func main() {
    surrealdb := database_sdk_poc.NewSurrealdb()
    defer surrealdb.Destroy()
    surrealdb.Create(database_sdk_poc.Person{Name: "Joh"})
    persons := surrealdb.Select()
    
   // defer person.Destroy()
    fmt.Printf("Created person: %+v\n", persons)
}
