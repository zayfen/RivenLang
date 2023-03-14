# zlang
a language implemented in rust

Learning resource: [TeenytinyCompiler](https://austinhenley.com/blog/teenytinycompiler2.html)

## grammar

```
<program>   ::= MAIN { <statement-list> }

<statement-list>    ::= <empty>
                        | <statement> <statement-list>

<statement>   ::=   <assign-statement>
                    | <call-expression> ;
                    | <return-statement>
                    | <function-statement>
                    | <if-statement>

<function-statement>  ::= function <identifier> ( <identifier-list> ) { <statement-list> }

<if-statement>  ::= if (<expression>) { <statement-list> }

<return-statement>  ::= return <expression> ;

<assign-statement>     ::= <identifier> = <expression> ;

<call-statement>    ::= <call-expression> ;

<expression-list>   ::= <expression>
                      | <expresison> , <expression-list>

// identifier (  <=> identifier [*/+-]
<expression>    ::= <arithmetic-expression>
                  | <call-expression>

<call-expression>       ::= <identifier> ()
                          | <identifier> ( <expression-list> )

<arithmetic-expression> ::= <term>
                        | <term> + <arithmetic-expression>
                        | <term> - <arithmetic-expression>

<term>      ::= <factor>
            | <factor> * <term>
            | <factor> / <term>

// "hello", 123, userName
<factor>    ::= <primary> | <identifier>

// "hello", 123
<primary>   ::= <string> | <number>

// userName, Age
<identifier-list>       ::= <identifier>
                        | <identifier> , <identifier-list>

// useName
<identifier>    ::= [a-z][A-Z]

<string>    ::= '<charactoers>'
            | ''
<characters>    ::= <character>
                | <character> <characters>


<number>    ::= <integer>
<integer>   ::= <decimal digit>
            | <decimal digit> <integer>
<decimal digit>     ::= 0|1|2|3|4|5|6|7|8|9
```
