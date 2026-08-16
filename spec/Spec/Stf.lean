/-
NORMATIVE STATUS: RATIFIED — human-reviewed; formal-verification ownership reserved per LEDGER D16

Human-ratified model of Protocol Spec §8 state-transition sequencing, checked arithmetic,
block-granularity atomicity, and the conservation theorem target.

B-rule validation, the subsidy schedule, authenticated-state storage, and root
computation remain abstract interfaces because their concrete definitions are outside
the P-005 authority surface. The reward itself is derived from ratified SI-004 Model 4
without selecting `R_MAX` or any curve-shaping value. No Al-owned, Sarah-owned,
G0-controlled, SI-001, SI-002, or SI-003 choice is instantiated here.
-/

import Spec.Tx
import Init.Omega

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

/--
The exact state-operation laws P-101 must prove for its concrete authenticated store.

The additive replacement equation is deliberately local: it relates one `setAccount`
call to the account value it replaces and therefore does not assume conservation.
Together with read-after-write and read-other-address, it is sufficient to prove the
STF's conservation result for every sender/recipient/miner aliasing pattern.
-/
structure LawfulStateOps
    (context : Context State)
    (ops : StateOps State) : Prop where
  totalBalances_setAccount
      (state : State)
      (address : Bytes)
      (replacement : Account) :
    ops.totalBalances (ops.setAccount state address replacement) +
        (context.txValidation.account state address).balance.toNat =
      ops.totalBalances state + replacement.balance.toNat
  account_set_same
      (state : State)
      (address : Bytes)
      (replacement : Account) :
    context.txValidation.account
        (ops.setAccount state address replacement) address = replacement
  account_set_other
      (state : State)
      (writtenAddress otherAddress : Bytes)
      (replacement : Account)
      (different : otherAddress ≠ writtenAddress) :
    context.txValidation.account
        (ops.setAccount state writtenAddress replacement) otherAddress =
      context.txValidation.account state otherAddress

/-- Replacing a balance with the result of checked subtraction removes exactly `delta`. -/
theorem totalBalances_after_checkedSub
    {State : Type}
    (context : Context State)
    (ops : StateOps State)
    (lawful : LawfulStateOps context ops)
    (state : State)
    (address : Bytes)
    (delta newBalance : UInt64)
    (checked :
      checkedSub
          (context.txValidation.account state address).balance delta =
        some newBalance) :
    ops.totalBalances
          (ops.setAccount state address
            { context.txValidation.account state address with
              balance := newBalance }) +
        delta.toNat =
      ops.totalBalances state := by
  have hLaw :=
    lawful.totalBalances_setAccount state address
      { context.txValidation.account state address with balance := newBalance }
  have hExact := checkedSub_eq_some_toNat checked
  change
    ops.totalBalances
          (ops.setAccount state address
            { context.txValidation.account state address with
              balance := newBalance }) +
        (context.txValidation.account state address).balance.toNat =
      ops.totalBalances state + newBalance.toNat at hLaw
  omega

/-- Replacing a balance with the result of checked addition adds exactly `delta`. -/
theorem totalBalances_after_checkedAdd
    {State : Type}
    (context : Context State)
    (ops : StateOps State)
    (lawful : LawfulStateOps context ops)
    (state : State)
    (address : Bytes)
    (delta newBalance : UInt64)
    (checked :
      checkedAdd
          (context.txValidation.account state address).balance delta =
        some newBalance) :
    ops.totalBalances
        (ops.setAccount state address
          { context.txValidation.account state address with
            balance := newBalance }) =
      ops.totalBalances state + delta.toNat := by
  have hLaw :=
    lawful.totalBalances_setAccount state address
      { context.txValidation.account state address with balance := newBalance }
  have hExact := checkedAdd_eq_some_toNat checked
  change
    ops.totalBalances
          (ops.setAccount state address
            { context.txValidation.account state address with
              balance := newBalance }) +
        (context.txValidation.account state address).balance.toNat =
      ops.totalBalances state + newBalance.toNat at hLaw
  omega

