use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

fn run_app(input: &str) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_archive_keeper"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;

            if let Some(stdin) = child.stdin.as_mut() {
                stdin.write_all(input.as_bytes())?;
            }

            child.wait_with_output()
        })
        .expect("application should run successfully");

    assert!(
        output.status.success(),
        "application exited with failure\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let mut combined = String::from_utf8_lossy(&output.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    combined
}

fn assert_contains_all(haystack: &str, expected: &[&str]) {
    for needle in expected {
        assert!(
            haystack.contains(needle),
            "missing expected text: {needle}\nfull output:\n{haystack}"
        );
    }
}

fn assert_not_contains(haystack: &str, needle: &str) {
    assert!(
        !haystack.contains(needle),
        "did not expect text: {needle}\nfull output:\n{haystack}"
    );
}

const DOC_ONE_ID: &str = "DOC-101-000";
const DOC_TWO_ID: &str = "DOC-102-001";

#[test]
fn app_starts_and_exits_cleanly() {
    let output = run_app("0\n");

    assert_contains_all(
        &output,
        &[
            "Welcome to The Archive Keeper",
            "Main Menu",
            "Exiting application... Bye!!!",
        ],
    );
}

#[test]
fn invalid_menu_selection_is_rejected_without_terminating_the_app() {
    let output = run_app("9\n0\n");

    assert_contains_all(
        &output,
        &[
            "Invalid input. Option must be between 0 and 6",
            "Exiting application... Bye!!!",
        ],
    );
}

#[test]
fn non_numeric_menu_input_is_rejected_without_terminating_the_app() {
    let output = run_app("abc\n0\n");

    assert_contains_all(
        &output,
        &[
            "Invalid input. Please enter a number",
            "Exiting application... Bye!!!",
        ],
    );
}

#[test]
fn documents_can_be_registered_and_viewed_with_unique_ids() {
    let output = run_app(
        concat!(
            "1\n",
            "Shared Title\n",
            "Author One\n",
            "1\n",
            "2024\n",
            "1\n",
            "Shared Title\n",
            "Author Two\n",
            "2\n",
            "2023\n",
            "2\n",
            "0\n",
        ),
    );

    assert_contains_all(
        &output,
        &[
            "Document registered successfully with ID: DOC-101-000",
            "Document registered successfully with ID: DOC-102-001",
            "Shared Title",
            "Author One",
            "Author Two",
            "Research Paper",
            "Technical Report",
            "Available",
        ],
    );

    assert_not_contains(&output, "Document registered successfully with ID: DOC-101-001");
}

#[test]
fn search_is_case_insensitive_and_supports_partial_matches() {
    let output = run_app(
        concat!(
            "1\n",
            "Introduction to Engine Design\n",
            "Acme Labs\n",
            "1\n",
            "2024\n",
            "1\n",
            "Engine Maintenance Guide\n",
            "Acme Labs\n",
            "2\n",
            "2023\n",
            "3\n",
            "ENGINE\n",
            "0\n",
        ),
    );

    assert_contains_all(
        &output,
        &[
            "Search Results for Keyword: engine",
            "Introduction to Engine Design",
            "Engine Maintenance Guide",
            "Acme Labs",
        ],
    );
}

#[test]
fn search_matches_author_category_and_partial_id() {
    let output = run_app(
        concat!(
            "1\n",
            "Engine Design\n",
            "Acme Labs\n",
            "1\n",
            "2024\n",
            "1\n",
            "Quarterly Report\n",
            "Research Team\n",
            "2\n",
            "2023\n",
            "3\n",
            "acme\n",
            "3\n",
            "technical\n",
            "3\n",
            "doc-101\n",
            "0\n",
        ),
    );

    assert_contains_all(
        &output,
        &[
            "Search Results for Keyword: acme",
            "Engine Design",
            "Search Results for Keyword: technical",
            "Quarterly Report",
            "Search Results for Keyword: doc-101",
            DOC_ONE_ID,
        ],
    );
}

#[test]
fn search_with_no_matches_reports_no_results() {
    let output = run_app(
        concat!(
            "1\n",
            "Engine Design\n",
            "Acme Labs\n",
            "1\n",
            "2024\n",
            "3\n",
            "does-not-exist\n",
            "0\n",
        ),
    );

    assert_contains_all(
        &output,
        &[
            "Search Results for Keyword: does-not-exist",
            "No results found for keyword [does-not-exist]",
        ],
    );
}

#[test]
fn search_is_read_only() {
    let output = run_app(
        concat!(
            "1\n",
            "Engine Design\n",
            "Acme Labs\n",
            "1\n",
            "2024\n",
            "3\n",
            "engine\n",
            "6\n",
            "0\n",
        ),
    );

    assert_contains_all(
        &output,
        &[
            "-> Total Number of Registered Documents - 1",
            "-> Total Number of Available  Documents - 1",
            "-> Total Number of Borrowed   Documents - 0",
        ],
    );
}

#[test]
fn borrow_return_and_statistics_reflect_current_state() {
    let output = run_app(
        concat!(
            "1\n",
            "Borrowable Report\n",
            "Research Team\n",
            "1\n",
            "2024\n",
            "4\n",
            "DOC-101-000\n",
            "4\n",
            "DOC-101-000\n",
            "5\n",
            "DOC-101-000\n",
            "5\n",
            "DOC-101-000\n",
            "6\n",
            "0\n",
        ),
    );

    assert_contains_all(
        &output,
        &[
            "Document [Borrowable Report] with ID [DOC-101-000] borrowed successfully",
            "Document is already borrowed",
            "Document [Borrowable Report] with ID [DOC-101-000] returned successfully",
            "Document is already available",
            "-> Total Number of Registered Documents - 1",
            "-> Total Number of Available  Documents - 1",
            "-> Total Number of Borrowed   Documents - 0",
        ],
    );
}

