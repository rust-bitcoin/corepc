# corepc project instructions

## Bug-hunting is a first-class goal

One of the goals of the codegen `into_model` work is to **catch bugs in the existing
hand-written code** (mostly `types/`), especially places where the canonical `crate::model`
types convert Core's JSON into the wrong Rust type.

When you find (or strongly suspect) such a bug:

1. Confirm it. Research online (Core RPC docs, Core source, functional tests, example values)
   before calling it a bug. Do not flag on a hunch.
2. Log it in `/home/renato/Desktop/rust-bitcoin/corepc_bugs_backlog.md` as a new numbered entry:
   where (file:line), what (wrong vs correct), evidence, status, and how codegen handles it.
3. Leave the affected portion of the generated code **half-done**, do not paper over it. In the
   generated `into_model`:
   - Leave the field that would feed the wrong canonical type unimplemented (a `todo!()` or an
     equivalent placeholder that does not compile a wrong conversion).
   - Add a comment with **what you think the correct type/conversion is**.
   - Note **what the (buggy) canonical type currently expects to receive**, so the mismatch is
     visible at the call site.
   - Finish with a juicy `TODO` comment so each case can be referenced and fixed later,
     one by one.

We leverage rust-bitcoin types wherever possible. But when `types/` itself is wrong, we generate
what *looks correct*, comment it, record the canonical type's (wrong) expectation, and leave the
`TODO` rather than silently reproducing the bug.
