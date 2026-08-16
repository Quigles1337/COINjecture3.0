/-
NORMATIVE STATUS: draft — pending human ratification; formal-verification ownership reserved per LEDGER D16.
-/

/-!
Draft §14 vector-case manifest.

Every `input_bytes` entry is a symbolic expression behind the abstract P-007/G0
codec interface. It is not a byte string and cannot be consumed as a canonical wire
vector. This prevents P-005 from resolving canonical encoding or SI-002/SI-003 by
fixture. No Al-owned, Sarah-owned, or G0-controlled parameter value appears here.
-/

import Lean.Data.Json
import Spec.Stf

namespace CJ3.Spec.Vectors

open Lean

structure Expected where
  accepted : Bool
  rule : String
  stateOutcome : String := "not_applicable"

structure DraftVector where
  name : String
  inputExpression : String
  interfaceRef : String
  expected : Expected

private def accept (rule : String) (stateOutcome := "not_applicable") : Expected :=
  { accepted := true, rule, stateOutcome }

private def reject (rule : String) (stateOutcome := "not_applicable") : Expected :=
  { accepted := false, rule, stateOutcome }

/--
The ratified §14 case list, expressed without canonical bytes or owned parameters.
`fee_min`, `tx_max_bytes`, `chain_network_id`, `reward`, and account values are
symbols supplied by the eventual ratified interface instantiation.
-/
def cases : List DraftVector := [
  {
    name := "v1-v8-baseline-pass"
    inputExpression := "TxCodec.encode(valid_tx_at_all_symbolic_boundaries)"
    interfaceRef := "V1 TxCodec (P-007/G0); cryptography/address interfaces abstract"
    expected := accept "V1-V8"
  },
  {
    name := "v1-size-at-limit-pass"
    inputExpression := "canonical_input_with_length(tx_max_bytes)"
    interfaceRef := "V1 TxCodec + symbolic TX_MAX_BYTES (P-8/G0)"
    expected := accept "V1"
  },
  {
    name := "v1-size-over-limit-reject"
    inputExpression := "canonical_input_with_length(tx_max_bytes + 1)"
    interfaceRef := "V1 TxCodec + symbolic TX_MAX_BYTES (P-8/G0)"
    expected := reject "V1"
  },
  {
    name := "v1-unknown-field-reject"
    inputExpression := "noncanonical_input_with_unknown_field"
    interfaceRef := "V1 TxCodec (P-007/G0); no concrete encoding selected"
    expected := reject "V1"
  },
  {
    name := "v2-signature-mismatch-reject"
    inputExpression := "canonical_tx_with_verify_signature_false"
    interfaceRef := "V2 signature/message interfaces (D15; canonical bytes abstract)"
    expected := reject "V2"
  },
  {
    name := "v3-address-binding-mismatch-reject"
    inputExpression := "canonical_tx_where_from_ne_addr(pubkey)"
    interfaceRef := "V3 address-derivation interface (single addr; P-007/G0)"
    expected := reject "V3"
  },
  {
    name := "v4-nonce-equal-pass"
    inputExpression := "canonical_tx_where_nonce_eq_state_nonce"
    interfaceRef := "V4 state account projection"
    expected := accept "V4"
  },
  {
    name := "v4-nonce-one-below-reject"
    inputExpression := "canonical_tx_where_nonce_plus_1_eq_state_nonce"
    interfaceRef := "V4 state account projection"
    expected := reject "V4"
  },
  {
    name := "v4-nonce-one-above-reject"
    inputExpression := "canonical_tx_where_nonce_eq_state_nonce_plus_1"
    interfaceRef := "V4 state account projection"
    expected := reject "V4"
  },
  {
    name := "v5-fee-at-minimum-pass"
    inputExpression := "canonical_tx_where_fee_eq_fee_min"
    interfaceRef := "V5 symbolic FEE_MIN (P-9; owner Al)"
    expected := accept "V5"
  },
  {
    name := "v5-fee-below-minimum-reject"
    inputExpression := "canonical_tx_where_fee_plus_1_eq_fee_min"
    interfaceRef := "V5 symbolic FEE_MIN (P-9; owner Al)"
    expected := reject "V5"
  },
  {
    name := "v6-exact-balance-pass"
    inputExpression := "canonical_tx_where_balance_eq_checked_add(amount,fee)"
    interfaceRef := "V6 checked-u64 arithmetic"
    expected := accept "V6"
  },
  {
    name := "v6-insufficient-by-one-reject"
    inputExpression := "canonical_tx_where_balance_plus_1_eq_checked_add(amount,fee)"
    interfaceRef := "V6 checked-u64 arithmetic"
    expected := reject "V6"
  },
  {
    name := "v6-u64-max-plus-one-reject"
    inputExpression := "canonical_tx_where_amount_eq_u64_max_and_fee_is_positive_and_ge_fee_min"
    interfaceRef := "V6 definitional u64::MAX neighborhood; symbolic FEE_MIN remains unfilled"
    expected := reject "V6"
  },
  {
    name := "v6-u64-max-boundary-pass"
    inputExpression := "canonical_tx_where_fee_eq_fee_min_and_amount_eq_u64_max_minus_fee_min_and_balance_eq_u64_max"
    interfaceRef := "V6 definitional u64::MAX neighborhood; symbolic FEE_MIN remains unfilled"
    expected := accept "V6"
  },
  {
    name := "v7-destination-32-bytes-pass"
    inputExpression := "canonical_tx_where_to_length_eq_32"
    interfaceRef := "V7 address shape"
    expected := accept "V7"
  },
  {
    name := "v7-destination-31-bytes-reject"
    inputExpression := "decoded_candidate_where_to_length_eq_31"
    interfaceRef := "V7 address shape; V1 interface supplies candidate for rule isolation"
    expected := reject "V7"
  },
  {
    name := "v8-network-match-pass"
    inputExpression := "canonical_tx_where_network_id_eq_chain_network_id"
    interfaceRef := "V8 symbolic chain network_id"
    expected := accept "V8"
  },
  {
    name := "v8-network-mismatch-reject"
    inputExpression := "canonical_tx_where_network_id_ne_chain_network_id"
    interfaceRef := "V8 symbolic chain network_id"
    expected := reject "V8"
  },
  {
    name := "v9-self-send-pass-and-fee-applies"
    inputExpression := "canonical_valid_tx_where_from_eq_to"
    interfaceRef := "V9 + ordered STF; all owned parameters symbolic"
    expected := accept "V9" "applied_with_fee_transfer_and_nonce_increment"
  },
  {
    name := "block-atomicity-late-transaction-failure"
    inputExpression := "block(valid_tx_0, tx_1_rejected_by_V4)"
    interfaceRef := "§8 evolving-state revalidation"
    expected := reject "V4" "exact_pre_block_state"
  },
  {
    name := "block-atomicity-reward-credit-overflow"
    inputExpression := "block_where_symbolic_reward_credit_overflows_u64"
    interfaceRef := "§8 reward interface (P-12/owners) + checked-u64 arithmetic"
    expected := reject "STF_CHECKED_REWARD" "exact_pre_block_state"
  }
]

private def expectedJson (expected : Expected) : Json :=
  Json.mkObj [
    ("accepted", Json.bool expected.accepted),
    ("rule", Json.str expected.rule),
    ("state", Json.str expected.stateOutcome)
  ]

private def vectorJson (vector : DraftVector) : Json :=
  Json.mkObj [
    ("name", Json.str vector.name),
    ("input_bytes", Json.mkObj [
      ("status", Json.str "NON-NORMATIVE TEST FIXTURE — ABSTRACT INTERFACE, NOT CANONICAL BYTES"),
      ("expression", Json.str vector.inputExpression),
      ("interface", Json.str vector.interfaceRef),
      ("normative", Json.bool false)
    ]),
    ("expected", expectedJson vector.expected)
  ]

def json : Json := Json.arr (cases.map vectorJson).toArray

def render : String := json.pretty

end CJ3.Spec.Vectors
