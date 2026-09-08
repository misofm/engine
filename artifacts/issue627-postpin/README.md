# Issue 627 post-pin preflight failure

The first authorized post-pin builder invocation targeted detached promotion
commit `0bb5a820be33a49393386617358ec1db2fe7577d` with the repin bypass unset.
The executor did not create `/tmp/issue627-postpin-output` before invoking the
unchanged builder. The builder rejected that argument during its initial
filesystem preflight with `output must be an existing non-symlink directory`.
No compilation or artifact publication began, the output path remains absent,
and no downstream static, resource, hermetic, SDK, matrix, formatting, or policy
command ran. The invocation was not retried.

The wrapper did not retain a numeric exit status; exact stderr, empty stdout,
and the absent output path show the builder stopped at argument preflight. A
subsequent identity census only recorded that absence and its raw diagnostics;
it did not invoke the builder or any qualification gate and was not retried.
Empty output census/hash records are retained. This record supplies no post-pin
artifact evidence and authorizes no further action. Astra LOW must review a
bounded final-attempt scope before any corrected invocation.
