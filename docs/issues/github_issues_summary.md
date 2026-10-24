# GitHub Issues Created

All issues created in repository: https://github.com/FranklinEkemezie/archive_keeper

## Group 1: Critical Data Integrity / Logic Bugs
- #1 No bounds checking when inserting documents (array out of bounds) - bug, high
- #2 Panic on invalid registration input (empty title/author) - bug, high  
- #9 ID generation uses separate counters and slot indexing is incorrect - bug, high

## Group 2: Validation & UX Bugs
- #3 UI does not validate empty title/author before ID allocation/creation - bug, medium
- #7 Negative ID input becomes large u32 due to cast - bug, low

## Group 3: Design/Robustness Improvements (Enhancements)
- #4 Replace panics with Result-based validation for robustness - enhancement, medium
- #5 Consider using Vec<Document> instead of fixed array - enhancement, low
- #6 Clarify and simplify ID/slot management - enhancement, medium
- #8 Long titles/authors truncated in table display - enhancement, low
