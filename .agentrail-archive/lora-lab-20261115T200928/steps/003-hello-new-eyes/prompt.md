Step hello-new-eyes (inserted 2026-11-05 on the user's second review
of LoRA hello: "we need asides, to explain lo:f_orward and lo:m_erge
(since a new reader interested in fine-tuning probably has never heard
of X_eTaL and is unfamiliar with the notation, and the example lacks
the u_se< showing where this came from, no link to the source/x-ref,
no explanation of these functions purpose/implementation)", and: "you
need to consider a 'new eyes' approach to documentation; these demos
are intended for beginners, who do not yet know ML, or X_eTaL").

1. shared/lesson: an aside component, and the explanation of a
   library function taken from the pinned library's own source (its
   doc comment, its definition, where it lives: the repository, the
   file and line at the pinned commit, its page in the
   cross-reference), so a page shows the real code, never a retyped
   copy; the imports and the library functions a program calls found
   from the program's text.
2. demos/lora-hello, read with new eyes: an aside on what X_eTaL is
   and how to read its notation; the import line shown with the code
   it enables; asides for lo:f_orward and lo:m_erge (purpose, the
   definition as the library has it, how to read it piece by piece,
   links to the source and the cross-reference); every library
   function and built-in word the program uses explained; the layer's
   numbers (W, A, B) shown, since the page says it can be read in
   full; the squeeze A x worked as arithmetic with the run's numbers;
   every remaining term of jargon explained or removed.
3. The rule: CLAUDE.md and docs/plan.md ("new eyes": the reader knows
   neither machine learning nor X_eTaL); docs/new-eyes.md, the
   checklist a page is read against; a test that every library
   function a page's program calls has its explanation on the page.
4. A reading of the finished page by a reviewer given only the page's
   text and told to read as a beginner; what it finds fixed or noted.
5. Tests, baselines reblessed after review, README, the demo's README,
   CHANGES.md; publish, check-live.
