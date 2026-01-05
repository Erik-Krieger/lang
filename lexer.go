package main

import (
	"fmt"
	"os"
)

type TokenType uint64

const (
	TOKEN_TYPE_NONE       TokenType = (1 << 0)
	TOKEN_TYPE_NUMBER     TokenType = (1 << 1)
	TOKEN_TYPE_IDENTIFIER TokenType = (1 << 2)
	TOKEN_TYPE_DASH       TokenType = (1 << 3)
)

type lexerContainer struct {
	tokens       []string
	wordStart    int
	numberStart  int
	currentToken TokenType
}

func parseFile(fileName string) ([]string, error) {
	data, err := os.ReadFile(fileName)
	if err != nil {
		fmt.Println("Error reading the file.")
		return nil, err
	}

	text := string(data)

	pk := &lexerContainer{
		tokens:       make([]string, 0),
		wordStart:    -1,
		numberStart:  -1,
		currentToken: TOKEN_TYPE_NONE,
	}

	for idx, c := range text {
		processToken(pk, idx, c, text)
	}

	for idx, token := range pk.tokens {
		fmt.Printf("%d: %s\n", idx, token)
	}

	return pk.tokens, nil
}

func processToken(pk *lexerContainer, idx int, c rune, text string) {
	switch pk.currentToken {
	case TOKEN_TYPE_NONE:
		if isNumber(c) {
			pk.currentToken = TOKEN_TYPE_NUMBER
			pk.numberStart = idx
			return
		} else if isLetter(c) || c == '_' {
			pk.currentToken = TOKEN_TYPE_IDENTIFIER
			pk.wordStart = idx
			return
		} else if isBracket(c) {
			pk.tokens = append(pk.tokens, string(c))
			return
		} else if isSpecialCharacter(c) {
			pk.tokens = append(pk.tokens, string(c))
		}
	case TOKEN_TYPE_NUMBER:
		if isNumber(c) || c == '_' {
			return
		} else {
			pk.tokens = append(pk.tokens, text[pk.numberStart:idx])
			pk.currentToken = TOKEN_TYPE_NONE
			pk.numberStart = -1
			processToken(pk, idx, c, text)
			return
		}
	case TOKEN_TYPE_IDENTIFIER:
		if isValidIdentifier(c) {
			return
		} else {
			pk.tokens = append(pk.tokens, text[pk.wordStart:idx])
			pk.currentToken = TOKEN_TYPE_NONE
			pk.wordStart = -1
			processToken(pk, idx, c, text)
			return
		}
	}
}

func isValidIdentifier(c rune) bool {
	return isLetter(c) || isNumber(c) || c == '_'
}

func isLetter(c rune) bool {
	return (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z')
}

func isNumber(c rune) bool {
	return c >= '0' && c <= '9'
}

func isBracket(c rune) bool {
	return c == '(' || c == ')' || c == '[' || c == ']' || c == '{' || c == '}' || c == '<' || c == '>'
}

func isSpecialCharacter(c rune) bool {
	return isOperator(c) || c == '=' || c == '!' || c == ';' || c == ':' || c == ',' || c == '.'
}

func isOperator(c rune) bool {
	return c == '+' || c == '-' || c == '*' || c == '/' || c == '%'
}
