# rp2350-sim
The goal is to emulate a rp2350, for a Raspberry Pi Pico 2. Built in Rust, so it can run natively as well in wasm.

## Progress
Currently attempts are being made to replicate the original Raspberry Pi Pico. This means creating the Cortex M0+, then migrating this over to emulate the Cortex M33

Implemented instructions

- [x] AdcT1
- [x] AddSpPlusImmediateT1
- [x] AddSpPlusImmediateT2
- [x] AddsT1
- [x] AddsT2
- [x] AddRegisterT1
- [x] AddRegisterT2
- [x] AdrT1
- [x] AndRegisterT1
- [x] AsrImmediateT1
- [x] AsrRegisterT1
- [x] BT1
- [x] BT2
- [x] BicRegisterT1
- [ ] BkptT1
- [x] BlT1
- [x] BlxT1
- [ ] BxT1
- [ ] CmnRegisterT1
- [ ] CmpImmediateT1
- [ ] CmpRegisterT1
- [ ] CmpRegisterT2
- [ ] CpsT1Id
- [ ] CpsT1Ie
- [x] DmbT1Sy
- [x] DsbT1Sy
- [x] EorRegisterT1
- [x] IsbT1Sy
- [x] LdmiaT1
- [ ] LdrImmediateT1
- [ ] LdrImmediateT2
- [ ] LdrLiteralT1
- [ ] LdrRegisterT1
- [ ] LdrbImmediateT1
- [ ] LdrbRegisterT1
- [ ] LdrhImmediateT1
- [ ] LdrhRegisterT1
- [ ] LdrsbRegisterT1
- [ ] LdrshRegisterT1
- [ ] LslImmediateT1
- [ ] LslRegisterT1
- [ ] LsrImmediateT1
- [ ] LsrRegisterT1
- [x] MovRegisterT1
- [ ] MovImmediateT1
- [ ] MrsT1
- [ ] MsrT1
- [ ] MulT1
- [ ] MvnT1
- [ ] OrrRegisterT1
- [ ] PopT1
- [x] PushT1
- [x] RevT1
- [x] Rev16T1
- [ ] RevshT1
- [ ] RorRegisterT1
- [ ] RsbImmediateT1
- [x] NopT1
- [ ] SbcRegisterT1
- [ ] SevT1
- [x] StmiaT1
- [ ] StrImmediateT1
- [ ] StrImmediateT2
- [ ] StrRegisterT1
- [ ] StrbImmediateT1
- [ ] StrbRegisterT1
- [ ] StrhImmediateT1
- [ ] StrhRegisterT1
- [x] SubSpMinusImmediateT1
- [ ] SubT1
- [ ] SubT2
- [ ] SubRegisterT1
- [ ] SvcT1
- [ ] SxtbT1
- [ ] SxthT1
- [ ] TstRegisterT1
- [ ] UdfT1
- [ ] UdfT2
- [x] UxtbT1
- [x] UxthT1
- [ ] WfeT1
- [ ] WfiT1
- [x] YieldT1