/-- Updating only an account nonce preserves total balances. -/
theorem totalBalances_after_nonce
    {State : Type}
    (context : Context State)
    (ops : StateOps State)
    (lawful : LawfulStateOps context ops)
    (state : State)
    (address : Bytes)
    (newNonce : UInt64) :
    ops.totalBalances
        (ops.setAccount state address
          { context.txValidation.account state address with nonce := newNonce }) =
      ops.totalBalances state := by
  have hLaw :=
    lawful.totalBalances_setAccount state address
      { context.txValidation.account state address with nonce := newNonce }
  change
    ops.totalBalances
          (ops.setAccount state address
            { context.txValidation.account state address with nonce := newNonce }) +
        (context.txValidation.account state address).balance.toNat =
      ops.totalBalances state +
        (context.txValidation.account state address).balance.toNat at hLaw
  omega

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

/-- Apply the already validated transaction with sequential reads and checked writes. -/
def applyValidatedTransaction
    (context : Context State)
    (ops : StateOps State)
    (minerAddress : Bytes)
    (state : State)
    (tx : Candidate) : Except Error State :=
  match checkedAdd tx.amount tx.fee with
  | none => .error Error.senderDebit
  | some debit =>
      let senderBefore := context.txValidation.account state tx.sender
      match checkedSub senderBefore.balance debit with
      | none => .error Error.senderDebit
      | some senderBalance =>
          let afterDebit :=
            ops.setAccount state tx.sender
              { senderBefore with balance := senderBalance }
          let recipientBefore :=
            context.txValidation.account afterDebit tx.recipient
          match checkedAdd recipientBefore.balance tx.amount with
          | none => .error Error.recipientCredit
          | some recipientBalance =>
              let afterRecipient :=
                ops.setAccount afterDebit tx.recipient
                  { recipientBefore with balance := recipientBalance }
              let minerBefore :=
                context.txValidation.account afterRecipient minerAddress
              match checkedAdd minerBefore.balance tx.fee with
              | none => .error Error.minerFeeCredit
              | some minerBalance =>
                  let afterMiner :=
                    ops.setAccount afterRecipient minerAddress
                      { minerBefore with balance := minerBalance }
                  let senderBeforeNonce :=
                    context.txValidation.account afterMiner tx.sender
                  match checkedAdd senderBeforeNonce.nonce 1 with
                  | none => .error Error.senderNonceIncrement
                  | some senderNonce =>
                      .ok <|
                        ops.setAccount afterMiner tx.sender
                          { senderBeforeNonce with nonce := senderNonce }

