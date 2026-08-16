/-
NORMATIVE STATUS: draft — pending human ratification; formal-verification ownership reserved per LEDGER D16.

Draft model of Protocol Spec §8 state-transition sequencing, checked arithmetic,
block-granularity atomicity, and the conservation theorem target.

B-rule validation, the subsidy schedule, authenticated-state storage, and root
computation remain abstract interfaces because their concrete definitions are outside
the P-005 authority surface. The reward itself is derived from ratified SI-004 Model 4
without selecting `R_MAX` or any curve-shaping value. No Al-owned, Sarah-owned,
G0-controlled, SI-001, SI-002, or SI-003 choice is instantiated here.
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
surface exposed to this model; each subsequent read is performed against the candidate
state returned by the preceding operation, including all address-aliasing cases.
Concrete read/write coherence is an explicit P-101 instantiation obligation.
-/
structure StateOps (State : Type) where
  setAccount : State → Bytes → Account → State
  totalBalances : State → Nat
  stateRootMatches : State → Bytes → Bool

/-- The ratified P-6 fixed-point scale. This value is definitional, not a TBD. -/
def qualityScale : Nat := 1_000_000

/--
Inputs to the ratified SI-004 Model 4 reward function.

`rMax` remains symbolic and owner-controlled; only its ratified domain `R_MAX ≥ 1`
appears here. `subsidy` is supplied by the unresolved P-12 schedule, and `quality` is
the checker-derived Q. There is no reward-bound assumption in this structure.
-/
structure RewardInputs where
  subsidy : UInt64
  quality : UInt64
  rMax : Nat
  rMaxAtLeastOne : 1 ≤ rMax

/-- The positive Model 4 floor-division denominator `R_MAX · SCALE`. -/
def rewardDenominator (inputs : RewardInputs) : Nat :=
  inputs.rMax * qualityScale

/--
The mathematical u128 numerator. Both multiplicands are bounded by u64 because the
quality term is capped before multiplication.
-/
def rewardNumerator (inputs : RewardInputs) : Nat :=
  inputs.subsidy.toNat *
    min inputs.quality.toNat (rewardDenominator inputs)

/-- Ratified SI-004 Model 4 reward, with exact floor division. -/
def rewardNat (inputs : RewardInputs) : Nat :=
  rewardNumerator inputs / rewardDenominator inputs

/-- `R_MAX ≥ 1` and the definitional positive SCALE make division well-defined. -/
theorem reward_denominator_positive (inputs : RewardInputs) :
    0 < rewardDenominator inputs := by
  unfold rewardDenominator
  exact Nat.mul_pos inputs.rMaxAtLeastOne (by decide)

/--
The multiplication performed by a u128 implementation cannot overflow: subsidy and
the capped quality factor are each strictly below `2^64`.
-/
theorem reward_numerator_fits_u128 (inputs : RewardInputs) :
    rewardNumerator inputs < 2 ^ 128 := by
  have hCappedQuality :
      min inputs.quality.toNat (rewardDenominator inputs) < 2 ^ 64 :=
    Nat.lt_of_le_of_lt
      (Nat.min_le_left inputs.quality.toNat (rewardDenominator inputs))
      inputs.quality.toNat_lt
  have hProduct :
      inputs.subsidy.toNat *
          min inputs.quality.toNat (rewardDenominator inputs) <
        (2 ^ 64) * (2 ^ 64) :=
    Nat.mul_lt_mul_of_lt_of_le
      inputs.subsidy.toNat_lt
      (Nat.le_of_lt hCappedQuality)
      (by decide)
  calc
    rewardNumerator inputs < (2 ^ 64) * (2 ^ 64) := by
      exact hProduct
    _ = 2 ^ 128 := by
      simp

/--
The issuance ceiling required by the SI-004 ruling.

This is proved from the floor-division formula and `min(Q, R_MAX·SCALE) ≤
R_MAX·SCALE`; `reward ≤ subsidy` is not an input, field, postulate, or assumption.
-/
theorem reward_le_subsidy (inputs : RewardInputs) :
    rewardNat inputs ≤ inputs.subsidy.toNat := by
  unfold rewardNat rewardNumerator
  apply Nat.div_le_of_le_mul
  simpa [Nat.mul_comm, Nat.mul_left_comm, Nat.mul_assoc] using
    Nat.mul_le_mul_left inputs.subsidy.toNat
      (Nat.min_le_right inputs.quality.toNat (rewardDenominator inputs))

/-- The lower endpoint of the ratified range for every valid `Q ≥ SCALE`. -/
theorem subsidy_div_rMax_le_reward
    (inputs : RewardInputs)
    (qualityValid : qualityScale ≤ inputs.quality.toNat) :
    inputs.subsidy.toNat / inputs.rMax ≤ rewardNat inputs := by
  have hScaleLeCap : qualityScale ≤ rewardDenominator inputs := by
    unfold rewardDenominator
    simpa using Nat.mul_le_mul_right qualityScale inputs.rMaxAtLeastOne
  have hScaleLeMin :
      qualityScale ≤ min inputs.quality.toNat (rewardDenominator inputs) :=
    (Nat.le_min).2 ⟨qualityValid, hScaleLeCap⟩
  have hNumerator :
      inputs.subsidy.toNat * qualityScale ≤ rewardNumerator inputs := by
    unfold rewardNumerator
    exact Nat.mul_le_mul_left inputs.subsidy.toNat hScaleLeMin
  have hDivided :=
    Nat.div_le_div_right (c := rewardDenominator inputs) hNumerator
  calc
    inputs.subsidy.toNat / inputs.rMax =
        inputs.subsidy.toNat * qualityScale / rewardDenominator inputs := by
      unfold rewardDenominator
      exact
        (Nat.mul_div_mul_right inputs.subsidy.toNat inputs.rMax
          (by decide : 0 < qualityScale)).symm
    _ ≤ rewardNat inputs := by
      exact hDivided

