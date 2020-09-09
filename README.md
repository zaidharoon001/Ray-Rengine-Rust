# Ray-Rengine Rust
Ray-Rengine is a regex engine that supports an advance form of regular expressions with explicit recursion

# Why Ray-Rengine and not Perl's extended RegEx
Ray-Rengine is supposed to be more readable than other regular expression because of the way it allows you to compose smaller regular expressions to build larger and more powerful ones. Recurion is also way more natural than it is any of the other regular expression dialects.

# Language Specification
Ray-Regular Expressions is the name of the language Ray-Rengine uses that is built specifically for it and is quite similar to normal regular expressions but here we have actual definition
are used to spread regular expressions in seprate parts and those definitions are then used for recurion in regular expressions. Here's the basic usage of this language.
```
main := 'base' (' ' | '\t' | '\n')* ('1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0')+
```
Now, you might be(reasonably) saying that's just regular expressions with extra steps and those extra steps being the `main` definition at the beginning but in actuality you can have as many defintions there
as you want with no restrictions whatsoever, but `main` is an entrypoint similar to main in C.

As you might be able to tell regex even here isn't the most readable thing so in order to solve that problem we have, as I previously discussed regular expressions. So it can be re-written as
```
whitespace := ' ' | '\t' | '\n'
nums := '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0'
main := 'base' whitespace* nums+
```
God that's better. We have only seen stuff that regular regular expressions and Ray-Regular Expressions can both do, but now we'll look at something that classic regular expressions(extended ones, like perl's, little unreadbaly can) cannot do but
Ray-Regular Expressions with a bit of recursion can. So, now we'll build an expression validator with support for paranthesis in Ray-Regular Expressions.
```
nums := '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0'
expr := term (('+'|'-') term)*
term := factor (('*'|'/') factor)*
factor := '(' expr ')' | nums+
main := expr
```
See that's readable RegEx. Let's add run it through some input. Like so,
```
12+3
18*(1-2)
```
All of them passed and this same technique can be used to validate basically anything. Now the only limitation is your imagination! and the size your stack.
