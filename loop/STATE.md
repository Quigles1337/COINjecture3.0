# COINjecture 3.0 Loop State

CYCLE: 0
PHASE: Cycle 0 scaffold
PACKET: NONE
BRANCH: master
REMOTE: https://github.com/Quigles1337/COINjecture3.0
CAPACITY_FLAG: unconfirmed
STATUS: STOPPED_PENDING_AL

## Ground-truth state

- Cycle 0 scaffold is the only authorized work in this cycle.
- No protocol code has been created.
- The packet queue is not approved.
- The COINjecture 2.0 capacity state for the next packet pickup was not supplied in
  this session. `CAPACITY_FLAG: unconfirmed` therefore prohibits packet pickup; it
  MUST NOT be interpreted as capacity clearance.

## Resume conditions

Before any packet begins, Al must:

1. ratify or revise D1, D2, D14, and D15;
2. rule on proposed D16;
3. approve or revise the queue in `loop/PACKETS.md`;
4. state the current COINjecture 2.0 capacity condition required by D11; and
5. identify the approved packet and branch.

P-008 additionally requires the exact public frontend repository URL from Al. Agents
MUST NOT infer that URL from the legacy COINjecture 2.0 repository.
