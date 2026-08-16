/-
NORMATIVE STATUS: draft — pending human ratification; formal-verification ownership reserved per LEDGER D16.
-/

import Spec.Vectors

private def usage : String :=
  "usage: lake exe vectors [optional-output-path]"

/--
With no argument, print the deterministic draft vector JSON. With one argument,
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
