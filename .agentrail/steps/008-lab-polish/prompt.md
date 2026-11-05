Step lab-polish (docs/plan.md, Saga 1 row 7 and the saga's acceptance;
docs/research.txt "Acceptance criteria"). The last step of the saga.

1. Explanations complete: a test per page lists every visible symbol
   and control and fails on one without a glossary term; fill what is
   missing. Every heat map has its legend and scale note, every slider
   its sentence.
2. Accessibility: browser tests that walk each lesson by keyboard
   alone and with touch events; fix what they find.
3. Determinism: a replay test per preset on every page; a test that
   playing, pausing and stepping change no value of the trace.
4. docs/novice-check.md: how to try the lessons with newcomers, and
   the questions to ask after (what does rank mean? why does W stay
   frozen? what do A and B do? why can a small adapter change the
   output?), with a place to record answers.
5. README: the demos table with the live links and a screenshot per
   lesson; Status; the catalog's order as the curriculum's.
6. docs/plan.md: Saga 1 marked done with what was built; Saga 2
   (basics-and-full) reviewed against what was learned, its steps
   ready to become the next saga. The asks files brought up to date.
7. `just gate --full`, publish, check-live.

Complete with --done. Report to the user: what is live, the asks to
take to the other repositories, and the proposal for Saga 2.
