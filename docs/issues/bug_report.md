# Bug Report - Archive Keeper

Analysis of the current implementation against [SPECS.md](../SPECS.md), covering the
fix pass in commits `5a16ceb..07b556b`.

- Repository: `FranklinEkemezie/archive_keeper`
- Analyzed commit: `07b556b`
- Test command: `cargo test`
- Test status: **15 passed, 3 failed**

The three failing tests are intentional: they expose the open bugs below and should
not be weakened. Source code was not changed while preparing this report.

Failing tests:

1. `eof_on_stdin_does_not_leave_the_application_looping` - GitHub #10
2. `archive_can_hold_more_than_two_documents` - GitHub #11
3. `invalid_category_selection_is_rejected` - GitHub #12

---

## Open Issue 1 - EOF causes an infinite empty-input loop

**GitHub:** #10  
**Severity:** High (hang, unbounded output)  
**Location:** `ui_prompt_user()`, `src/main.rs:134-157`  
**Introduced by:** commit `db09976` ("fix: reject empty input at the prompt")

### Description

`ui_prompt_user()` now rejects empty input. When stdin reaches EOF, `read_line`
returns `Ok(0)` with an empty buffer, which is treated as a retryable empty value.
Because EOF is permanent, the loop never receives different input.

### Reproduction

```bash
cargo run < /dev/null
```

The application prints `[ERROR] User entered an empty value` forever and never exits.
In a 2-second run with stdin redirected from `/dev/null`, the process produced about
209,000 error lines.

Automated reproduction:

```bash
cargo test eof_on_stdin_does_not_leave_the_application_looping
```

### Expected

EOF should be handled as end-of-input (for example, exit cleanly) rather than being
retried. The read result should distinguish `Ok(0)` from an empty line.

---

## Open Issue 2 - Archive capacity was reduced to two documents

**GitHub:** #11  
**Severity:** High (capacity regression)  
**Location:** `MAX_NUM_OF_DOCUMENT`, `src/main.rs:39`, and `INITIAL_DOCUMENTS`,
`src/main.rs:45-56`  
**Introduced by:** commit `77cb897` ("fix: reject registrations when the archive is full")

### Description

The capacity was reduced from 10 to 2 and eight placeholder entries were commented
out. After two registrations, every further attempt prints `Archive is full!`.

The out-of-bounds crash is fixed, but the new limit is unrealistically small. The
specification does not define a two-document archive and talks about registering
"several documents" and displaying several documents readably.

### Reproduction

Register two documents, then choose option `1` again.

```text
Document registered successfully with ID: DOC-101-000
Document registered successfully with ID: DOC-102-001
Archive is full!
```

Automated reproduction:

```bash
cargo test archive_can_hold_more_than_two_documents
```

### Expected

Restore a realistic capacity (at least the previous 10) or use `Vec<Document>` so the
archive can grow. Two documents is almost certainly a temporary test value.

---

## Open Issue 3 - Category menu accepts 0 and then panics

**GitHub:** #12  
**Severity:** High (panic on invalid user input)  
**Location:** `ui_prompt_user_option()` `src/main.rs:185-196`, and
`document_category_prompt_category()` `src/main.rs:608-634` (panic at line 631)

### Description

The option prompt accepts any value less than or equal to `max_option`, so `0` passes
for the category menu. The category handler only matches `1..=4` and panics on any
other value.

### Reproduction

1. Choose option `1` (Register a new document).
2. Enter a title and author.
3. At the category prompt enter `0`.

Observed:

```text
thread 'main' panicked at src/main.rs:631:
Unknown option: 0
```

Automated reproduction:

```bash
cargo test invalid_category_selection_is_rejected
```

### Expected

