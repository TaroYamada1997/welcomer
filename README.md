# Welcomer

## Overview

This simple CLI program generates a greeting message in various languages based on the user's name and chosen language.

## Features

- Greet in 10 different languages
- Use command-line arguments to specify name and language
- Fallback to English if the specified language is not supported

## Usage

The program accepts two arguments:

- `-n, --name <NAME>`: The name of the person to greet
- `-l, --language <LANGUAGE>`: The language code for the greeting

## Supported Languages

- en: English
- ja: Japanese
- zh: Chinese
- es: Spanish
- fr: French
- de: German
- it: Italian
- ru: Russian
- ko: Korean
- zu: Zulu

If an unsupported language code is specified, the program will default to an English greeting.

## Dependencies

This program uses the `clap` crate for parsing command-line arguments.

## Version

Current version: 0.1.0