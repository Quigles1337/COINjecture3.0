/-
NORMATIVE STATUS: draft — pending human ratification; formal-verification ownership reserved per LEDGER D16.

Draft model of Protocol Spec §8 state-transition sequencing, checked arithmetic,
block-granularity atomicity, and the conservation theorem target.

B-rule validation, reward selection, authenticated-state storage, and root computation
remain abstract interfaces because their concrete definitions are outside the P-005
authority surface. No Al-owned, Sarah-owned, G0-controlled, SI-001, SI-002, or SI-003
choice is instantiated here.
-/

import Spec.Tx

namespace Cj3.Spec.Stf

open Cj3.Spec.Tx

/-- The §8 block projection used by the draft transition model. -/
structure Block where
  transactions : List Bytes
  minerAddress : Bytes
  expectedStateRoot : Bytes
deriving BEq

/--
Abstract authenticated-state operations. `setAccount` is the only state update
surface exposed to this model; each subsequent read observes the candidate state
returned by the preceding operation, including all address-aliasing cases.
-/
structure StateOps (State : Type) where
  setAccount : State → Bytes → Account → State
  totalBalances : State → Nat
  stateRootMatches : State → Bytes → Bool

/--
Symbolic §8 dependencies. `blockRulesHold` stands only for B1–B8 and B11–B12.
`rewardAmount` is the checked §11 miner credit. `subsidyAmount` is the distinct value
named by §8's conservation equation. SI-004 intentionally leaves their relationship
unconstrained.
-/
structure Context (State : Type) where
  txValidation : ValidationContext State
  blockRulesHold : Block → Bool
  rewardAmount : Block → UInt64
  subsidyAmount : Block → UInt64

/-- Typed failures preserve the exact stage at which a candidate block was rejected. -/
inductive Error where
  | blockRules
  | txInvalid (cause : Tx.Error)
  | senderDebit
  | recipientCredit
  | minerFeeCredit
  | senderNonceIncrement
  | rewardCredit
  | stateRootMismatch
deriving Repr, BEq, DecidableEq

/--
Apply one transaction after re-running V1–V9 against the current candidate state.
The explicit `minerAddress` argument comes from the enclosing block.
-/
def applyTransaction
    (context : Context State)
    (ops : StateOps State)
    (minerAddress : Bytes)
    (state : State)
    (input : Bytes) : Except Error State := do
  let tx ←
    (Tx.validate context.txValidation state input).mapError Error.txInvalid
  let debit ← requireSome Error.senderDebit (checkedAdd tx.amount tx.fee)
  let senderBefore := context.txValidation.account state tx.sender
  let senderBalance ←
    requireSome Error.senderDebit (checkedSub senderBefore.balance debit)
  let afterDebit :=
    ops.setAccount state tx.sender { senderBefore with balance := senderBalance }

  let recipientBefore := context.txValidation.account afterDebit tx.recipient
  let recipientBalance ←
    requireSome Error.recipientCredit (checkedAdd recipientBefore.balance tx.amount)
  let afterRecipient :=
    ops.setAccount afterDebit tx.recipient
      { recipientBefore with balance := recipientBalance }

  let minerBefore := context.txValidation.account afterRecipient minerAddress
  let minerBalance ←
    requireSome Error.minerFeeCredit (checkedAdd minerBefore.balance tx.fee)
  let afterMiner :=
    ops.setAccount afterRecipient minerAddress { minerBefore with balance := minerBalance }

  let senderBeforeNonce := context.txValidation.account afterMiner tx.sender
  let senderNonce ←
    requireSome Error.senderNonceIncrement (checkedAdd senderBeforeNonce.nonce 1)
  pure <|
    ops.setAccount afterMiner tx.sender { senderBeforeNonce with nonce := senderNonce }

/-- Ordered fold: each transaction is validated against the state produced so far. -/
def applyTransactions
    (context : Context State)
    (ops : StateOps State)
    (minerAddress : Bytes) :
    State → List Bytes → Except Error State
  | state, [] => pure state
  | state, input :: rest => do
      let next ← applyTransaction context ops minerAddress state input
      applyTransactions context ops minerAddress next rest

/-- Apply the symbolic §11 reward with checked addition. -/
def applyReward
    (context : Context State)
    (ops : StateOps State)
    (block : Block)
    (state : State) : Except Error State := do
  let minerBefore := context.txValidation.account state block.minerAddress
  let minerBalance ←
    requireSome Error.rewardCredit
      (checkedAdd minerBefore.balance (context.rewardAmount block))
  pure <|
    ops.setAccount state block.minerAddress { minerBefore with balance := minerBalance }

/--
Construct the complete post-state candidate without committing it. Every error
discards this internal candidate at the atomic wrapper.
-/
def applyBlockCandidate
    (context : Context State)
    (ops : StateOps State)
    (state : State)
    (block : Block) : Except Error State := do
  if !context.blockRulesHold block then throw Error.blockRules
  let afterTransactions ←
    applyTransactions context ops block.minerAddress state block.transactions
  let afterReward ← applyReward context ops block afterTransactions
  if !ops.stateRootMatches afterReward block.expectedStateRoot then
    throw Error.stateRootMismatch
  pure afterReward

/-- The only externally visible block result: rejected blocks retain `preState`. -/
structure AtomicOutcome (State : Type) where
  state : State
  error : Option Error

/-- Block-granularity atomic commit; no partially applied state is returned. -/
def applyBlockAtomic
    (context : Context State)
    (ops : StateOps State)
    (preState : State)
    (block : Block) : AtomicOutcome State :=
  match applyBlockCandidate context ops preState block with
  | .ok postState => { state := postState, error := none }
  | .error cause => { state := preState, error := some cause }

/-- Mechanical statement of §8's rejection-is-unchanged requirement. -/
theorem rejection_preserves_state
    (context : Context State)
    (ops : StateOps State)
    (preState : State)
    (block : Block)
    (cause : Error)
  (h : (applyBlockAtomic context ops preState block).error = some cause) :
    (applyBlockAtomic context ops preState block).state = preState := by
  unfold applyBlockAtomic at h ⊢
  split at * <;> simp_all

/--
Literal transcription of §8's conservation theorem target. The candidate transition
above credits `rewardAmount`, while this target names `subsidyAmount`; SI-004 records
that the governing prose does not currently make those quantities coherent. No
theorem is asserted and no equality between them is introduced here.
-/
def ConservationTarget
    (context : Context State)
    (ops : StateOps State)
    (preState : State)
    (block : Block) : Prop :=
  match applyBlockCandidate context ops preState block with
  | .ok postState =>
      ops.totalBalances postState =
        ops.totalBalances preState + (context.subsidyAmount block).toNat
  | .error _ => True

end Cj3.Spec.Stf
