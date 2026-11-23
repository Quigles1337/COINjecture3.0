/-
NORMATIVE STATUS: RATIFIED — human-reviewed; formal-verification ownership reserved per LEDGER D16

The exported vector artifact remains non-normative and symbolic. Al's ruling of
2026-11-22 resolved SI-001, SI-002 and SI-003 without changing it.
-/

import Spec.Vectors

private def usage : String :=
  "usage: lake exe vectors [optional-output-path]"

/--
With no argument, print the deterministic non-normative vector JSON. With one argument,
write that same JSON plus a terminal newline to the requested artifact path.
-/
def main (args : List String) : IO UInt32 := do
  let rendered := Cj3.Spec.Vectors.render ++ "\n"
  match args with
  | [] =>
      IO.print rendered
      return 0
  | [path] =>
      IO.FS.writeFile path rendered
      return 0
  | ["--", path] =>
      IO.FS.writeFile path rendered
      return 0
  | _ =>
      IO.eprintln usage
      return 2
