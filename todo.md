# TODO
* **Feature:** capture points
  * Capture points have ids
  * They mark begin and end state
  * Everything in between will be captured and saved labeled with the id
  
* **Feature:** Unoptimized parsing through dfa and nfa

* **Feature:** Nested FAs in each other
  * Later: Small, non-recursive ones can be merged into (more efficient, less complicated for computing)
  * A stack keeps track of current FA to be run
  * Deterministic:
    * Parallel FAs need to be checked for same beginning sequences.
    * Those should be merged together and the start of a unique terminal will link to the actual FA.
    * All capture points in this merge progress need to be taken in.
    * As soon as the actual FA was entered, it will insert a marker to a list, so we can keep track of what capture point are important

* **Feature:** Compiling the Nested DFA to machine code

* **Feature:** Build a nice frontend library