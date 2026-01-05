package main

import (
	"errors"
	"fmt"
	"os"
	"strings"
)

type Token struct {
	Type  TokenType
	Value string
}

var keywordMap map[string]Token = map[string]Token{
	"pub": {
		Type:  TOKEN_TYPE_KEYWORD,
		Value: "pub",
	},
	"fn": {
		Type:  TOKEN_TYPE_KEYWORD,
		Value: "fn",
	},
	"return": {
		Type:  TOKEN_TYPE_KEYWORD,
		Value: "return",
	},
	"let": {
		Type:  TOKEN_TYPE_KEYWORD,
		Value: "let",
	},
	"mut": {
		Type:  TOKEN_TYPE_KEYWORD,
		Value: "mut",
	},
	"u32": {
		Type:  TOKEN_TYPE_DATA_TYPE,
		Value: "u32",
	},
	"i64": {
		Type:  TOKEN_TYPE_DATA_TYPE,
		Value: "i64",
	},
}
var tokens []Token = make([]Token, 0)

func tokenize(lexerTokens []string) error {
	skipIterationCount := 0

	for idx, token := range lexerTokens {
		if skipIterationCount > 0 {
			skipIterationCount -= 1
			continue
		}

		switch token {
		case ";":
			tokens = append(tokens, Token{
				Type:  TOKEN_TYPE_SEMICOLON,
				Value: ";",
			})
			continue
		case ":":
			tokens = append(tokens, Token{
				Type:  TOKEN_TYPE_COLON,
				Value: ":",
			})
			continue
		case ",":
			tokens = append(tokens, Token{
				Type:  TOKEN_TYPE_COMMA,
				Value: ",",
			})
			continue
		case ".":
			tokens = append(tokens, Token{
				Type:  TOKEN_TYPE_COMMA,
				Value: ".",
			})
			continue
		case "(":
			tokens = append(tokens, Token{
				Type:  TOKEN_TYPE_OPEN_PAREN,
				Value: "(",
			})
			continue
		case ")":
			tokens = append(tokens, Token{
				Type:  TOKEN_TYPE_CLOSE_PAREN,
				Value: ")",
			})
			continue
		case "{":
			tokens = append(tokens, Token{
				Type:  TOKEN_TYPE_OPEN_CURLY,
				Value: "{",
			})
			continue
		case "}":
			tokens = append(tokens, Token{
				Type:  TOKEN_TYPE_CLOSE_CURLY,
				Value: "}",
			})
			continue
		case "[":
			tokens = append(tokens, Token{
				Type:  TOKEN_TYPE_OPEN_BRACKET,
				Value: "[",
			})
			continue
		case "]":
			tokens = append(tokens, Token{
				Type:  TOKEN_TYPE_CLOSE_BRACKET,
				Value: "]",
			})
			continue
		case "-":
			if len(lexerTokens)-1 > idx {
				if lexerTokens[idx+1] == ">" || lexerTokens[idx+1] == "-" || lexerTokens[idx+1] == "=" {
					tokens = append(tokens, Token{
						Type:  TOKEN_TYPE_OPERATOR,
						Value: fmt.Sprintf("-%s", lexerTokens[idx+1]),
					})
					skipIterationCount = 1
					continue
				} else if isNumber(rune(lexerTokens[idx+1][0])) {
					tokens = append(tokens, Token{
						Type:  TOKEN_TYPE_NUMBER,
						Value: fmt.Sprintf("-%s", lexerTokens[idx+1]),
					})
					skipIterationCount = 1
					continue
				}
			}

			tokens = append(tokens, Token{
				Type:  TOKEN_TYPE_OPERATOR,
				Value: "-",
			})
			continue
		case "+":
			if len(lexerTokens)-1 > idx {
				if lexerTokens[idx+1] == "+" || lexerTokens[idx+1] == "=" {
					tokens = append(tokens, Token{
						Type:  TOKEN_TYPE_OPERATOR,
						Value: fmt.Sprintf("+%s", lexerTokens[idx+1]),
					})
					skipIterationCount = 1
					continue
				}
			}

			tokens = append(tokens, Token{
				Type:  TOKEN_TYPE_OPERATOR,
				Value: "+",
			})
			continue
		case "*":
			if len(lexerTokens)-1 > idx {
				if lexerTokens[idx+1] == "*" || lexerTokens[idx+1] == "=" {
					tokens = append(tokens, Token{
						Type:  TOKEN_TYPE_OPERATOR,
						Value: fmt.Sprintf("*%s", lexerTokens[idx+1]),
					})
					skipIterationCount = 1
					continue
				}
			}

			tokens = append(tokens, Token{
				Type:  TOKEN_TYPE_OPERATOR,
				Value: "*",
			})
			continue
		case "/":
			if len(lexerTokens)-1 > idx {
				if lexerTokens[idx+1] == "/" || lexerTokens[idx+1] == "=" {
					tokens = append(tokens, Token{
						Type:  TOKEN_TYPE_OPERATOR,
						Value: fmt.Sprintf("/%s", lexerTokens[idx+1]),
					})
					skipIterationCount = 1
					continue
				}
			}

			tokens = append(tokens, Token{
				Type:  TOKEN_TYPE_OPERATOR,
				Value: "/",
			})
			continue
		case "%":
			if len(lexerTokens)-1 > idx {
				if lexerTokens[idx+1] == "=" {
					tokens = append(tokens, Token{
						Type:  TOKEN_TYPE_OPERATOR,
						Value: "%=",
					})
					skipIterationCount = 1
					continue
				}
			}

			tokens = append(tokens, Token{
				Type:  TOKEN_TYPE_OPERATOR,
				Value: "%",
			})
			continue
		case "=":
			if len(lexerTokens)-1 > idx {
				if lexerTokens[idx+1] == "=" {
					tokens = append(tokens, Token{
						Type:  TOKEN_TYPE_OPERATOR,
						Value: "==",
					})
					skipIterationCount = 1
					continue
				}
			}

			tokens = append(tokens, Token{
				Type:  TOKEN_TYPE_OPERATOR,
				Value: "=",
			})
			continue
		case "!":
			if len(lexerTokens)-1 > idx {
				if lexerTokens[idx+1] == "=" {
					tokens = append(tokens, Token{
						Type:  TOKEN_TYPE_OPERATOR,
						Value: "!=",
					})
					skipIterationCount = 1
					continue
				}
			}

			tokens = append(tokens, Token{
				Type:  TOKEN_TYPE_OPERATOR,
				Value: "!",
			})
			continue
		case "&":
			if len(lexerTokens)-1 > idx {
				if lexerTokens[idx+1] == "&" || lexerTokens[idx+1] == "=" {
					tokens = append(tokens, Token{
						Type:  TOKEN_TYPE_OPERATOR,
						Value: fmt.Sprintf("&%s", lexerTokens[idx+1]),
					})
					skipIterationCount = 1
					continue
				}
			}

			tokens = append(tokens, Token{
				Type:  TOKEN_TYPE_OPERATOR,
				Value: "&",
			})
			continue
		case "|":
			if len(lexerTokens)-1 > idx {
				if lexerTokens[idx+1] == "|" || lexerTokens[idx+1] == "=" {
					tokens = append(tokens, Token{
						Type:  TOKEN_TYPE_OPERATOR,
						Value: fmt.Sprintf("|%s", lexerTokens[idx+1]),
					})
					skipIterationCount = 1
					continue
				}
			}

			tokens = append(tokens, Token{
				Type:  TOKEN_TYPE_OPERATOR,
				Value: "|",
			})
			continue
		case "^":
			if len(lexerTokens)-1 > idx && lexerTokens[idx+1] == "=" {
				tokens = append(tokens, Token{
					Type:  TOKEN_TYPE_OPERATOR_BITWISE_XOR_ASSIGN,
					Value: "^=",
				})
				skipIterationCount = 1
				continue
			}

			tokens = append(tokens, Token{
				Type:  TOKEN_TYPE_OPERATOR_BITWISE_XOR,
				Value: "^",
			})
			continue
		case "<":
			if len(lexerTokens)-1 > idx && lexerTokens[idx+1] == "<" {
				if len(lexerTokens)-2 > idx && lexerTokens[idx+2] == "=" {
					tokens = append(tokens, Token{
						Type:  TOKEN_TYPE_OPERATOR_BITWISE_SHIFT_LEFT_ASSIGN,
						Value: "<<=",
					})
					skipIterationCount = 2
					continue
				}

				tokens = append(tokens, Token{
					Type:  TOKEN_TYPE_OPERATOR_BITWISE_SHIFT_LEFT,
					Value: "<<",
				})

				skipIterationCount = 1
				continue
			}

			tokens = append(tokens, Token{
				Type:  TOKEN_TYPE_OPEN_ANGLE,
				Value: "<",
			})
			continue
		case ">":
			if len(lexerTokens)-1 > idx && lexerTokens[idx+1] == ">" {
				if len(lexerTokens)-2 > idx && lexerTokens[idx+2] == "=" {
					tokens = append(tokens, Token{
						Type:  TOKEN_TYPE_OPERATOR_BITWISE_SHIFT_RIGHT_ASSIGN,
						Value: ">>=",
					})
					skipIterationCount = 2
					continue
				}

				tokens = append(tokens, Token{
					Type:  TOKEN_TYPE_OPERATOR_BITWISE_SHIFT_RIGHT,
					Value: ">>",
				})

				skipIterationCount = 1
				continue
			}

			tokens = append(tokens, Token{
				Type:  TOKEN_TYPE_CLOSE_ANGLE,
				Value: ">",
			})
			continue
		}

		value, hasKey := keywordMap[token]
		if hasKey {
			tokens = append(tokens, value)
			continue
		}

		if isNumber(rune(token[0])) {
			allNum := true

			for _, c := range token[1:] {
				if isNumber(c) || c == '_' {
					continue
				}

				allNum = false
				break
			}

			if allNum {
				var tokenType TokenType
				numPeriodCount := strings.Count(token, ".")
				switch numPeriodCount {
				case 0:
					tokenType = TOKEN_TYPE_NUMBER_INT
				case 1:
					tokenType = TOKEN_TYPE_NUMBER_FLOAT
				default:
					return errors.New("Malformed number token.")
				}

				cleanNum := strings.ReplaceAll(token, "_", "")
				tokens = append(tokens, Token{
					Type:  tokenType,
					Value: cleanNum,
				})
				continue
			}
		}

		if isLetter(rune(token[0])) || token[0] == '_' {
			allIdent := true
			for _, c := range token {
				if isValidIdentifier(c) {
					continue
				}

				allIdent = false
				break
			}

			if allIdent {
				tokens = append(tokens, Token{
					Type:  TOKEN_TYPE_IDENTIFIER,
					Value: token,
				})
				continue
			}
		}

		fmt.Printf("Reached the of the parsing loop with an unhandled token: %s\n", token)
		os.Exit(1)
	}

	for idx, tok := range tokens {
		fmt.Printf("%d: %s, %s\n", idx, tok.Type.String(), tok.Value)
	}

	return nil
}
