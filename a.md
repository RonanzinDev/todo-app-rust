## [{'id': 'LPARENT', 'value': '('}, {'id': 'number', 'value': 1}, {'id': 'plus', 'value': '+'}, {'id': 'number', 'value': 1}, {'id': 'RPARENT', 'value': ')'}] 


1 + 1 -> Precedenc order = None -> expr = 1 + 1 -> {"flag": 0, operator: "-", "values": [1,1]}
( 1 + 1 ) -> 'Precedenc order' ( ) -> expr = 1 + 1 -> {"flag": 1, "operator": "+", "values": [1,1]} 

def f(expr's):
    match expr:
        case []:
            return
    


Expr = {"left": f(expr), "operator", "rigth" : value}



3 + (1 + 2) -> [3, [1 , 2]]
```python
def calc(expr):
    result = 0
    match operator:
        case '+':
            result += val
        case '-':
            result -= val
        case '*':
            result *= val
        case '/':
            result /= val

    return 
    