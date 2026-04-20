## 📐 Specification: [Module/Function Name] | smith-windows

**🎯 Purpose:** [1 sentence: why it's needed, what problem it solves]

**📥 Input:**
- `parameter` (type) | constraint | example

**📤 Output:**
- `result` (type) | side effect | example
- On error: error type | what DOES NOT change

**⚠️ Boundaries:**
- What if input is empty / `0` / `NaN` / max?
- What if called in an invalid state?

**✅ Success criteria:**
- [ ] All scenarios from "Boundaries" handled without panics
- [ ] State doesn't break on error
- [ ] Log/metric records result or failure cause

---
## 🗓️ Implementation plan (for `/plan`)
- [ ] Create/update file: `src/[module]/[file].rs`
- [ ] Implement types and functions with contracts
- [ ] Add tests: positive, boundary, negative
- [ ] Update docs: `ARCHITECTURE.md` or `ADR/XXX.md`
- [ ] Checks: `cargo test`, `cargo clippy -- -D warnings`
