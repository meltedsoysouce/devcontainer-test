#!/bin/bash

echo "Testing Todo CLI Application..."

# Test help command
echo -e "help\nexit" | cargo run --quiet

echo -e "\n--- Testing add command ---"
echo -e "add Buy groceries\nlist\nexit" | cargo run --quiet

echo -e "\n--- Testing done command ---"
echo -e "add Task 1\nadd Task 2\nlist\ndone 1\nlist\nexit" | cargo run --quiet

echo -e "\n--- Testing delete command ---"
echo -e "add Task to delete\nlist\ndelete 1\nlist\nexit" | cargo run --quiet

echo -e "\nAll tests completed!"