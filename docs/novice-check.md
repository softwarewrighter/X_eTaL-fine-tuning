# Trying the lessons with newcomers

The lessons are written for a reader who knows neither machine
learning nor X_eTaL and may not program (`docs/new-eyes.md`). The
tests check what a test can: every word that opens has an
explanation, every control has a spoken name and is reached by
keyboard, every number comes from the X_eTaL run. Whether a newcomer
actually comes away understanding is checked only by sitting one
down in front of the lessons. This is how, and what to ask.

The research this repository started from (`docs/research.txt`,
"Acceptance criteria") sets the bar: a beginner completes the first
lesson without reading anything else, and afterward can explain what
rank means, why W stays frozen, what A and B do, and why a small
adapter can change a model's output.

## Who

Someone who has used a chatbot and has heard that models can be
"fine-tuned" or "trained", and no more than that. Not a programmer,
or not one who has done machine learning. One person at a time; two
or three people in all tell more than one.

## How

1. Open the live catalog (https://softwarewrighter.github.io/X_eTaL-fine-tuning/)
   and ask them to start where it says to start: the welcome page,
   then LoRA hello. Say nothing about what the pages contain.
2. Watch, and write down where they stop, scroll back, or open an
   explanation. Do not help unless they are stuck for more than a
   minute; then note what was unclear and the words you used, since
   those words are what the page lacked.
3. Let them go on to the LoRA lab, LoRA training and the rest for as
   long as they care to; note where they stop wanting to.
4. Ask the questions below, in their order, after the first lesson
   (questions 1 to 4) and after the last one they read (5 to 8). Let
   them answer in their own words; do not correct them during the
   answer. Write the answer down as said.
5. Ask what they would try next, if anything, and what they would
   want the page to have told them.

Two other things to check without asking: whether they used the
keyboard or touch and anything failed to respond; and whether they
read any X_eTaL. The code is meant to be skippable; a newcomer who
felt they had to read it has found a page that leans on it.

## The questions

Each question names what a good answer contains. An answer need not
use these words; it needs the idea.

1. **What does the rank mean?** The width of the adapter's two thin
   tables, how many numbers go through the squeeze; a small rank is
   a small adapter and a coarse change, a larger rank a larger
   adapter and a finer one; on an 8 by 8 layer rank 4 is already as
   many numbers as the layer itself. A poor answer: "how good it
   is", or a number without a meaning.
2. **Why does W stay frozen?** W holds what the model already knows;
   leaving it alone keeps that; the adapter adds a correction beside
   it instead of overwriting it; fewer numbers to train and to
   store. A poor answer: "because it is big".
3. **What do A and B do?** A squeezes the input down to a few
   numbers, B spreads them back out to the layer's size; together,
   scaled by the strength, they are the correction added to W's
   output; B starts at zero so the adapter starts by changing
   nothing. A poor answer: names them as "the weights" without the
   squeeze and the spread.
4. **Why can a small adapter change the output?** Because the
   correction is added to every output, and a change along a few
   directions is enough to move the outputs that matter for the new
   task; it does not need to move every number of W to do so. A good
   answer may add that this is also why it cannot do everything:
   lesson 5 shows a frozen half that cannot leave its subspace.
5. **Where do the numbers of A and B come from?** From examples: a
   prediction, its error, the gradient, a small step, many times;
   the loss on the examples trained on goes down, and the loss on
   held-out examples says whether what was learned carries over.
6. **What does a larger rank buy, and cost?** More numbers trained
   and held while training; a lower loss on the task, up to a point;
   the held-out loss can rise; nothing on the page said a rank was
   better, only what was measured.
7. **Did the adapter protect what the layer knew before?** On lesson
   5's task, no: every method moved the old outputs; the page said an
   adapter was not by itself a guard of old behavior. (The lesson is
   honest about this; a reader who says "yes" has read the hope, not
   the measurement.)
8. **Name two other ways to change a model besides LoRA.** From the
   landscape: changing every weight or chosen ones; learning without
   back-propagation; a closed-form edit; an AI doing the editing; or
   changing no weight at all (the context). What each one changes,
   if they can say it.

## Recording the answers

One file per session under `docs/novice/`, named by date
(`2026-11-20.md`), not by the person. Record: the date, the
background in one line ("uses chatbots, no programming"), the
lessons read and the time spent on each, where they stopped or asked
for help (quote the words), each question with the answer as said and
your reading of it (has the idea / partly / not), what they would try
next, and what they wanted the page to have said. No names.

A pattern across two or three sessions (the same stop, the same
missing idea) is a change to the page, taken as a step; the one
session's remark is noted and waited on.

## What this does not check

Whether the lessons are right about machine learning (the tests and
the cited papers do that), whether a reader who already knows the
subject finds them too slow (they may; the modes exist for that),
and anything about real models (none is used here yet).