/-- A threshold-valid solution earns exactly `floor(subsidy / R_MAX)`. -/
theorem reward_at_threshold
    (inputs : RewardInputs)
    (atThreshold : inputs.quality.toNat = qualityScale) :
    rewardNat inputs = inputs.subsidy.toNat / inputs.rMax := by
  have hScaleLeCap : qualityScale ≤ rewardDenominator inputs := by
    unfold rewardDenominator
    simpa using Nat.mul_le_mul_right qualityScale inputs.rMaxAtLeastOne
  unfold rewardNat rewardNumerator
  rw [atThreshold, Nat.min_eq_left hScaleLeCap]
  unfold rewardDenominator
  exact
    Nat.mul_div_mul_right inputs.subsidy.toNat inputs.rMax
      (by decide : 0 < qualityScale)

/-- A solution at or above the symbolic quality cap earns the full subsidy. -/
theorem reward_at_or_above_cap
    (inputs : RewardInputs)
    (atOrAboveCap : rewardDenominator inputs ≤ inputs.quality.toNat) :
    rewardNat inputs = inputs.subsidy.toNat := by
  unfold rewardNat rewardNumerator
  rw [Nat.min_eq_right atOrAboveCap]
  exact Nat.mul_div_cancel inputs.subsidy.toNat (reward_denominator_positive inputs)

/-- The `R_MAX = 1` boundary degenerates to full subsidy for every valid Q. -/
theorem reward_rMax_one
    (inputs : RewardInputs)
    (rMaxOne : inputs.rMax = 1)
    (qualityValid : qualityScale ≤ inputs.quality.toNat) :
    rewardNat inputs = inputs.subsidy.toNat := by
  apply reward_at_or_above_cap
  simpa [rewardDenominator, rMaxOne] using qualityValid

/-- The complete ratified reward range for any validity-threshold Q. -/
theorem reward_range
    (inputs : RewardInputs)
    (qualityValid : qualityScale ≤ inputs.quality.toNat) :
    inputs.subsidy.toNat / inputs.rMax ≤ rewardNat inputs ∧
      rewardNat inputs ≤ inputs.subsidy.toNat :=
  ⟨subsidy_div_rMax_le_reward inputs qualityValid, reward_le_subsidy inputs⟩

/-- The ceiling proof makes the checked conversion back to u64 total. -/
theorem reward_fits_u64 (inputs : RewardInputs) :
    rewardNat inputs < UInt64.size :=
  Nat.lt_of_le_of_lt (reward_le_subsidy inputs) inputs.subsidy.toNat_lt

/-- The exact checked-back-to-u64 reward credited by §8 step 3. -/
def rewardAmount (inputs : RewardInputs) : UInt64 :=
  UInt64.ofNatLT (rewardNat inputs) (reward_fits_u64 inputs)

@[simp] theorem rewardAmount_toNat (inputs : RewardInputs) :
    (rewardAmount inputs).toNat = rewardNat inputs := by
  unfold rewardAmount
  exact UInt64.toNat_ofNatLT

/--
Symbolic §8 dependencies. `blockRulesHold` stands only for B1–B8 and B11–B12.
The reward inputs are abstract values, but the credited amount is always derived by
the proved Model 4 function above.
-/
structure Context (State : Type) where
  txValidation : ValidationContext State
  blockRulesHold : Block → Bool
  rewardInputs : Block → RewardInputs

/-- The exact STF credit is bounded by its symbolic P-12 subsidy ceiling. -/
theorem credited_reward_le_subsidy
    (context : Context State)
    (block : Block) :
    (rewardAmount (context.rewardInputs block)).toNat ≤
      (context.rewardInputs block).subsidy.toNat := by
  rw [rewardAmount_toNat]
  exact reward_le_subsidy (context.rewardInputs block)

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

/-- Apply the derived §11 Model 4 reward with checked addition. -/
def applyReward
    (context : Context State)
    (ops : StateOps State)
    (block : Block)
    (state : State) : Except Error State := do
  let minerBefore := context.txValidation.account state block.minerAddress
  let reward := rewardAmount (context.rewardInputs block)
  let minerBalance ←
    requireSome Error.rewardCredit
      (checkedAdd minerBefore.balance reward)
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
Literal transcription of amended §8's conservation theorem target. The transition
and target use the same derived Model 4 reward. `reward_le_subsidy` proves separately
that realized issuance never exceeds the symbolic subsidy ceiling.
-/
def ConservationTarget
    (context : Context State)
    (ops : StateOps State)
    (preState : State)
    (block : Block) : Prop :=
  match applyBlockCandidate context ops preState block with
  | .ok postState =>
      ops.totalBalances postState =
        ops.totalBalances preState + rewardNat (context.rewardInputs block)
  | .error _ => True

end Cj3.Spec.Stf