`0` should be rejected with a message and the prompt should be shown again. This
violates section 8 of the specification ("Selecting a menu option that does not
exist ... must not terminate the application"). The same prompt helper should be
reviewed wherever valid choices do not start at 0.

---

## Open Issue 4 - Internal functions still panic instead of returning errors

**GitHub:** #4  
**Severity:** Medium  
**Location:** `src/main.rs:508, 553, 583, 586, 589, 593, 659, 685, 689`

### Description

The UI now prevents the empty-title/author cases, but validation and state helpers
still panic on invalid data:

- `document_create()` panics on empty title/author, invalid category, invalid year.
- `app_get_archive_statistics()` panics on an unknown category.
- `document_set_availability()` panics when asked for an invalid transition.
- `document_category_get_label()` panics on an unknown category.
- `app_data_generate_document_id()` panics when the archive is full (currently
  guarded by the caller).

The underlying functions are one refactor away from panicking on real input and are
hard to reuse/test. Returning `Result` (or a dedicated validation error) would keep
the specification's "do not panic under normal usage" guarantee robust.

### Reproduction

The title/author panics are currently unreachable through the UI because empty input
is rejected. They remain reachable to any future caller and are covered by the general
concern rather than a UI reproduction.

---

## Open Issue 5 - Fixed-size document array

**GitHub:** #5  
**Severity:** Medium  
**Location:** `DocumentRepository` / `INITIAL_DOCUMENTS`, `src/main.rs:39-56`

### Description

The archive is still a fixed array (`[Document; MAX_NUM_OF_DOCUMENT]`) with hardcoded
placeholder entries. Combined with the reduced `MAX_NUM_OF_DOCUMENT = 2`, this makes
capacity both tiny and inflexible. Any future increase requires editing the
placeholder list to match, and the commented-out entries are easy to get out of sync.

### Recommendation

Use `Vec<Document>` and keep only registered documents (or derive placeholders from
the vector length). This removes the manual placeholder count and makes the full
archive check a simple length comparison.

---

## Open Issue 6 - Long titles and authors are truncated in tables

**GitHub:** #8  
**Severity:** Low  
**Location:** `src/main.rs:302, 318, 339, 363`

### Description

Titles are printed with a width of 32 and authors with 24. Longer values are silently
truncated by Rust's formatting, so the view and search screens can hide part of a
document's title or author. This is a usability issue, not a data corruption issue.

### Reproduction

Register a document with a title longer than 32 characters and view the archive.

---

## Observations (not currently filed as separate bugs)

- Borrow/return match the document ID exactly, including case. IDs are displayed as
  uppercase (`DOC-101-000`), so entering lowercase is treated as "not found". Search
  is case-insensitive; borrow/return matching is not. The specification only requires
  case-insensitive text search, so this is an inconsistency rather than a clear
  violation.
- `app_data_generate_document_id()` takes `&mut AppData` but does not mutate it.
  This is a minor API smell.
- Category `0` panic has a sibling risk: any future prompt using
  `ui_prompt_user_option()` can accept a leading zero choice if its valid range does
  not start at 0. The prompt helper should take an explicit minimum.

---

## Resolved and closed in this pass

These items from the previous report were fixed and their GitHub issues closed:

| GitHub | Item | Resolution | Commit |
|---|---|---|---|
| #1 | No bounds checking when inserting documents | Registration checks `app_data_is_archive_full()` first; full archive is reported instead of indexing out of bounds. | `77cb897` |
| #2 | Panic on empty title/author | Empty/whitespace input is rejected at the prompt and re-requested. | `db09976` |
| #3 | UI did not validate empty title/author | Same prompt-level validation; no ID is allocated for empty input. | `db09976` |
| #6 | Confusing ID/slot management | `AppData` is now `(next_index, repository)` and the two counters were removed. | `5a16ceb` |
| #7 | Negative ID cast to a large `u32` | IDs are strings; no integer cast remains. | `5a16ceb` |
| #9 | Separate ID counters and wrong slot indexing | Single next-free-slot index and string ID format `DOC-<category>-<slot>`. | `5a16ceb` |

## Verified behavior

The passing tests cover:

- start and deliberate exit;
- invalid and non-numeric menu options do not terminate the app;
- registration and viewing with unique string IDs;
- identical titles coexisting with different IDs;
- case-insensitive partial search by title, author, category, and ID;
- no-match search output and search being read-only;
- borrow/return success and rejection paths;
- rejected operations leaving the archive unchanged;
- statistics totals and per-category percentages;
- empty title/author retry;
- invalid year and non-numeric year retry;
- archive-full handling (as currently implemented with capacity 2).

## How to reproduce everything

```bash
cargo test
```

Expected at the time of writing: 15 passed, 3 failed. The failures correspond to the
three open GitHub bugs above and are expected to remain red until those bugs are
fixed.
