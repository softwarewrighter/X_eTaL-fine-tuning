# X_eTaL-fine-tuning tasks. Recipes call scripts/*.sh, which hold the
# logic and work without just too. `just` alone lists the recipes.
# Adapted from X_eTaL-ML's justfile.

set positional-arguments

# List the recipes
default:
    @just --list

# Get X_eTaL-ML at the known-good commit in XETAL_ML_COMMIT (clone in work/xetal-ml), build the X_eTaL it pins (binary bin/xetal), and get X_eTaL-libraries at XETAL_LIBRARIES_COMMIT (work/xetal-libraries)
xetal:
    @scripts/xetal-ml.sh

# Pin a newer X_eTaL-ML: a committed ref of ../X_eTaL-ML (default HEAD) into XETAL_ML_COMMIT, built; then just gate --full and commit
ml-pin ref="HEAD":
    scripts/ml-pin.sh "$1"

# Pin a newer X_eTaL-libraries: a committed ref of ../X_eTaL-libraries (default HEAD) into XETAL_LIBRARIES_COMMIT; then just gate --full and commit
libs-pin ref="HEAD":
    scripts/xetal-libraries.sh --pin "$1"

# The pins: XETAL_ML_COMMIT (and the X_eTaL commit X_eTaL-ML pins), XETAL_LIBRARIES_COMMIT, and the binary's own report
xetal-version:
    @scripts/xetal-ml.sh >/dev/null
    @echo "XETAL_ML_COMMIT $(cat XETAL_ML_COMMIT)"
    @echo "  its XETAL_COMMIT $(cat work/xetal-ml/XETAL_COMMIT)"
    @echo "  (its own XETAL_LIBRARIES_COMMIT $(cat work/xetal-ml/XETAL_LIBRARIES_COMMIT), not used here)"
    @echo "XETAL_LIBRARIES_COMMIT $(cat XETAL_LIBRARIES_COMMIT)"
    @bin/xetal --version

# Evaluate an expression with the pinned xetal, every library on XETAL_PATH: just eval "'+ r_/ 1 2 3"
eval expr:
    @scripts/xt eval -e "$1"

# The library search path (X_eTaL-ML's libraries at its pin, X_eTaL-libraries' at this repo's own): export XETAL_PATH="$(just path)"
path:
    @scripts/libpath.sh

# Check the pinned toolchain: the clone at the pin, the CLI builds, answers and reports its commit; LoRA, Check and Matrix load
check-xetal:
    scripts/check-xetal.sh

# The demos (lessons and experiments), in catalog order
demos:
    @scripts/py scripts/demos.py list

# Start a demo sub-project from demos/_template: just new-demo lora-lab "LoRA lab"
new-demo slug title:
    scripts/new-demo.sh "$1" "$2"

# Run a demo's program (default SLUG.xtl) with the pinned xetal: just run lora-hello
run slug file="":
    @scripts/run-demo.sh "$1" ${2:+"$2"}

# Run a demo's program as a notebook: each statement drawn, then its output
show slug file="":
    @scripts/run-demo.sh --echo "$1" ${2:+"$2"}

# A demo's program after macro expansion (what its macro calls became): just expand lora-lab
expand slug file="":
    @cd demos/$1 && XETAL_PATH="$(../../scripts/libpath.sh ../../)" ../../bin/xetal expand "${2:-$1.xtl}"

# Test one demo: its reg-rs baselines (CLI, page), web/ tests, test.sh
test-demo slug:
    scripts/test-demos.sh "$1"

# Accept one demo's current output as its reg-rs baselines, creating missing ones (review the diff!)
bless slug:
    XETAL_BLESS=1 scripts/test-demos.sh "$1"

# The libraries of this repository and their aliases
libs:
    @scripts/py scripts/libs.py table

# Start a library from templates/Library: just new-lib FineTune ft: "full fine-tuning of a layer"
new-lib name alias summary:
    scripts/new-lib.sh "$1" "$2" "$3"

# Run a library's test programs (or one): just run-lib FineTune basics
run-lib name program="":
    @scripts/run-lib.sh "$1" ${2:+"$2"}

# Run a library's demos (or one): just demo FineTune
demo name program="":
    @scripts/run-lib.sh --demos "$1" ${2:+"$2"}

# Test one library: its reg-rs baselines, pinned types and expansions
test-lib name:
    scripts/test-libs.sh "$1"

# Accept one library's current output as its baselines (review the diff!)
bless-lib name:
    XETAL_BLESS=1 scripts/test-libs.sh "$1"

# Every demo and library baseline
test:
    scripts/test-demos.sh
    scripts/test-libs.sh

# Run every ## >> example in the libraries' doc comments (xetal doc --test); the gate runs it
doc-test:
    scripts/doc-test.sh

# Check that every name is bound once in its scope (X_eTaL lang-choices M1); the gate runs it
check-rebind:
    scripts/py scripts/check-rebind.py

# Load one demo's built page in headless Chrome and check it shows X_eTaL's results
browser-check slug:
    scripts/browser-check.sh "$1"

# Walk one demo's built page by keyboard alone and by touch in headless Chrome (the browser check runs it too)
browser-walk slug:
    scripts/browser-walk.sh "$1"

# Build the live site into pages/ (not tracked; just publish publishes it)
pages:
    scripts/build-pages.sh

# The cross-reference (xetal doc) of every library and demo program, into pages/doc; just pages builds it too
doc:
    scripts/doc-site.sh

# Screenshots of the built pages for the catalog's cards and the READMEs (demos/<slug>/screenshot.png)
screenshots *slugs:
    scripts/screenshots.sh "$@"

# Publish pages/ as the gh-pages branch's only commit (the live site); needs a clean work tree
publish:
    scripts/publish-pages.sh

# Verify the deployed site after a publish: the catalog reports this commit and every live page runs X_eTaL (headless Chrome)
check-live:
    scripts/check-live.sh

# Serve the built pages/ as GitHub Pages will: http://127.0.0.1:8212/X_eTaL-fine-tuning/ (8212 is this repo's port; each X_eTaL repo has its own)
serve-pages port="8212":
    scripts/serve-pages.sh "$1"

# Serve one demo's web app locally, rebuilt on change: just serve lora-lab
serve slug port="8212": xetal
    cd demos/{{slug}}/web && trunk serve --release --port {{port}} --address 127.0.0.1

# The pre-commit gate: the cheap checks and what the change touches; just gate --full for everything
gate *args:
    scripts/gate.sh "$@"
