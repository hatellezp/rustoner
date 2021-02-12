# rustoner: a Description Logic reasoner written in rust
**it works now**

## Description 

[Description Logics](http://dl.kr.org) allow for fast and accurate reasoner systems. It is the main objective of DLs.
This project has a first objective to implement in [rust](https://www.rust-lang.org/) a reasoner for the simple logic
[dl_lite_r](https://link.springer.com/article/10.1007/s10817-007-9078-x).

DLs works like model theory, where you have axioms (that we put in a TBox) and also some grounded knowledge 
(that we put in ABoxes).
From this you can, under the limitations of the logic that you use (*dl_lite_r* here), ask
questions (queries), know if there are problems in your data (consistency verification) and sometimes ask
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
Man < Human
Human < Mortal
ENDTBOX
```
where **<** indicates that __Human__ is a _subclass_ of __Mortal__
and if you want your answer in a file called *__answer__* then call 
rustoner as follows:
```shell script
./rustoner --task ctb --tbox are_men_mortals --output answer
```

You should get something like this in *__answer__*:
```
BEGINTBOX
Man < Mortal
Human < Mortal
Man < Human
ENDTBOX
```

You can put your *symbols* declaration in a file elsewhere and specify it with
the option
```
--symbol path_to_symbol_file
```

**Note**: if you don't specify an output file, you will get your answer
on _stdout_:
```shell script
----<TBox>
    {
     : Man (<) Mortal
     : Human (<) Mortal
     : Man (<) Human
    }
```

**Note**: if you want __(a lot of!)__ explanations you can specify a 
verbose option:
```
--verbose
```

### Finding Consequences

Same as the previous task, only know you want to know consequences of
grounded assertions. Put your axioms in  **are_men_mortals**:
```
BEGINSYMBOL
concept : Man
concept : Human
concept : Mortal
ENDSYMBOL 

BEGINTBOX
Man < Human
Human < Mortal
ENDTBOX
```

and you knowledge in a file **a_man**:
```
BEGINABOX
Socrates: Man
ENDABOX
```

Call **rustoner** with the complete abox task:
```shell script
./rustoner --task cab --tbox are_men_mortals --abox a_man --output answer
```

You should get the following in the **answer** file:
```
BEGINABOX
Socrates: Man
Socrates: Human
Socreates: Mortal
ENDABOX
```

### SQLite

You can also put your ontology on a [sqlite](https://www.sqlite.org/index.html) database.

__Why sqlite ?__
Because it's portable, supported everywhere and you can't use it without any
knowledge of client server protocol of larger database systems.

Coming back, you can create a copy of your ontology on a sqlite database with the
command 
```
./rustoner --task init --tbox are_men_mortals 
```
it will create a file __are_men_mortals.db__ with your facts and other several tables.
Then you can add and complete several abox with the command 
```
 ./rustoner --task cab --db are_men_mortals.db --abox a_man 
```
each abox will benefit of its own tables (one for roles and one for concepts) and all
can live in the same database.

### Querying

### Ranking Assertions

## Where are we

For the moment:
* I'm translating part of my work already done in __Python__;
* adding support for text files;
* implementing the [assertion ranking](http://ceur-ws.org/Vol-2663/paper-20.pdf);
* adding support for sql based databases