/-- The checked mutation sequence transfers value without minting or burning it. -/
private theorem applyValidatedTransaction_conserves_core
    {State : Type}
    (context : Context State)
    (ops : StateOps State)
    (lawful : LawfulStateOps context ops)
    (minerAddress : Bytes)
    (state postState : State)
    (tx : Candidate)
    (applied :
      applyValidatedTransaction context ops minerAddress state tx = .ok postState) :
    ops.totalBalances postState = ops.totalBalances state := by
  unfold applyValidatedTransaction at applied
  cases hDebit : checkedAdd tx.amount tx.fee with
  | none =>
      simp [hDebit] at applied
  | some debit =>
      let senderBefore := context.txValidation.account state tx.sender
      cases hSenderBalance : checkedSub senderBefore.balance debit with
      | none =>
          simp [hDebit, senderBefore, hSenderBalance] at applied
      | some senderBalance =>
          let afterDebit :=
            ops.setAccount state tx.sender
              { senderBefore with balance := senderBalance }
          let recipientBefore :=
            context.txValidation.account afterDebit tx.recipient
          cases hRecipientBalance :
              checkedAdd recipientBefore.balance tx.amount with
          | none =>
              simp [hDebit, senderBefore, hSenderBalance, afterDebit,
                recipientBefore, hRecipientBalance] at applied
          | some recipientBalance =>
              let afterRecipient :=
                ops.setAccount afterDebit tx.recipient
                  { recipientBefore with balance := recipientBalance }
              let minerBefore :=
                context.txValidation.account afterRecipient minerAddress
              cases hMinerBalance : checkedAdd minerBefore.balance tx.fee with
              | none =>
                  simp [hDebit, senderBefore, hSenderBalance, afterDebit,
                    recipientBefore, hRecipientBalance, afterRecipient,
                    minerBefore, hMinerBalance] at applied
              | some minerBalance =>
                  let afterMiner :=
                    ops.setAccount afterRecipient minerAddress
                      { minerBefore with balance := minerBalance }
                  let senderBeforeNonce :=
                    context.txValidation.account afterMiner tx.sender
                  cases hSenderNonce : checkedAdd senderBeforeNonce.nonce 1 with
                  | none =>
                      simp [hDebit, senderBefore, hSenderBalance, afterDebit,
                        recipientBefore, hRecipientBalance, afterRecipient,
                        minerBefore, hMinerBalance, afterMiner,
                        senderBeforeNonce, hSenderNonce] at applied
                  | some senderNonce =>
                      let finalState :=
                        ops.setAccount afterMiner tx.sender
                          { senderBeforeNonce with nonce := senderNonce }
                      have hPost : finalState = postState := by
                        simpa [hDebit, senderBefore, hSenderBalance, afterDebit,
                          recipientBefore, hRecipientBalance, afterRecipient,
                          minerBefore, hMinerBalance, afterMiner,
                          senderBeforeNonce, hSenderNonce, finalState] using applied
                      have hDebitTotals :
                          ops.totalBalances afterDebit + debit.toNat =
                            ops.totalBalances state := by
                        dsimp [afterDebit, senderBefore]
                        exact
                          totalBalances_after_checkedSub context ops lawful
                            state tx.sender debit senderBalance
                            (by simpa [senderBefore] using hSenderBalance)
                      have hRecipientTotals :
                          ops.totalBalances afterRecipient =
                            ops.totalBalances afterDebit + tx.amount.toNat := by
                        dsimp [afterRecipient, recipientBefore]
                        exact
                          totalBalances_after_checkedAdd context ops lawful
                            afterDebit tx.recipient tx.amount recipientBalance
                            (by simpa [recipientBefore] using hRecipientBalance)
                      have hMinerTotals :
                          ops.totalBalances afterMiner =
                            ops.totalBalances afterRecipient + tx.fee.toNat := by
                        dsimp [afterMiner, minerBefore]
                        exact
                          totalBalances_after_checkedAdd context ops lawful
                            afterRecipient minerAddress tx.fee minerBalance
                            (by simpa [minerBefore] using hMinerBalance)
                      have hNonceTotals :
                          ops.totalBalances finalState =
                            ops.totalBalances afterMiner := by
                        dsimp [finalState, senderBeforeNonce]
                        exact
                          totalBalances_after_nonce context ops lawful
                            afterMiner tx.sender senderNonce
                      have hDebitExact :
                          debit.toNat = tx.amount.toNat + tx.fee.toNat :=
                        checkedAdd_eq_some_toNat hDebit
                      rw [← hPost]
                      omega

/--
Apply one transaction after re-running V1–V9 against the current candidate state.
The explicit `minerAddress` argument comes from the enclosing block.
-/
def applyTransaction
    (context : Context State)
    (ops : StateOps State)
    (minerAddress : Bytes)
    (state : State)
    (input : Bytes) : Except Error State :=
  match Tx.validate context.txValidation state input with
  | .error cause => .error (Error.txInvalid cause)
  | .ok tx => applyValidatedTransaction context ops minerAddress state tx

/-- Exhaustive realizable equality partitions for sender, recipient, and miner. -/
inductive AddressAliasing
    (sender recipient miner : Bytes) : Prop where
  | allDistinct
      (senderRecipient : sender ≠ recipient)
      (senderMiner : sender ≠ miner)
      (recipientMiner : recipient ≠ miner)
  | senderRecipient
      (same : sender = recipient)
      (minerDifferent : sender ≠ miner)
  | senderMiner
      (same : sender = miner)
      (recipientDifferent : sender ≠ recipient)
  | recipientMiner
      (same : recipient = miner)
      (senderDifferent : sender ≠ recipient)
  | allThree
      (senderRecipient : sender = recipient)
      (senderMiner : sender = miner)

/-- Equality is decidable for abstract bytes, so the alias partition is exhaustive. -/
theorem classifyAddressAliasing
    (sender recipient miner : Bytes) :
    AddressAliasing sender recipient miner := by
  by_cases hSenderRecipient : sender = recipient
  · by_cases hSenderMiner : sender = miner
    · exact .allThree hSenderRecipient hSenderMiner
    · exact .senderRecipient hSenderRecipient hSenderMiner
  · by_cases hSenderMiner : sender = miner
    · exact .senderMiner hSenderMiner hSenderRecipient
    · by_cases hRecipientMiner : recipient = miner
      · exact .recipientMiner hRecipientMiner hSenderRecipient
      · exact .allDistinct hSenderRecipient hSenderMiner hRecipientMiner

