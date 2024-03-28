# Movie Fetcher

Movie Fetcher is a command-line application that fetches movie data from a movie database API.

## Features

- Fetch and display information about movies from a public API.
- Command-line arguments for easy and flexible use.

## Requirements

- Rust and Cargo (the Rust package manager) installed on your machine.
- An API key for the [Movie Database API](https://rapidapi.com/).

## Installation

1. Clone the repository to your local machine:

2. Build the project using Cargo
    `cargo build --release`
    This will create an executable in the target/release directory.

### Usage
After building the project, you can run the application from the command line.
`./target/release/movie_fetcher "Movie Name" --api-key "your-rapidapi-key"`

## Example
`./target/release/movie_fetcher "The Matrix" --api-key "abc123xyz"`

## Development
To contribute to this project or test locally, you can run the application using Cargo:

`cargo run -- "The Matrix" --api-key "abc123xyz"`


## Contributing
Contributions are welcome! Please feel free to submit a pull request.


- Replace `https://github.com/yourusername/movie_fetcher.git` with the actual URL of your GitHub repository.
- You may need to provide additional details on getting an API key from the Movie Database API or link to the documentation.
- Adjust the executable path and example commands according to your build setup and file structure.
- Include a `LICENSE` file in your repository if you reference it in the README.


