# Changelog file 


Well i started by building the "scanner" and then, I figured out something i need a "logger" so i 
built one, then i realized something else, I need "language logger" also, so i started by defninng 
the enums for the errors and then i built the language logger.

Right after this i begun to optimize the scanner so it would be more efficient, and cover most of
the cases and parse any code to tokens, i figured out that i need some test cases so it will be
tested 100% so any further optimization will be checked by the tests.

Right now i am working on the parser, i am still reading that chapter of a book, i spent the last
night reading in WIKI blogs about abstract machines and some other things like the deremensitic and
non-dereministic machienes like the turing machine and pushdown automatan, yahh a bit tough but i
can handle it i think yahh :skull:.

---

I figured out there's a lot to think about like the Chomsky Hirarcy and how languages are built in
the first place, it's like a big puzzle, but i am still working on it.

So a lexical grammar(Scanner) look at things in a different way, like the alphabet is a char and the 
strings are tokens or lexeme and the syntactic grammar(Parser) is how you build the syntax of the 
language,  it looks are the alphabet as tokens and the strings are the syntax of the 
language(e.g. expression).

So now at this point the scanner just let me know what the tokens are and the parser will build the 
syntax of the language from the tokens, because i can not list all the cases and track them to the 
scanner this will be overwholeming and will take forever. so i will set a finitie of set of rules, 
and i will use them to generate strings that are in the grammar, The strings generarted this way 
will be called "Derivations" because each derived from the rules of the grammar, each on of these 
rules are so-called "production", each production has a head which is the name of the rule, and a 
body which the rule extends to.

---

There are two types of symbols used in the productions, terminals and non-terminals, terminals are
literal values from the language, and they do not expand further (e.g. if, then, else), non-terminals
they reference to other rules, and they can expand further (e.g. expression, statement, etc).

Why they're called terminal? Simply because they are the leaves of the grammar tree, they are the 
end of the derivation, think of them like the final characters or tokens in the output, and the 
non-terminals are the nodes of the grammar tree, they are the internal nodes of the derivation, they
can expand into sequences of terminals and non-terminals.

Think about it like a tree, the leaves are the terminals, the internal nodes are the non-terminals,
and the root is the start symbol of the grammar, it is the entry point of the derivation.

---

A syntactic grammar defines a finite set of rules that, when applied recursively, form a tree 
structure (parse tree) which generates valid strings (text/code) in a language.

<div style="text-align: center;">

            Expression
           /    |     \
      Number   +   Expression
        |           |
        1        Number
                     |
                     2

</div>


So to make a grammar to the `lox` language i will start by some expression that's gonna give me a
start to make this run and work, and i will add on more rules as i go on.

```
I tried to come up with something clean. Each rule is a name, followed by an
arrow (→), followed by a sequence of symbols, and finally ending with a
semicolon (;)

Terminals are quoted strings, and nonterminals are lowercase
words and capitalized words are terminals that are a single
lexeme whose text representation may vary.

- "|" means OR, so we will pipe values to reduce the redundancy of the grammar.
- "()" means grouping, to select a group of rules with in the middle of a production.
- "*" to allow symbols of groups to be repeated zero or more times.
- "+" means that the preceding production have to appear at least once.
- "?" means that the preceding production have to appear zero or more times.  
```

- Letrials : Numbers, Strings, Booleans, Null
- Unary operators : +, -, !
- Binary operators : +, -, *, /, %, ==, !=, >, <, >=, <=
- Grouping : ( )

That will give me the the power to build such a block like this:

<div style="text-align: center;">

1 - ( 2 * 3 ) < 4 = false

</div>

So the grammar for this will be 

<div style="text-align: center;">

    expression -> literal | unary | binary | grouping;
    literal -> NUMBER | STRING | "true" | "false" | "nil";
    grouping -> ( expression );
    unary -> ( "-" | "+" ) expression;
    binary -> expression operator expression;
    operator -> "+" | "-" | "*" | "/" | "%" | "==" | "!=" | ">" | "<" | ">=" | "<=";

</div>

--- 

Right now i have the grammar i need to implement the syntax tree, i feel like i am on the right
track, i have obtained so much knowledge and i am feeling happy for knowing this typeof of things,
i have been always questioning myself about the power of the language, and how to implement it, 
and at this moment in the book i feel like oh, it was that easy? at the beganning of the chapter 
i was like what the hell i am even reading, sounded like non-sense to me, but now after reading 
for a while i feel comfortable to understand this chapter, i learn everytime i tackle something 
hard that it was not hard, i just did not know how it worked in the first place, which sounds 
helarious :slight_smile: :duck:.

I could have mushed them all into a single expression class with an arbitrary list of children like
some compilers do, what i will do is to define a base enum for the different types of expressions,
and then for each type of expression -- each production under expression -- i will define a subenum
that has fields for the non-terminals of the production, this way i get a compiler error let's say
for accecing the second operand of a unary expression. 

---


