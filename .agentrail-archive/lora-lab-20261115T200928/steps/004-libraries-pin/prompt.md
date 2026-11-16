Step libraries-pin (inserted 2026-11-05; the user: "if you ask for
changes in multiple repos, expect multiple repo changes to arrive
separately and not all at once. Is it easier to pin each separately
and test after each single pin change?" Yes.)

Give X_eTaL-libraries a pin of its own here, so a delivery from it is
taken alone and tested alone, without waiting for X_eTaL-ML to move
its libraries pin.

1. XETAL_LIBRARIES_COMMIT (starting at the commit X_eTaL-ML pins, so
   nothing changes today); scripts/xetal-libraries.sh clones it into
   work/xetal-libraries at that commit (never edited); `just libs-pin
   [REF]`; `just xetal-version` shows it; check-xetal.sh checks it.
2. The search path (scripts/libpath.sh, scripts/xt) takes
   X_eTaL-libraries from that clone, not from X_eTaL-ML's copy.
3. The pages agree with the command line: shared/lesson puts every
   library of that clone in the store, replacing the copies
   X_eTaL-ML's microscope embedded at its own libraries pin; a test.
4. X_eTaL itself stays X_eTaL-ML's (the page shell is compiled against
   X_eTaL-ML's copy; a pin of its own would let the command line and
   the browser run different languages): said in the plan.
5. docs/plan.md F1, CLAUDE.md rule 2, README, the asks files (FM12 is
   no longer needed here), CHANGES.md. `just gate --full`.