#[test]
fn rejected_operations_do_not_change_the_archive() {
    let output = run_app(
        concat!(
            "1\n",
            "Stable Document\n",
            "Archive Team\n",
            "1\n",
            "2024\n",
            "4\n",
            "DOC-999-999\n",
            "5\n",
            "DOC-999-999\n",
            "5\n",
            "DOC-101-000\n",
            "4\n",
            "DOC-101-000\n",
            "5\n",
            "DOC-101-000\n",
            "6\n",
            "0\n",
        ),
    );

    assert_contains_all(
        &output,
        &[
            "No document with ID [DOC-999-999] found!",
            "No document ID [DOC-999-999] found!",
            "Document is already available",
            "Document [Stable Document] with ID [DOC-101-000] borrowed successfully",
            "Document [Stable Document] with ID [DOC-101-000] returned successfully",
            "-> Total Number of Registered Documents - 1",
            "-> Total Number of Available  Documents - 1",
            "-> Total Number of Borrowed   Documents - 0",
        ],
    );
}

#[test]
fn statistics_show_per_category_counts_after_registering() {
    let output = run_app(
        concat!(
            "1\n",
            "Paper One\n",
            "Author One\n",
            "1\n",
            "2024\n",
            "1\n",
            "Report Two\n",
            "Author Two\n",
            "2\n",
            "2023\n",
            "6\n",
            "0\n",
        ),
    );

    assert_contains_all(
        &output,
        &[
            "-> Total Number of Registered Documents - 2",
            "-> Total Number of Available  Documents - 2",
            "-> Total Number of Borrowed   Documents - 0",
            "Research Paper",
            "Technical Report",
            "Meeting Notes",
            "Internal Memo",
            "50.00",
        ],
    );
}

#[test]
fn empty_title_and_author_are_rejected_then_retried() {
    let output = run_app(
        concat!(
            "1\n",
            "\n",
            "Valid Title\n",
            "\n",
            "Valid Author\n",
            "1\n",
            "2024\n",
            "0\n",
        ),
    );

    assert_contains_all(
        &output,
        &[
            "User entered an empty value",
            "Document registered successfully with ID: DOC-101-000",
        ],
    );
}

#[test]
fn invalid_years_are_rejected_until_a_valid_year_is_entered() {
    let output = run_app(
        concat!(
            "1\n",
            "Dated Document\n",
            "Author\n",
            "1\n",
            "1899\n",
            "2027\n",
            "not-a-year\n",
            "2024\n",
            "0\n",
        ),
    );

    assert_contains_all(
        &output,
        &[
            "Year must be between 1900 and 2026",
            "Invalid input. Please enter a number",
            "Document registered successfully with ID: DOC-101-000",
        ],
    );
}

#[test]
fn invalid_category_selection_is_rejected() {
    let output = run_app(
        concat!(
            "1\n",
            "Categorised Document\n",
            "Author\n",
            "5\n",
            "0\n",
            "1\n",
            "2024\n",
            "0\n",
        ),
    );

    assert_contains_all(
        &output,
        &[
            "Invalid input. Option must be between 0 and 4",
            "Document registered successfully with ID: DOC-101-000",
        ],
    );
}

#[test]
fn empty_search_keyword_is_rejected_then_retried() {
    let output = run_app(
        concat!(
            "1\n",
            "Engine Design\n",
            "Acme Labs\n",
            "1\n",
            "2024\n",
            "3\n",
            "\n",
            "engine\n",
            "0\n",
        ),
    );

    assert_contains_all(
        &output,
        &[
            "User entered an empty value",
            "Search Results for Keyword: engine",
            "Engine Design",
        ],
    );
}

#[test]
fn registering_when_the_archive_is_full_is_reported_gracefully() {
    let output = run_app(
        concat!(
            "1\n",
            "First Document\n",
            "Author One\n",
            "1\n",
            "2024\n",
            "1\n",
            "Second Document\n",
            "Author Two\n",
            "2\n",
            "2023\n",
            "1\n",
            "6\n",
            "0\n",
        ),
    );

    assert_contains_all(
        &output,
        &[
            "Document registered successfully with ID: DOC-101-000",
            "Document registered successfully with ID: DOC-102-001",
            "Archive is full!",
            "-> Total Number of Registered Documents - 2",
        ],
    );
}

#[test]
fn archive_can_hold_more_than_two_documents() {
    let output = run_app(
        concat!(
            "1\n",
            "First Document\n",
            "Author One\n",
            "1\n",
            "2024\n",
            "1\n",
            "Second Document\n",
            "Author Two\n",
            "2\n",
            "2023\n",
            "1\n",
            "Third Document\n",
            "Author Three\n",
            "3\n",
            "2022\n",
            "0\n",
        ),
    );

    assert_contains_all(
        &output,
        &["Document registered successfully with ID: DOC-103-002"],
    );
}

#[test]
fn eof_on_stdin_does_not_leave_the_application_looping() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_archive_keeper"))
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("application should start");

    let deadline = Instant::now() + Duration::from_secs(2);

    loop {
        if let Some(status) = child.try_wait().expect("failed to inspect child") {
            assert!(status.success(), "application exited with a failure status");
            return;
        }

        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            panic!("application did not exit when stdin reached EOF");
        }

        thread::sleep(Duration::from_millis(20));
    }
}