/--
Every successfully applied transaction conserves value under the P-101 store laws.

The case split is intentionally explicit: it discharges the all-distinct,
sender=recipient, sender=miner, recipient=miner, and all-three-equal executions of the
same sequential-read transition. No pairwise-distinct premise is hidden in the core
accounting proof.
-/
theorem applyTransaction_conserves
    {State : Type}
    (context : Context State)
    (ops : StateOps State)
    (lawful : LawfulStateOps context ops)
    (minerAddress : Bytes)
    (state postState : State)
    (input : Bytes)
    (applied :
      applyTransaction context ops minerAddress state input = .ok postState) :
    ops.totalBalances postState = ops.totalBalances state := by
  cases hValidate : Tx.validate context.txValidation state input with
  | error cause =>
      simp [applyTransaction, hValidate] at applied
  | ok tx =>
      have hAppliedValidated :
          applyValidatedTransaction context ops minerAddress state tx =
            .ok postState := by
        simpa [applyTransaction, hValidate] using applied
      have hCore :=
        applyValidatedTransaction_conserves_core
          context ops lawful minerAddress state postState tx hAppliedValidated
      cases classifyAddressAliasing tx.sender tx.recipient minerAddress with
      | allDistinct _ _ _ => exact hCore
      | senderRecipient _ _ => exact hCore
      | senderMiner _ _ => exact hCore
      | recipientMiner _ _ => exact hCore
      | allThree _ _ => exact hCore

/-- Ordered fold: each transaction is validated against the state produced so far. -/
def applyTransactions
    (context : Context State)
    (ops : StateOps State)
    (minerAddress : Bytes) :
    State → List Bytes → Except Error State
  | state, [] => .ok state
  | state, input :: rest =>
      match applyTransaction context ops minerAddress state input with
      | .error cause => .error cause
      | .ok next => applyTransactions context ops minerAddress next rest

/-- A successful ordered transaction fold preserves total balances. -/
theorem applyTransactions_conserves
    {State : Type}
    (context : Context State)
    (ops : StateOps State)
    (lawful : LawfulStateOps context ops)
    (minerAddress : Bytes)
    (state postState : State)
    (inputs : List Bytes)
    (applied :
      applyTransactions context ops minerAddress state inputs = .ok postState) :
    ops.totalBalances postState = ops.totalBalances state := by
  induction inputs generalizing state with
  | nil =>
      simp [applyTransactions] at applied
      subst postState
      rfl
  | cons input rest ih =>
      cases hFirst : applyTransaction context ops minerAddress state input with
      | error cause =>
          simp [applyTransactions, hFirst] at applied
      | ok nextState =>
          have hRest :
              applyTransactions context ops minerAddress nextState rest =
                .ok postState := by
            simpa [applyTransactions, hFirst] using applied
          calc
            ops.totalBalances postState = ops.totalBalances nextState :=
              ih nextState hRest
            _ = ops.totalBalances state :=
              applyTransaction_conserves
                context ops lawful minerAddress state nextState input hFirst

/-- Apply the derived §11 Model 4 reward with checked addition. -/
def applyReward
    (context : Context State)
    (ops : StateOps State)
    (block : Block)
    (state : State) : Except Error State :=
  let minerBefore := context.txValidation.account state block.minerAddress
  let reward := rewardAmount (context.rewardInputs block)
  match checkedAdd minerBefore.balance reward with
  | none => .error Error.rewardCredit
  | some minerBalance =>
      .ok <|
        ops.setAccount state block.minerAddress
          { minerBefore with balance := minerBalance }

/-- A successful reward credit adds exactly the proved Model 4 reward. -/
theorem applyReward_conserves
    {State : Type}
    (context : Context State)
    (ops : StateOps State)
    (lawful : LawfulStateOps context ops)
    (block : Block)
    (state postState : State)
    (applied : applyReward context ops block state = .ok postState) :
    ops.totalBalances postState =
      ops.totalBalances state + rewardNat (context.rewardInputs block) := by
  let minerBefore := context.txValidation.account state block.minerAddress
  let reward := rewardAmount (context.rewardInputs block)
  cases hMinerBalance : checkedAdd minerBefore.balance reward with
  | none =>
      simp [applyReward, minerBefore, reward, hMinerBalance] at applied
  | some minerBalance =>
      let finalState :=
        ops.setAccount state block.minerAddress
          { minerBefore with balance := minerBalance }
      have hPost : finalState = postState := by
        simpa [applyReward, minerBefore, reward, hMinerBalance, finalState] using applied
      have hTotals :
          ops.totalBalances finalState =
            ops.totalBalances state + reward.toNat := by
        dsimp [finalState, minerBefore]
        exact
          totalBalances_after_checkedAdd context ops lawful
            state block.minerAddress reward minerBalance
            (by simpa [minerBefore, reward] using hMinerBalance)
      rw [← hPost, hTotals]
      exact congrArg (ops.totalBalances state + ·)
        (rewardAmount_toNat (context.rewardInputs block))

