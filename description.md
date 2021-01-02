__DL-Lite Description__

_Roles_ and _Concepts_ can be defined inductively,
after that we specify _TBox_ and _ABox_ axioms and 
assertions respectively

* roles:
    * base role (type R)
    * the inverse of a base role (type R)
    * the negation of one of the above (type E)
* concepts:
    * bottom (type B)
    * base concept (type B)
    * existential restriction with a role of first or second complexity (type R)
    * the negation of one of the above (type C)
* tbox axioms:
    * basic concept assertions: B subsumed_by C
    * role assertions: R subsumed_by E
* abox assertions:
    *  A(a) where A is a base concept
    * P(a,b)  where P is a base role
   
    
__Some details on the implementation__

For a more economic use of the memory all information will be stored
in a bag (Context), and all tbox, abox objects will reference these
objects in the bag. I'm not sure, but I don't think that new items
should be created by the procedures, if it is so, then this
implementation is correct.

So TBoxItem, ABoxItem, TBox and ABox, all will have a reference to
a context Context.

You should remember that '0' the number is reserved for concept 'Bottom'
when mapping names to numbers