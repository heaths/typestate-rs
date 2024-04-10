// Copyright 2024 Heath Stewart.
// Licensed under the MIT License. See LICENSE.txt in the project root for license information.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", azure_typestate_example::say_hello(None));
    Ok(())
}
