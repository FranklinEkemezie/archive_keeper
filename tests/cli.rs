use std::process::{Command, Stdio};

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
            "Document registered successfully with ID: 1",
            "Document registered successfully with ID: 2",
            "Shared Title",
            "Author One",
            "Author Two",
            "Research Paper",
            "Technical Report",
            "Available",
        ],
    );
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
fn borrow_return_and_statistics_reflect_current_state() {
    let output = run_app(
        concat!(
            "1\n",
            "Borrowable Report\n",
            "Research Team\n",
            "1\n",
            "2024\n",
            "4\n",
            "1\n",
            "4\n",
            "1\n",
            "5\n",
            "1\n",
            "5\n",
            "1\n",
            "6\n",
            "0\n",
        ),
    );

    assert_contains_all(
        &output,
        &[
            "Document [Borrowable Report] with ID [1] borrowed successfully",
            "Document is already borrowed",
            "Document [Borrowable Report] with ID [1] returned successfully",
            "Document is already available",
            "-> Total Number of Registered Documents - 1",
            "-> Total Number of Available  Documents - 1",
            "-> Total Number of Borrowed   Documents - 0",
        ],
    );
}