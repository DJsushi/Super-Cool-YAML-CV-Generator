# Rust CV Generation command line tool

Welcome! This _should_ be a command line tool written in Rust that imports a CV written in a type-safe YAML file and
exports it (eventually) directly into a PDF file.

Right now, I'm first focusing on generating a website, and then, later, converting that website somehow into a PDF
by using the (hopefully) already installed web engine so that the program stays minimal and doesn't bundle a whole
web runtime with itself.