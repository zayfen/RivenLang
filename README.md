# RivenLang
A simple language for children implemented in rust.
We can teach children basic math knowledge and basic coding knowledge in RivenLang.

 Why `RivenLang`? 
 Because my cute daughter named "芮文" in Chinese, its pronunciation very like the word `riven` in English. So, as a matter of course, I named this programming language `riven`.
 
 

 ## Example
> fib.riven
``` rust
// Code example

program {
    fn fib(n) {
      if (n = 0) {
        return 0;
      }

      if (n = 1) {
        return 1;
      }

      // calculate fib(n-1) and fib(n-2)
      fib_n_1 = fib(n-1);
      fib_n_2 = fib(n-2);
      fib_n = fib(n-1) + fib(n-2); // fib_n_1 + fib_n_2;
      printf("fib(%d): %d\n", n, fib_n);
      return fib_n;
    }

    fn main() {
      fib(5);
      return 0;
    }
  }

```

Output:

``` text
❯ ./b.out
fib(2): 1
fib(2): 1
fib(3): 2
fib(2): 1
fib(2): 1
fib(2): 1
fib(3): 2
fib(2): 1
fib(4): 3
fib(2): 1
fib(2): 1
fib(3): 2
fib(2): 1
fib(2): 1
fib(3): 2
fib(2): 1
fib(2): 1
fib(2): 1
fib(3): 2
fib(2): 1
fib(4): 3
fib(2): 1
fib(2): 1
fib(3): 2
fib(5): 5

```

How to compile?
`riven <your source file path>`, e.g.: `riven fib.riven`

If build successfully, there is a file named `b.out` in the directory source file stays, 
Now, we can run `b.out` in terminal just like the way your run `ls cd`. 

## grammar

``` text


<program>   ::= MAIN { <statement-list> }

<statement-list>    ::= <empty>
                        | <statement> <statement-list>

<statement>   ::=   <assign-statement>
                    | <call-expression> ;
                    | <return-statement>
                    | <function-statement>
                    | <if-statement>

<function-statement>  ::= fn <identifier> ( <identifier-list> ) { <statement-list> }

<if-statement>  ::= if (<logic-expression>) { <statement-list> }

<return-statement>  ::= return <expression> ;

<assign-statement>     ::= <identifier> = <expression> ;

<call-statement>    ::= <call-expression> ;

<logic-expression> ::= <compare-expression>
                    |  not (<logic-expression>)
                    |  and (<logic-expression> , <logic-expression>)
                    |  or (<logic-expression> , <logic-expression>)



<compare-expression>   ::= <expression>
                      | <expression> = <expression>
                      | <expression> > <expression>
                      | <expression> < <expression>  

<expression-list>   ::= <expression>
                      | <expresison> , <expression-list>

// identifier (  <=> identifier [*/+-]
<expression>    ::= <component-arithmetic-expression>

<call-expression>       ::= <identifier> ()
                          | <identifier> ( <expression-list> )

<component-arithmetic-expression> ::= <component-term>
                                    | <component-term> + <component-arithmetic-expression>
                                    | <component-term> - <component-arithmetic-expression>

<compoennt-term>  ::= <component-factor>
                    | <component-factor> * <component-term>
                    | <component-factor> / <component-term>

// like factor
<component-factor>  ::= <arithmetic-expression>
                      | ( <component-factor> ) 

<arithmetic-expression> ::= <term>
                        | <term> + <arithmetic-expression>
                        | <term> - <arithmetic-expression>

<term>      ::= <factor>
            | <factor> * <term>
            | <factor> / <term>

<factor>    ::= <primary> | <identifier> | <call-expression>

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

