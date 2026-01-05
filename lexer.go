package main

import (
	"fmt"
	"os"
)

type TokenType uint64

func (t TokenType) String() string {
	switch t {
	case TOKEN_TYPE_NONE:
		return "TOKEN_TYPE_NONE"
	case TOKEN_TYPE_NUMBER:
		return "TOKEN_TYPE_NUMBER"
	case TOKEN_TYPE_NUMBER_INT:
		return "TOKEN_TYPE_NUMBER_INT"
	case TOKEN_TYPE_NUMBER_FLOAT:
		return "TOKEN_TYPE_NUMBER_FLOAT"
	case TOKEN_TYPE_IDENTIFIER:
		return "TOKEN_TYPE_IDENTIFIER"
	case TOKEN_TYPE_OPERATOR:
		return "TOKEN_TYPE_OPERATOR"
	case TOKEN_TYPE_KEYWORD:
		return "TOKEN_TYPE_KEYWORD"
	case TOKEN_TYPE_DATA_TYPE:
		return "TOKEN_TYPE_DATA_TYPE"
	case TOKEN_TYPE_SEMICOLON:
		return "TOKEN_TYPE_SEMICOLON"
	default:
		return "Error: Unknown Type"
	}
}

const (
	TOKEN_TYPE_NONE                                TokenType = 0
	TOKEN_TYPE_NUMBER                              TokenType = 100
	TOKEN_TYPE_NUMBER_INT                          TokenType = 101
	TOKEN_TYPE_NUMBER_FLOAT                        TokenType = 102
	TOKEN_TYPE_IDENTIFIER                          TokenType = 200
	TOKEN_TYPE_KEYWORD                             TokenType = 300
	TOKEN_TYPE_OPERATOR                            TokenType = 400
	TOKEN_TYPE_OPERATOR_UNARY                      TokenType = 401
	TOKEN_TYPE_OPERATOR_LOGICAL_OR                 TokenType = 410
	TOKEN_TYPE_OPERATOR_LOGICAL_AND                TokenType = 411
	TOKEN_TYPE_OPERATOR_LOGICAL_NOT                TokenType = 412
	TOKEN_TYPE_OPERATOR_BITWISE_SHIFT_LEFT         TokenType = 450
	TOKEN_TYPE_OPERATOR_BITWISE_SHIFT_LEFT_ASSIGN  TokenType = 451
	TOKEN_TYPE_OPERATOR_BITWISE_SHIFT_RIGHT        TokenType = 452
	TOKEN_TYPE_OPERATOR_BITWISE_SHIFT_RIGHT_ASSIGN TokenType = 453
	TOKEN_TYPE_OPERATOR_BITWISE_OR                 TokenType = 460
	TOKEN_TYPE_OPERATOR_BITWISE_OR_ASSIGN          TokenType = 461
	TOKEN_TYPE_OPERATOR_BITWISE_AND                TokenType = 462
	TOKEN_TYPE_OPERATOR_BITWISE_AND_ASSIGN         TokenType = 463
	TOKEN_TYPE_OPERATOR_BITWISE_XOR                TokenType = 464
	TOKEN_TYPE_OPERATOR_BITWISE_XOR_ASSIGN         TokenType = 464
	TOKEN_TYPE_DATA_TYPE                           TokenType = 500
	TOKEN_TYPE_SEMICOLON                           TokenType = 600
	TOKEN_TYPE_COLON                               TokenType = 601
	TOKEN_TYPE_COMMA                               TokenType = 602
	TOKEN_TYPE_PERIOD                              TokenType = 603
	TOKEN_TYPE_OPEN_PAREN                          TokenType = 1000
	TOKEN_TYPE_CLOSE_PAREN                         TokenType = 1001
	TOKEN_TYPE_OPEN_BRACKET                        TokenType = 1100
	TOKEN_TYPE_CLOSE_BRACKET                       TokenType = 1101
	TOKEN_TYPE_OPEN_CURLY                          TokenType = 1200
	TOKEN_TYPE_CLOSE_CURLY                         TokenType = 1201
	TOKEN_TYPE_OPEN_ANGLE                          TokenType = 1300
	TOKEN_TYPE_CLOSE_ANGLE                         TokenType = 1301
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
	return isOperator(c) || c == '=' || c == '!' || c == ';' || c == ':' || c == ',' || c == '.' || c == '^' || c == '&' || c == '|' || c == '~'
}

func isOperator(c rune) bool {
	return c == '+' || c == '-' || c == '*' || c == '/' || c == '%'
}
