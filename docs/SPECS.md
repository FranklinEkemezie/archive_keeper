# Project 2: The Archive Keeper

## Background

A small research organization keeps a record of documents it receives from different departments. These documents include research papers, technical reports, meeting notes, and internal memos.

Currently, the organization tracks these documents using handwritten registers. This makes it difficult to find a particular document, determine which documents are still available, and keep track of documents that have been borrowed.

The organization wants a simple terminal-based application to manage its document archive.

You have been hired to build the first version of this application.

Your client understands the problem and knows what they want the application to do. However, they are not concerned with the programming language features or the internal implementation. They expect you to make reasonable technical decisions.

---

## 1. General Requirements

The application must run in a terminal and present a menu that allows an employee to manage the archive.

It must support the following operations:

1. Register a new document.
    
2. View all registered documents.
    
3. Search for a document.
    
4. Borrow a document.
    
5. Return a borrowed document.
    
6. View archive statistics.
    
7. Exit the application.
    

The application should remain usable until the employee chooses to exit.

For this first version, the archive only needs to exist while the application is running. Documents do not need to be saved permanently between separate executions.

## 2. Registering Documents

An employee should be able to register a document by providing the following information:

- **Title:** The document's title.
    
- **Author:** The person or organization responsible for it.
    
- **Category:** The type of document, such as Research Paper, Technical Report, Meeting Notes, or Internal Memo.
    
- **Year:** The year the document was published or created.
    

Each registered document must receive a unique identification number that the application assigns automatically.

The application must not allow a document to be registered if any required information is missing or invalid.

For this version, the year must be between 1900 and the current year.

Two documents may have the same title. The application must still be able to distinguish between them.

After a successful registration, the employee should receive confirmation and see the document's assigned identification number.

## 3. Viewing the Archive

An employee should be able to view all documents currently registered in the archive.

For each document, display its:

- Identification number.
    
- Title.
    
- Author.
    
- Category.
    
- Year.
    
- Availability status.
    

The availability status must clearly indicate whether the document is available or currently borrowed.

If the archive contains no documents, the application should communicate that instead of displaying an empty or confusing list.

The listing should be readable even when several documents have been registered.

## 4. Searching for Documents

Employees should be able to search the archive using a document's title, author, category, or identification number.

Text searches should be case-insensitive. For example, searching for `ENGINE` should be capable of finding a document titled `Introduction to Engine Design`.

Text searches should also support partial matches. An employee should not have to remember the complete title or author's name.

The application should display every matching document, not just the first one.

If nothing matches the search, it should tell the employee that no matching documents were found.

Searching must not change the archive.

## 5. Borrowing Documents

An employee should be able to borrow a document by entering its identification number.

A document can only be borrowed if it exists in the archive and is currently available.

When a borrowing operation succeeds, the document's status must change to borrowed.

The application must reject attempts to borrow:

- A document that does not exist.
    
- A document that has already been borrowed.
    

Rejected operations must not accidentally change the status of any document.

The employee should receive a clear message explaining whether the operation succeeded or why it could not be completed.

## 6. Returning Documents

An employee should be able to return a borrowed document using its identification number.

A successful return should make the document available for borrowing again.

The application must reject attempts to return a document that does not exist or is already available.

A failed return operation must not alter the archive.

## 7. Archive Statistics

The organization wants a quick overview of its archive.

Provide a statistics screen showing:

- The total number of registered documents.
    
- The number of available documents.
    
- The number of borrowed documents.
    
- The number of documents in each category.
    

The statistics must reflect the current state of the archive. For example, borrowing a document should affect the available and borrowed counts immediately.

If the archive is empty, the statistics screen should still display sensible results.

## 8. Input and Error Handling

Employees are not expected to understand technical errors.

The application should handle invalid input gracefully and explain what went wrong in ordinary language.

Examples include:

- Entering letters where a number is expected.
    
- Selecting a menu option that does not exist.
    
- Entering an identification number that is not registered.
    
- Providing an invalid year.
    
- Attempting an operation that is not permitted.
    

An invalid menu selection must not terminate the application.

Where an operation fails, the application should preserve the existing archive data.

The application should also handle unexpected input without panicking under normal usage.

## 9. User Experience

The application should be straightforward enough for an employee with no technical background to use without assistance.

The main menu should be displayed clearly, and employees should be able to return to it after completing an operation.

Messages should distinguish between successful operations, invalid input, and operations that cannot be completed.

The application should not require employees to restart it after every registration, search, borrowing, or return.

---

## Acceptance Criteria

The application will be considered complete when the organization can demonstrate the following:

1. An employee can register several documents and receive a unique identification number for each.
    
2. Documents with identical titles can coexist in the archive.
    
3. The employee can view all registered documents and their current availability.
    
4. Searches work using identification numbers and partial, case-insensitive text matches.
    
5. Available documents can be borrowed, and borrowed documents cannot be borrowed again.
    
6. Borrowed documents can be returned, making them available again.
    
7. Invalid operations do not corrupt or unintentionally modify the archive.
    
8. Archive statistics remain accurate after registrations, borrowings, and returns.
    
9. Invalid user input does not ordinarily crash or terminate the application.
    
10. The employee can perform multiple operations during one execution and exit deliberately.
    

---

## Scope and Constraints

This is the first version of the Archive Keeper.

- It is a single-user, terminal-based application.
    
- It does not require a graphical interface.
    
- It does not require a database or permanent storage.
    
- It does not require user accounts, authentication, or permissions.
    
- It does not need to track individual borrowers or borrowing dates.
    
- It does not need to communicate with external services.
    
- It should use Rust's standard library only. No external crates are necessary.
    

You may decide how to represent documents, organize the program, structure the menu, validate input, and implement the archive operations.

You are responsible for choosing an approach that satisfies the requirements while keeping the program understandable and maintainable.

**The goal is not merely to make the application work. It is to make sensible engineering decisions while building software that behaves correctly.**