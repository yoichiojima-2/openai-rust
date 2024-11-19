#!/bin/zsh

# simply pack all the information and pass openai-rust ask for general questions
introductory_message="i will give you a output of tree . && find . -name \"*.py\" -exec cat {} \; explain what the project is doing"
(echo ${introductory_message} && tree . && find . -name "*.py" -exec cat {} \;) | xargs -0 openai-rust ask