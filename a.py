a = True
def is_digit(val: str):
    return val.isdigit()

def tokenize(t: str):
    trim = t.replace(" ", "")
    tok = []
    for i in range(0, len(trim)):
        match trim[i]:
            case '(' :
                tok.append({'id': 'LPARENT', 'value': trim[i]})
            case ')' :
                tok.append({'id': 'RPARENT', 'value': trim[i]})
            case '+' :
                tok.append({'id': 'plus', 'value': trim[i]})
            case '-':
                tok.append({'id': 'minus', 'value': trim[i]})
            case '*':
                tok.append({'id': 'times', 'value': trim[i]})
            case '/':
                tok.append({'id': 'div', 'value': trim[i]})
            case _:
                if is_digit(trim[i]):
                    tok.append({"id": "number", "value": int(trim[i])})
                else:
                    tok.append({"id": "unknow", "value": trim[i]})
                
    return tok
                
                
while a:
    result = input("> ")
    print(tokenize(result))
