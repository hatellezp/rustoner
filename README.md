# rustoner: a Description Logic reasoner written in rust
**(under heavy developpement)**


## Description 

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
