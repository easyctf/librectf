# Challenge Import

You can import challenges into your CTF platform directly by using the challenge import tool.

## Format

The tool should be run from the root directory of all challenges. Each subdirectory of that directory will be considered a challenge, so if you want some extra files that are not considered to be part of a challenge, put a `.` before the name to hide the directory. The directory name is used as a **unique identifier** for this challenge, and will be stored in the database.

**Note:** on subsequent runs of the import tool, if a challenge exists in the database with the same name as the challenge directory, any scores for that challenge will not be wiped; instead, the problem data will simply be overwritten. If you'd like to wipe the scores as well, you must manually delete them.

Each directory *must* have a `problem.toml` that contains metadata about the problem. This must contain a minimum of these fields:

* `title: str` - The title of your challenge.
* `description: str | description_file: path` - Specifying either of these provides a description. `description_file` will read the description from the file.
* `grader: path` - The path to the grader file.

As an example, a full configuration for an automatically generated Caesar cipher challenge is provided here:

```toml
title: "Caesar cipher"
description: """
"""
grader: "grader.py"
```

## Grader

The grader is an executable which receives information about the submission and must return information about the correctness of the submission. It will receive the problem nonce and team ID through command-line arguments, and then the user's flag submission through standard input. It should print some feedback about the submission to standard output, and then exit with 1 (incorrect), or 0 (correct) to indicate whether the submission was correct or not.

**Note:** The grader file must be an *executable*. If you wish to use a scripting language such as Python or Perl, make sure you add the [shebang](https://en.wikipedia.org/wiki/Shebang_(Unix)) (`#!`) line at the top of the file to indicate the means by which to run this file.

As an example, the grader for a basic string flag is provided here:

```py
#!/usr/bin/env python3
import sys
user_flag = input().strip()
if user_flag != "flag{congratulations}":
    print("Incorrect.")
    sys.exit(1)
print("Correct.")
# automatically exits with status 0
```

Also, the grader for an automatically generated Caesar cipher challenge is provided here:

```py
#!/usr/bin/env python3
import sys
import random

# for brevity, we'll just assume that these arguments exist without checking
nonce = sys.argv[1]
tid = sys.argv[2]

# seed the random generator
rng = random.SystemRandom("{}_{}".format(nonce, tid))
actual_flag = "flag{congratulations_%s}" % rng.token_hex(6)

# check correctness
user_flag = input().strip()
if user_flag != actual_flag:
    print("Incorrect.")
    sys.exit(1)
print("Correct.")
# automatically exits with status 0
```

## Problem build dependencies

Sometimes, there may be extra steps you'd like to take before you import the challenges, such as building binaries or generating files. Perhaps you are storing all of your challenges in a code repository and don't want to commit build artifacts. In order to cater to this need, OpenCTF will check for the existence of a `Makefile` in the challenge directory and run it before anything else happens.

Make sure that wherever you're running this import tool, you also have all the dependencies required to run the Makefiles. Don't put anything user- or team-specific in these Makefiles, since they are only run *once* during import. If you'd like to generate files that are specific to a user or team, set the challenge to `autogen` in `problem.toml`.
