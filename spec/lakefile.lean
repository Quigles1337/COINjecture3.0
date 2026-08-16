import Lake

open Lake DSL

package «cj3-spec» where
  version := v!"0.1.0"

lean_lib Spec

@[default_target]
lean_exe vectors where
  root := `Main
