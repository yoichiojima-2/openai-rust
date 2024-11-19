#!/bin/zsh

introductory_message="i will give you a return  tree . && find . -name \"*.py\" -exec cat {} \; explain what this project is doing"

(echo ${introductory_message} && tree . && find . -name "*.py" -exec cat {} \;) | xargs -0 openai-rust ask