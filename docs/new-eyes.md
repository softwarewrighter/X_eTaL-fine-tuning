# New eyes: how a page is written and checked

The reader of a page here is a beginner. They do not know machine
learning: not what a weight, a matrix, a gradient or training is.
They do not know X_eTaL, have probably never heard of it, and may not
program at all. They came with a practical question, such as "how do
I teach my local LLM new things without catastrophic forgetting or
excessive time and electricity?". A page that loses them has failed,
however correct it is.

This is the checklist a page is written and read against (the user,
2026-11-05, after two reviews of the first page: it "jumps into the
subject", then "we need asides ... a new reader ... has never heard of
X_eTaL and is unfamiliar with the notation"; "consider a 'new eyes'
approach"; "links to hidden sections which use ELI5 to explain each
concept"; "direct links from code snippets to the xref").

## The checklist

1. **Framed.** The page opens with the question in the reader's
   words, the background needed, the idea and why it is a good answer,
   and what the page is and is not; it closes with what it showed, and
   did not, against the question, and where to go next
   (`lesson::chrome::frame`; the browser check requires the six).
2. **Every idea can be opened.** Each term of machine learning or of
   X_eTaL is, at its first use in a section, a word that opens a plain
   explanation in place: a few sentences, an everyday picture, no
   other jargon (`shared/lesson/glossary.tsv`; in the prose,
   `[shown|id]`). All of them are listed once near the end. A test
   fails a mark that names no idea.
3. **No code is needed to follow it.** The story is told in words and
   numbers. Code is folded into asides, opened by those who want it.
   No code in a heading.
4. **One calculation is worked in full** before anything is summed
   up: every step in order, each with its numbers, so no number
   appears from nowhere. Tables the text speaks of are shown.
5. **Code is explained where it is shown.** A call of a library
   function comes with the line that imports its library, what the
   function is for, its definition exactly as the pinned library has
   it (`lesson::explain`, never a retyped copy), a reading of it piece
   by piece, and links to its source and its entry in the
   cross-reference. A test checks every library function the program
   calls is found in its library.
6. **Code is described as it is drawn.** The page draws X_eTaL
   decorated (an arrow for `:=`, a raised prefix for `lo:`, an
   underlined letter). The prose speaks of what the reader sees, and
   says once how it is typed.
7. **Every function in code is a link** to its entry in the
   cross-reference: built-in words, X_eTaL-libraries' functions,
   X_eTaL-ML's (`lesson::code::Linker`). The browser check fails a
   link that lands nowhere. Each built-in word the program uses is
   also explained in a table; a test checks none is missing.
8. **One name for one thing.** If the library or the literature has
   another name (the update, delta), say so once.
9. **Nothing is over-claimed.** What was not trained was not shown to
   be learned or forgotten; a count is not a measurement of time or
   power. The summary says what was shown and what was not.
10. **The letters are introduced.** Before a formula is used, each
    letter in it is said to be a list, a table or a single number, and
    what writing two side by side means.

## The reading

Before a page is called done, its text is read cold by a reviewer who
is given only the page's visible text and the reader described above,
and asked where that reader gets lost, what is used before it is
explained, what is over-claimed, and what is out of order. The author
of a page cannot do this: they know too much. What the reading finds
is fixed, or noted on the page's README as known.

The first page's reading (2026-11-05) found, among others: prose
naming notation as typed while the page drew it otherwise; only one
step of the calculation worked through; three names for the
correction; merging seeming to contradict "frozen"; a summary claiming
no forgetting where nothing had been trained. All were fixed.

## How to get the text to read

```bash
just pages                         # build the page
scripts/browser-dom.sh SLUG        # its rendered document; the text inside <main> is the page
```
