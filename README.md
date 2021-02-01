# rustoner: a Description Logic reasoner written in rust
**(under heavy (not that heavy now...) developpement)**


## Description 

[Description Logics](http://dl.kr.org) allow for fast and accurate reasoner systems. It is the main objective of DLs.
This project has a first objective to implement in [rust](https://www.rust-lang.org/) a reasoner for the simple logic
[dl_lite_r](https://link.springer.com/article/10.1007/s10817-007-9078-x).

DLs works like model theory, where you have axioms (that we put in a TBox) and also some grounded knowledge 
(that we put in ABoxes).
From this you can, under the limitations of the logic that you use (*dl_lite_r* here), ask
question (queries), know if there are problems in your data (consistency verification) and sometimes ask
for implicit information in your data (reason and inference).

## Use

### Completing axioms 
Suppose you want to complete a group of axioms 
_"a man is human"_ and _"a human is mortal"_.
Then put them in a text file *__are_men_mortals__*:
```
BEGINSYMBOL
concept : Man // concept are unary relations
concept : Human
concept : Mortal
role : eats // and role are binary relations (that we don't use in this really simple example)
ENDSYMBOL // you can put a comment here if you want

BEGINTBOX
Man : Human
Human : Mortal
ENDTBOX
```
and if you want your answer in a file called *__answer__* then call 
rustoner as follows:
```shell script
./rustoner --task ctb --path_tbox are_men_mortals --path_output answer
```

You should get something like this in *__answer__*:
```
BEGINTBOX
Man : Mortal
ENDTBOX
```

### Finding Consequences

### Ranking Assertions

## Where are we

For the moment:
* I'm translating part of my work already done in __Python__;
* adding support for text files;
* implementing the [assertion ranking](http://ceur-ws.org/Vol-2663/paper-20.pdf);
* adding support for sql based databases
