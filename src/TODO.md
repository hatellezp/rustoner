- tasks that I want:
    - [x] verify tbox (for consistency)
        - [x] add consequence tree if possible
            - [x] !!! in the mutex find a way to add new impliers
    - [x] complete tbox
    - [x] generate consequence tree tbox
    - [x] verify abox
        - [x] the consequence tree is valid here too
            - [x] same with the mutex here
    - complete abox
    - generate consequence tree abox
    - rank abox
    - create graph

- see why you have duplicates on the tbox
- look up Cleora written in rust for graphs

### pour demain:
 - une fois la matrice de conflicts faite (pas celle agregée):
    - [x] vois s'il n'y que des conflicts alors pas de problème
    - [x] sinon:
        - faire une matrice ou il n'y que un seul élément propre et 
          le reste ce sont des element conflictifs
        - faire un mapping pour aller et retour de ça et eviter des
          computations
    - [x] normaliser vers 1 si ils sont tous conflictifs, sinon ajouster
      pour que le seul fait non conflicting revient à 1
 - mapper le ranking vers les faits dans la abox
 - creer un graph de conflict avec les fait conflictifs (**Cleora**)
 - quoi qu'il arrive finir **l'interface!!!** 

### pour quand j'aurais le temps:
 - benchmark:
    - [x] read symbol 
    - read and write tbox
    - read and write abox
    - complete tbox
    - complete abox
    - rank !!
- creer des exemples parlants
- parser des XML :(
- benchmark with LUBM at least