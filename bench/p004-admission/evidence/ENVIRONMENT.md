# P-004 legacy calibration environment

- Captured UTC: 2026-08-16T14:55:49.5560003Z
- Host OS: Microsoft Windows 11 Home 10.0.26200 64-bit
- Processor: AMD Ryzen AI 9 HX 370 w/ Radeon 890M
- Logical processors: 24
- Cargo: cargo 1.97.1 (c980f4866 2026-06-30)
- Rust compiler:
    rustc 1.97.1 (8bab26f4f 2026-07-14)
    binary: rustc
    commit-hash: 8bab26f4f68e0e26f0bb7960be334d5b520ea452
    commit-date: 2026-07-14
    host: x86_64-pc-windows-msvc
    release: 1.97.1
    LLVM version: 22.1.6
- CJ3 frame/base revision: 413231616514f21a596dafc8c277d670051db15b
- CJ3 harness source SHA-256: 5FCDA553CDAF7A6B91CE7CCCA4A6D94863C47FCFBE0889B5488B984ADC78BD78
- Legacy driver source SHA-256: F5B44644EFCE3BEFFBCAEFC7B9CBB4A4938A372977C41645F3DC546010C330CB
- Orchestration script SHA-256: 2FEC4030C04C8C25F35AF84C788FEC4A7ED6BFBE655ED610C3CDDF141059454F
- Legacy source root: C:\Users\LEET\COINjecture2.0-network
- Legacy source revision: 58e0397fddd8e5ebd0d84fe00fbd022fae2b17ff
- Driver warmups/samples/checker repetitions: 3 / 21 / 10000
- Provisional checker comparison: 15,000,000 ns (P-003 recommendation; not G0-ratified)
- Generated driver lock SHA-256: C1A48885D64A8D85D22A3EE24807853BE3112C9569741B7D2C563D7628EC321D
- Raw JSONL SHA-256: 8A4B1590266F6CB0BD34D73FE4538B929215201F8957ADC7258DB8099CBDABCF
- Generated Markdown SHA-256: 07D0BA0ED8A4514CB68D073A48288C740023D019E9C87ABF21D6EC1E2CE79299
- Legacy source SHA-256 pins: Cargo.lock=9930A209663DD812D03DD654D5EA8F850152667DE455191B7C4645EB1CDB1BEA; consensus\src\miner.rs=3AA12C026FF8CCEEFB00CC42B9D1EC8BE44CEB7B8D5366AA09EC04CACB014BC5; consensus\src\problem_registry.rs=02D001745A17E0F48A8660BC64C515490B7ACC22845E6784872A7F199F9E7A4B; core\src\problem.rs=EDB039CFBFBFE46AC39D4B0DEB0465CC779C04FE5B17C5C8C7DBB3509DCDBE27

The driver compiled the exact local 2.0 path dependencies in an isolated temporary
Cargo project. Only the three executable mining variants were timed. Descriptor-only
registry entries and the Custom payload were emitted as explicit unmeasured records.
Checker observations are per-operation integer averages from bounded timed batches;
the batch size is recorded above and in every JSONL row.
