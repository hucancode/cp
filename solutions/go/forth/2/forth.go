package forth

import (
	"fmt"
	"strings"
)

func evaluateMath(stack *[]int, cmd string) error {
	n := len(*stack)
	if n < 2 {
		return fmt.Errorf("not enough element in the stack %v", stack)
	}
	a := (*stack)[n-2]
	b := (*stack)[n-1]
	*stack = (*stack)[:n-2]
	switch cmd {
	case "+":
		*stack = append(*stack, a+b)
	case "-":
		*stack = append(*stack, a-b)
	case "*":
		*stack = append(*stack, a*b)
	case "/":
		if b == 0 {
			return fmt.Errorf("division by zero")
		}
		*stack = append(*stack, a/b)
	}
	return nil
}

func preprocess(ops map[string][]string, program []string) []string {
	ret := make([]string, 0)
	for _, cmd := range program {
		cmd = strings.ToLower(cmd)
		if _, ok := ops[cmd]; ok {
			ret = append(ret, ops[cmd]...)
		} else {
			ret = append(ret, cmd)
		}
	}
	return ret
}

func evaluate(stack *[]int, program []string) error {
	fmt.Printf("evaluating %v\n", program)
	for _, cmd := range program {
		n := len(*stack)
		switch strings.ToLower(cmd) {
		case "+", "-", "*", "/":
			err := evaluateMath(stack, cmd)
			if err != nil {
				return err
			}
		case "dup":
			if n == 0 {
				return fmt.Errorf("empty stack")
			}
			*stack = append(*stack, (*stack)[n-1])
		case "drop":
			if n == 0 {
				return fmt.Errorf("empty stack")
			}
			*stack = (*stack)[:n-1]
		case "swap":
			if n < 2 {
				return fmt.Errorf("not enough element in the stack %v", stack)
			}
			(*stack)[n-1], (*stack)[n-2] = (*stack)[n-2], (*stack)[n-1]
		case "over":
			if n < 2 {
				return fmt.Errorf("not enough element in the stack %v", stack)
			}
			*stack = append(*stack, (*stack)[n-2])
			continue
		default:
			var i int
			_, err := fmt.Sscanf(cmd, "%d", &i)
			if err != nil {
				return fmt.Errorf("invalid command %s", cmd)
			}
			*stack = append(*stack, i)
		}
	}
	return nil
}

func isValidIdentifier(name string) bool {
	if len(name) == 0 {
		return false
	}
	var i int
	_, err := fmt.Sscanf(name, "%d", &i)
	return err != nil
}

func Forth(input []string) ([]int, error) {
	stack := make([]int, 0)
	ops := make(map[string][]string)
	for _, program := range input {
		cmds := strings.Fields(program)
		n := len(cmds)
		if n == 0 {
			continue
		}
		if cmds[0] == ":" && cmds[n-1] == ";" {
			cmds[1] = strings.ToLower(cmds[1])
			if !isValidIdentifier(cmds[1]) {
				return nil, fmt.Errorf("invalid identifier %s", cmds[1])
			}
			ops[cmds[1]] = preprocess(ops, cmds[2:n-1])
			continue
		} else {
			cmds := preprocess(ops, cmds)
			err := evaluate(&stack, cmds)
			if err != nil {
				return nil, err
			}
		}
	}
	return stack, nil
}
