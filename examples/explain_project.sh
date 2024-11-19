#!/bin/zsh

(echo "i will give you a return  tree . && find . -name \"*.py\" -exec cat {} \; explain what this project is doing" && tree . && find . -name "*.py" -exec cat {} \;) | xargs -0 openai-rust ask