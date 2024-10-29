# Skibidi-lang
## Hur kör man den?
Kör `cargo run` för att se alla commandon. Nedan finns några exempel.

### Köra en fil
`cargo run run program.skl`

### Se AST
`cargo run ast program.skl`

## BNF
`https://bnfplayground.pauliankline.com/?bnf=%3Cprogram%3E%20%3A%3A%3D%20%3Cstatement%3E%2B%0A%3Cstatement%3E%20%3A%3A%3D%20(%3Cret%3E%20%7C%20%3Cif%3E%20%7C%20%3Celse%3E%20%7C%20%3Cwhile%3E%20%7C%20%3Cfunction%3E%20%7C%20%3Cdefinition%3E%20%7C%20%3Cassignment%3E%20%7C%20%3Cstmt_call%3E)%20%3Cs%3E%0A%3Cs%3E%20%3A%3A%3D%20(%22%20%22%20%7C%20%22%5Cn%22)*%0A%3Cnumber%3E%20%3A%3A%3D%20(%220%22%20%7C%20%20%5B1-9%5D%20%5B0-9%5D*)%0A%3Cvar%3E%20%3A%3A%3D%20%5Ba-z%5D%2B%0A%0A%3Cret%3E%20%3A%3A%3D%20%22sigma%22%20%3Cs%3E%20%3Cexpr%3E%20%3Cs%3E%20%22%7C%22%0A%3Cif%3E%20%3A%3A%3D%20%22sus%22%20%3Cs%3E%20%3Cexpr%3E%20%3Cs%3E%20%22%3E%3E%22%20%3Cs%3E%20%3Cstatement%3E*%20%3Cs%3E%20%22%3C%3C%22%0A%3Celse%3E%20%3A%3A%3D%20%22sussy%22%20%3Cs%3E%20%22%3E%3E%22%20%3Cs%3E%20%3Cstatement%3E*%20%3Cs%3E%20%22%3C%3C%22%0A%3Cwhile%3E%20%3A%3A%3D%20%22edge%22%20%3Cs%3E%20%3Cexpr%3E%20%3Cs%3E%20%22%3E%3E%22%20%3Cs%3E%20%3Cstatement%3E*%20%3Cs%3E%20%22%3C%3C%22%0A%3Cfunction%3E%20%3A%3A%3D%20%22skibidi%22%20%3Cs%3E%20%3Cvar%3E%20%22(%22%20(%3Cvar%3E%20%3Cs%3E%20%22%2C%22%3F%20%3Cs%3E)*%20%20%22)%22%20%3Cs%3E%20%22%3E%3E%22%20%3Cs%3E%20%3Cstatement%3E*%20%3Cs%3E%20%22%3C%3C%22%0A%3Cdefinition%3E%20%3A%3A%3D%20%22looksmaxxing%22%20%3Cs%3E%20%3Cassignment%3E%0A%3Cassignment%3E%20%3A%3A%3D%20%3Cvar%3E%20%3Cs%3E%20%22%3D%22%20%3Cs%3E%20%3Cexpr%3E%20%3Cs%3E%20%22%7C%22%0A%3Cstmt_call%3E%20%3A%3A%3D%20%3Ccall%3E%20%3Cs%3E%20%22%7C%22%0A%3Ccall%3E%20%3A%3A%3D%20%3Cvar%3E%20%22(%22%20%3Cs%3E%20%3Cexpr%3E%20%3Cs%3E%20%22)%22%20%0A%0A%3Cexpr%3E%20%3A%3A%3D%20%3Cadd_sub_expr%3E%0A%3Cadd_sub_expr%3E%20%3A%3A%3D%20%3Cmul_div_expr%3E%20(%20%3Cs%3E%20(%22rizz%22%20%7C%20%22fanumtax%22)%20%3Cs%3E%20%3Cmul_div_expr%3E)%3F%0A%3Cmul_div_expr%3E%20%3A%3A%3D%20%3Cequals_expr%3E%20(%20%3Cs%3E%20(%22gyatt%22%20%7C%20%22mog%22)%20%3Cs%3E%20%3Cequals_expr%3E)%3F%0A%3Cequals_expr%3E%20%3A%3A%3D%20%3Cprimary_expr%3E%20(%20%3Cs%3E%20(%22%3D%3D%22%20%7C%20%22!%3D%22)%20%3Cs%3E%20%3Cprimary_expr%3E)%3F%0A%3Cprimary_expr%3E%20%3A%3A%3D%20(%3Cnumber%3E%20%7C%20%3Ccall%3E%20%7C%20%3Cvar%3E%20%7C%20%22(%22%20%3Cs%3E%20%3Cexpr%3E%20%3Cs%3E%20%22)%22)%0A%0A&name=Skibidi-lang`

## Syntax
Nedan defineras samma simpla program i python och skibidi-lang.

Python:
```py
def fib(n):
    if n == 0:
        return 0
    elif n == 1:
        return 1
    else:
        return lol(n-1) + lol(n-2)

i = 0
while i != 10:
    a = fib(i)
    print(a)

    i = i + 1
```

Skibidi-lang:
```skl
looksmaxxing lol = 123|

skibidi fib(n) >>
    sus n == 0 >>
        sigma 0|
    <<
    sus n == 1 >>
        sigma 1|
    << 
    sussy >>
        sigma fib(n fanumtax 1) rizz fib(n fanumtax 2)|
    << 
<<

looksmaxxing i = 0|
edge (i != 10) >>
    looksmaxxing a = fib(i)|
    print(a)|

    i = i rizz 1|
<<
```
