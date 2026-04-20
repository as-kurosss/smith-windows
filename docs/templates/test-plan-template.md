## 🧪 Test Plan: [Name] | smith-windows

**✅ Positive:** [input] → [expected output]
**🔄 Boundary:** `0` / empty / max → [expected]
**❌ Negative:** Invalid input → [expected error]

**🔍 Mandatory checks:**
- [ ] On `Err`, state unchanged
- [ ] No duplicate events/logs
- [ ] No `unwrap()`, `panic!`, or blocking in async

---
## 🗓️ For `/plan`: tests as steps
- [ ] Create test file
- [ ] Implement tests: base, boundary, negative
- [ ] Run `cargo test -- --nocapture`