/--
Construct the complete post-state candidate without committing it. Every error
discards this internal candidate at the atomic wrapper.
-/
def applyBlockCandidate
    (context : Context State)
    (ops : StateOps State)
    (state : State)
    (block : Block) : Except Error State :=
  match context.blockRulesHold block with
  | false => .error Error.blockRules
  | true =>
      match
        applyTransactions
          context ops block.minerAddress state block.transactions with
      | .error cause => .error cause
      | .ok afterTransactions =>
          match applyReward context ops block afterTransactions with
          | .error cause => .error cause
          | .ok afterReward =>
              match
                ops.stateRootMatches afterReward block.expectedStateRoot with
              | false => .error Error.stateRootMismatch
              | true => .ok afterReward

/--
A successfully constructed block candidate mints exactly its derived Model 4 reward.
Transaction fees cancel inside `applyTransactions_conserves`; no other value source is
available to this transition.
-/
theorem applyBlockCandidate_conserves
    {State : Type}
    (context : Context State)
    (ops : StateOps State)
    (lawful : LawfulStateOps context ops)
    (state postState : State)
    (block : Block)
    (applied : applyBlockCandidate context ops state block = .ok postState) :
    ops.totalBalances postState =
      ops.totalBalances state + rewardNat (context.rewardInputs block) := by
  cases hRules : context.blockRulesHold block with
  | false =>
      simp [applyBlockCandidate, hRules] at applied
  | true =>
      cases hTransactions :
          applyTransactions
            context ops block.minerAddress state block.transactions with
      | error cause =>
          simp [applyBlockCandidate, hRules, hTransactions] at applied
      | ok afterTransactions =>
          cases hReward : applyReward context ops block afterTransactions with
          | error cause =>
              simp [applyBlockCandidate, hRules, hTransactions, hReward] at applied
          | ok afterReward =>
              cases hRoot :
                  ops.stateRootMatches afterReward block.expectedStateRoot with
              | false =>
                  simp [applyBlockCandidate, hRules, hTransactions, hReward,
                    hRoot] at applied
              | true =>
                  have hPost : afterReward = postState := by
                    simpa [applyBlockCandidate, hRules, hTransactions, hReward,
                      hRoot] using applied
                  have hTransactionTotals :=
                    applyTransactions_conserves
                      context ops lawful block.minerAddress state
                        afterTransactions block.transactions hTransactions
                  have hRewardTotals :=
                    applyReward_conserves
                      context ops lawful block afterTransactions afterReward hReward
                  rw [← hPost]
                  calc
                    ops.totalBalances afterReward =
                        ops.totalBalances afterTransactions +
                          rewardNat (context.rewardInputs block) := hRewardTotals
                    _ = ops.totalBalances state +
                          rewardNat (context.rewardInputs block) := by
                      rw [hTransactionTotals]

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
Literal transcription of amended §8's conservation property. The transition and
property use the same derived Model 4 reward. `reward_le_subsidy` proves separately
that realized issuance never exceeds the symbolic subsidy ceiling, and
`conservationTarget_holds` discharges this property from `LawfulStateOps` below.
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

/-- The amended §8 conservation property is proved for every candidate outcome. -/
theorem conservationTarget_holds
    {State : Type}
    (context : Context State)
    (ops : StateOps State)
    (lawful : LawfulStateOps context ops)
    (preState : State)
    (block : Block) :
    ConservationTarget context ops preState block := by
  cases hCandidate : applyBlockCandidate context ops preState block with
  | error cause =>
      simp [ConservationTarget, hCandidate]
  | ok postState =>
      simpa [ConservationTarget, hCandidate] using
        applyBlockCandidate_conserves
          context ops lawful preState postState block hCandidate

end Cj3.Spec.Stf
