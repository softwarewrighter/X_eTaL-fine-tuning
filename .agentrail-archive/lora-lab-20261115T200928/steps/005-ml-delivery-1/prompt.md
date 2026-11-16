Step ml-delivery-1 (inserted 2026-11-05; X_eTaL-ML's agent, relayed by
the user: FM1, FM2, FM3 and FM5 have landed there, "they can now mark
those landed and drop the workarounds once their pin moves").

Take X_eTaL-ML's first delivery, alone (docs/plan.md F1).
1. Move XETAL_ML_COMMIT to its pushed HEAD (`just ml-pin`), run `just
   gate --full`, commit the pin file by itself.
2. Check each ask against what landed (its docs/fine-tuning-asks.md is
   its word on them): FM1 (MICROSCOPE_LIBS, libs::names), FM2
   (chrome::Site, header_of, footer_of), FM3 (lo:s_calingRs), FM5
   (microscope::heat). Note anything asked that did not land, and
   anything landed that does not fit this repository.
3. Drop the workarounds: shared/lesson's own embedding of libraries
   and its run wrappers (FM1), what of its header and footer FM2 now
   gives; the pending steps' prompts (rank-and-scaling uses
   lo:s_calingRs; lesson-shell builds on microscope::heat).
4. The ledger (docs/xetal-ML-asks.md), the plan, README, CHANGES.md;
   `just gate --full`; publish and check-live.
