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

```shell script
./dllitemm
```
BEGINSYMBOL
concept : Man
concept : Human
ENDSYMBOL

BEGINTBOX
Man : Human // this is a comment
ENDTBOX
// this is also a comment

BEGINABOX
hatellezp : Man
// Socrates : Man (this line is also ignored)
ENDABOX
```


The idea is really simple, this engine uses a DL formalim to
implement reasoning.

You have a **TBox** which stores the theory 
(e.g. _"all men are human"_) and a **ABox** which stores
facts (e.g. _"hatellezp is a man"_).
When you mix both this system (_should_) arrive to
output that _"hatellezp is a human"_.

## How it works

Two main approaches :
   * first an inner reasoner (for the moment implemented for 
[dl_lite_r](https://link.springer.com/article/10.1007/s10817-007-9078-x));
   * a translation to a [SAT](https://en.wikipedia.org/wiki/Boolean_satisfiability_problem) 
   instance a use an external solver.


## Where are we

For the moment I'm translating my work on this from **Python 3**
to rust, should be significantly faster more adapted to works
with logic
