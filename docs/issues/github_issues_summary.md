# GitHub Issues Summary

Repository: https://github.com/FranklinEkemezie/archive_keeper

## Open

| # | Title | Type | Severity |
|---|---|---|---|
| 10 | EOF on stdin causes an infinite empty-input retry loop | bug | High |
| 11 | Archive capacity was reduced to two documents | bug | High |
| 12 | Category menu accepts 0 and then panics | bug | High |
| 4 | Replace panics with Result-based validation for robustness | enhancement | Medium |
| 5 | Consider using Vec<Document> instead of fixed array | enhancement | Medium |
| 8 | Long titles/authors truncated in table display | enhancement | Low |

## Closed / resolved in this pass

| # | Title | Resolution commit |
|---|---|---|
| 1 | No bounds checking when inserting documents (array out of bounds) | 77cb897 |
| 2 | Panic on invalid registration input (empty title/author) | db09976 |
| 3 | UI does not validate empty title/author before ID allocation/creation | db09976 |
| 6 | Clarify and simplify ID/slot management | 5a16ceb |
| 7 | Negative ID input becomes large u32 due to cast | 5a16ceb |
| 9 | ID generation uses separate counters and slot indexing is incorrect | 5a16ceb |

## Test status

`cargo test` currently reports 15 passed and 3 failed. The failures are deliberate
regression tests for issues #10, #11, and #12.
