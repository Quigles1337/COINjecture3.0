/-
NORMATIVE STATUS: draft — pending human ratification; formal-verification ownership reserved per LEDGER D16.

Draft encoding of Protocol Spec §7 transaction-validity rules V1–V9.

No canonical codec, signature implementation, address derivation, protocol maximum,
fee floor, or chain identifier is selected here. Those values and operations are
supplied through `ValidationContext`. This keeps every owner/G0-controlled TBD
symbolic. SI-001, SI-002, and SI-003 remain OPEN in
`loop/reports/SPEC-ISSUES.md`; this module neither instantiates nor resolves them.
-/

namespace Cj3.Spec.Tx

/-- An uninterpreted byte sequence. Fixed-width requirements remain explicit rules. -/
abbrev Bytes := ByteArray

/-- A transaction candidate after the abstract strict decoder accepts its bytes. -/
structure Candidate where
  networkId : UInt32
  sender : Bytes
  recipient : Bytes
  amount : UInt64
  fee : UInt64
  nonce : UInt64
  publicKey : Bytes
  signature : Bytes
deriving BEq

/-- The §8 account projection needed by V4 and V6. -/
structure Account where
  balance : UInt64
  nonce : UInt64
deriving Repr, BEq

/-- Lift an optional checked result into a typed rejection. -/
def requireSome (error : E) : Option A → Except E A
  | some value => .ok value
  | none => .error error

/--
Abstract dependencies of V1–V8.

`strictCanonicalDecode` owns strict decoding, field rejection, and canonical byte
rules. It is intentionally not implemented by P-005. In particular, no byte-level
choice from the unresolved SI-002/SI-003 family is smuggled through this interface.
`signatureVerifies` represents precisely the V2 predicate over the specified
domain-separated preimage, without choosing its byte construction here.
-/
structure ValidationContext (State : Type) where
  txMaxBytes : Nat
  strictCanonicalDecode : Bytes → Option Candidate
  signatureVerifies : Candidate → Bool
  addressOf : Bytes → Bytes
  feeMin : UInt64
  chainNetworkId : UInt32
  account : State → Bytes → Account

/-- Typed rejection reasons retain the V-rule that failed. -/
inductive Error where
  | v1StrictCanonicalDecode
  | v2Signature
  | v3SenderBinding
  | v4NonceEquality
  | v5FeeFloor
  | v6CheckedFunds
  | v7RecipientWidth
  | v8NetworkIsolation
deriving Repr, BEq, DecidableEq

/-- One greater than the maximum mathematical `u64` value. -/
def u64Modulus : Nat := 2 ^ 64

/-- Exact checked addition: overflow is represented by `none`, never wraparound. -/
def checkedAdd (left right : UInt64) : Option UInt64 :=
  let sum := left.toNat + right.toNat
  if h : sum < u64Modulus then
    some (UInt64.ofNatLT sum (by simpa [u64Modulus] using h))
  else
    none

/-- Exact checked subtraction: underflow is represented by `none`. -/
def checkedSub (left right : UInt64) : Option UInt64 :=
  if _h : right.toNat ≤ left.toNat then
    let difference := left.toNat - right.toNat
    have differenceFits : difference < UInt64.size :=
      Nat.lt_of_le_of_lt (Nat.sub_le left.toNat right.toNat) left.toNat_lt
    some (UInt64.ofNatLT difference differenceFits)
  else
    none

/-- V1: bounded input plus success of the unresolved strict canonical decoder. -/
def v1 (context : ValidationContext State) (input : Bytes) : Option Candidate :=
  if input.size ≤ context.txMaxBytes then
    context.strictCanonicalDecode input
  else
    none

/-- V2: abstract verification of the exact §7 domain-separated signing predicate. -/
def v2 (context : ValidationContext State) (tx : Candidate) : Bool :=
  context.signatureVerifies tx

/-- V3: explicit sender/address binding at the validation site. -/
def v3 (context : ValidationContext State) (tx : Candidate) : Bool :=
  tx.sender == context.addressOf tx.publicKey

/-- V4: strict equality with the current state's account nonce. -/
def v4 (context : ValidationContext State) (state : State) (tx : Candidate) : Bool :=
  tx.nonce == (context.account state tx.sender).nonce

/-- V5: the fee floor is a symbolic context value, not a P-005 constant. -/
def v5 (context : ValidationContext State) (tx : Candidate) : Bool :=
  tx.fee ≥ context.feeMin

/-- V6: checked `amount + fee`, followed by the current-state balance test. -/
def v6 (context : ValidationContext State) (state : State) (tx : Candidate) : Bool :=
  match checkedAdd tx.amount tx.fee with
  | none => false
  | some debit => (context.account state tx.sender).balance ≥ debit

/-- V7: genesis imposes only the explicit 32-byte recipient-width requirement. -/
def v7 (tx : Candidate) : Bool :=
  tx.recipient.size == 32

/-- V8: replay isolation against the symbolic chain network identifier. -/
def v8 (context : ValidationContext State) (tx : Candidate) : Bool :=
  tx.networkId == context.chainNetworkId

/-- V9: self-sends are legal; this rule never rejects a candidate. -/
def v9 (_tx : Candidate) : Bool := true

/--
The single §7 validity function. Mempool and block callers must use this same
predicate; P-005 deliberately provides no alternate or bypass constructor.
-/
def validate
    (context : ValidationContext State)
    (state : State)
    (input : Bytes) : Except Error Candidate := do
  let tx ← requireSome Error.v1StrictCanonicalDecode (v1 context input)
  if !v2 context tx then throw Error.v2Signature
  if !v3 context tx then throw Error.v3SenderBinding
  if !v4 context state tx then throw Error.v4NonceEquality
  if !v5 context tx then throw Error.v5FeeFloor
  if !v6 context state tx then throw Error.v6CheckedFunds
  if !v7 tx then throw Error.v7RecipientWidth
  if !v8 context tx then throw Error.v8NetworkIsolation
  if !v9 tx then unreachable!
  pure tx

/-- V9 is independent of sender/recipient equality. -/
theorem v9_allows_self_send (tx : Candidate) (_h : tx.sender = tx.recipient) :
    v9 tx = true := by
  simp [v9]

end Cj3.Spec.Tx
