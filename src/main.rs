use std::{io::{self, Write}};

type Option = u8;
type OptionInfo = (
    String, // option menu
    Option, // max option
    String, // heading
    String, // label
);

type DateYear = u16;

type DocumentId = String;
type DocumentCategory = u8;
type Document = (
    DocumentId,         // id       - The document (auto-generated) ID
    String,             // title    - The document's title.
    String,             // author   - The person or organisation responsible for it.
    DocumentCategory,   // category - The type of document.
    DateYear,           // year     - The year the document was published or created.
    bool,               // borrowed - Whether the document has been borrowed

    // Meta-data
    bool                // placeholder  - Whether the details represent a registered document or just a placeholder
);

const DOCUMENT_CATEGORY_RESEARCH_PAPER:     DocumentCategory = 101;
const DOCUMENT_CATEGORY_TECHNICAL_REPORT:   DocumentCategory = 102;
const DOCUMENT_CATEGORY_MEETING_NOTES:      DocumentCategory = 103;
const DOCUMENT_CATEGORY_INTERNAL_MEMO:      DocumentCategory = 104;

const DOCUMENT_CATEGORIES: [DocumentCategory; 4] = [
    DOCUMENT_CATEGORY_RESEARCH_PAPER,
    DOCUMENT_CATEGORY_TECHNICAL_REPORT,
    DOCUMENT_CATEGORY_MEETING_NOTES,
    DOCUMENT_CATEGORY_INTERNAL_MEMO
];

const MAX_NUM_OF_DOCUMENT: usize = 2;
const MIN_VALID_DOCUMENT_YEAR: DateYear = 1900;
const CURRENT_YEAR: DateYear = 2026;

type DocumentRepository = [Document; MAX_NUM_OF_DOCUMENT]; 

const INITIAL_DOCUMENTS: DocumentRepository = [
    (String::new(), String::new(), String::new(), DOCUMENT_CATEGORY_RESEARCH_PAPER, 1900, false, true),
    (String::new(), String::new(), String::new(), DOCUMENT_CATEGORY_RESEARCH_PAPER, 1900, false, true),
    // (String::new(), String::new(), String::new(), DOCUMENT_CATEGORY_RESEARCH_PAPER, 1900, false, true),
    // (String::new(), String::new(), String::new(), DOCUMENT_CATEGORY_RESEARCH_PAPER, 1900, false, true),
    // (String::new(), String::new(), String::new(), DOCUMENT_CATEGORY_RESEARCH_PAPER, 1900, false, true),
    // (String::new(), String::new(), String::new(), DOCUMENT_CATEGORY_RESEARCH_PAPER, 1900, false, true),
    // (String::new(), String::new(), String::new(), DOCUMENT_CATEGORY_RESEARCH_PAPER, 1900, false, true),
    // (String::new(), String::new(), String::new(), DOCUMENT_CATEGORY_RESEARCH_PAPER, 1900, false, true),
    // (String::new(), String::new(), String::new(), DOCUMENT_CATEGORY_RESEARCH_PAPER, 1900, false, true),
    // (String::new(), String::new(), String::new(), DOCUMENT_CATEGORY_RESEARCH_PAPER, 1900, false, true),
];

type AppData = (
    usize,              // next available index
    DocumentRepository  // document repository
);

type ArchiveStatistics = (
    usize,              // total
    usize,              // available
    usize,              // borrowed

    (
        usize,          // research paper
        usize,          // technical report
        usize,          // meeting notes
        usize,          // internal memo
    ),

    (
        f64,            // research paper
        f64,            // technical report
        f64,            // meeting notes
        f64,            // internal memo
    )
);


fn main() {

    ui_print_heading("Welcome to The Archive Keeper");

    let document_repository: DocumentRepository = INITIAL_DOCUMENTS;

    let mut app_data: AppData = (0, document_repository);

    loop {

        let option = main_menu_prompt();

        if ! main_menu_handle_option(option, &mut app_data) { return; }
    
    }

}

/*
 * -------------------------------------
 * UI
 * ------------------------------------
 */

fn ui_print_heading(heading: &str) {
    println!();
    println!("{:=^40}", "");
    println!("{heading:^40}");
    println!("{:=^40}", "");
    println!();
}

fn ui_print_subheading(heading: &str) {
    println!();
    println!("{heading:40}");
    println!("{:-^40}", "");
}

fn ui_print_info_msg(msg: &str) {
    println!();
    println!("[INFO] {msg}");
    println!();
}

fn ui_print_error_msg(msg: &str) {
    println!();
    println!("[ERROR] {msg}");
    println!();
}

fn ui_prompt_user(label: &str) -> String {

    loop {
        print!("{label}: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut buf = String::from("");

        let result = io::stdin().read_line(&mut buf);
        if result.is_err() {
            ui_print_error_msg("Failed to get user input. Please try again");
            continue;
        }

        let buf = buf.trim();
        if buf.is_empty() {
            ui_print_error_msg("User entered an empty value");
            continue;
        }


        return String::from(buf.trim());
    }
}

fn ui_prompt_user_int(label: &str) -> i32 {
    loop {
        let result = ui_prompt_user(label);

        let result = result.parse::<i32>();
        if result.is_err() {
            ui_print_error_msg("Invalid input. Please enter a number");
            continue;
        }

        return result.expect("Invalid input");
    }
}

fn ui_prompt_user_uint(label: &str) -> u32 {
    loop {
        let result = ui_prompt_user_int(label);
        if result.is_negative() {
            ui_print_error_msg("Invalid input. Please try again");
            continue;
        }

        return result.cast_unsigned();
    }
}

fn ui_prompt_user_option(label: &str, max_option: Option) -> Option {
    loop {
        let option = ui_prompt_user_uint(label) as Option;
        if option > max_option {
            ui_print_error_msg(&std::format!(
                "Invalid input. Option must be between 0 and {max_option}"));
            continue;
        }

        return option;
    }
}

fn ui_prompt_user_menu_option(option_info: OptionInfo) -> Option {

    let (menu, max_option, heading, label) = option_info;

    ui_print_subheading(&std::format!("{heading}"));
    println!("{menu}");

    return ui_prompt_user_option(&label, max_option);
}


/*
 * -------------------------------------
 * MAIN MENU
 * ------------------------------------
 */

fn main_menu_prompt() -> Option {

    const MAIN_MENU: &str = "
1. Register a new document.    
2. View all registered documents.
3. Search for a document.
4. Borrow a document.
5. Return a borrowed document.
6. View archive statistics.
0. Exit the application.
    ";

    const MAIN_MENU_MAX_OPTION: Option = 6;

    ui_prompt_user_menu_option((
        MAIN_MENU.to_string(),
        MAIN_MENU_MAX_OPTION,
        "Main Menu".to_string(),
        "Choose an option".to_string()
    ))
}

fn main_menu_handle_option(option: Option, app_data: &mut AppData) -> bool {

    return match option {
        1 => main_menu_register_document(app_data),
        2 => main_menu_view_registered_documents(app_data),
        3 => main_menu_search_document(app_data),
        4 => main_menu_borrow_document(app_data),
        5 => main_menu_return_borrowed_document(app_data),
        6 => main_menu_view_archive_stats(app_data),
        0 => main_menu_exit_app(),
        _ => panic!("Unknown option: {option}")
    }
}

fn main_menu_register_document(app_data: &mut AppData) -> bool {

    
    if app_data_is_archive_full(app_data) {
        ui_print_error_msg("Archive is full!");
        return true;
    }

    ui_print_subheading("Register a new document");

    let title = ui_prompt_user("Document Title");
    let author = ui_prompt_user("Author");
    let category = document_category_prompt_category();
    let year: DateYear;

    loop {
        let input = ui_prompt_user_uint("Year") as DateYear;
        if ! (input >= MIN_VALID_DOCUMENT_YEAR && input <= CURRENT_YEAR) {
            ui_print_error_msg(&std::format!(
                "Year must be between {MIN_VALID_DOCUMENT_YEAR} and {CURRENT_YEAR}"));
            continue;
        }

        year = input;
        break;
    }

    let id = app_data_generate_document_id(app_data, category);

    let document = document_create(&id, &title, &author, category, year);

    if ! app_data_insert_document(app_data, document) {
        ui_print_error_msg("An error occurred while registering your document. Please try again!");
        return true;
    }

    ui_print_info_msg(&std::format!(
        "Document registered successfully with ID: {id}"));

    true

}

fn main_menu_view_registered_documents(app_data: &AppData) -> bool {


    if app_get_registered_document_count(app_data) == 0 {
        ui_print_info_msg("No Registered Documents");
        return true;
    }

    let table_heading = std::format!("| {:^8} | {:^8} | {:^32} | {:^24} | {:^24} | {:^8} | {:^16} |",
        "S/N", "ID", "Title", "Author", "Category", "Year", "Availability");
    let horizontal_border = "-".repeat(table_heading.len());

    ui_print_subheading("List of Registered Documents");
    println!("{horizontal_border}");
    println!("{table_heading}");
    println!("{horizontal_border}");

    let mut i = 0;
    for d in &app_data.1[..] {

        if document_is_placeholder(d) { continue; }

        i += 1;

        println!("| {:^8} | {:^8} | {:32} | {:24} | {:24} | {:^8} | {:16} |",
            i,
            document_get_id(d),
            document_get_title(d),
            document_get_author(d),
            document_get_category_label(d),
            document_get_year(d),
            document_get_availability(d)
        );
        println!("{horizontal_border}");
    }

    true
}

fn main_menu_search_document(app_data: &AppData) -> bool {

    ui_print_subheading("Search documents by title, author, category, or ID");

    let search_query = ui_prompt_user("Enter keyword").to_lowercase();

    let table_heading = std::format!("| {:^8} | {:^8} | {:^32} | {:^24} | {:^24} | {:^8} | {:^16} |",
        "S/N", "ID", "Title", "Author", "Category", "Year", "Availability");
    let horizontal_border = "-".repeat(table_heading.len());

    ui_print_subheading(&std::format!(
        "Search Results for Keyword: {search_query}"));
    println!("{horizontal_border}");
    println!("{table_heading}");
    println!("{horizontal_border}");

    let mut i = 0;
    for d in &app_data.1[..] {

        if document_is_placeholder(d) { continue; }

        if ! (
            document_get_title(d).to_lowercase().contains(&search_query) ||         // title
            document_get_author(d).to_lowercase().contains(&search_query) ||        // author
            document_get_category_label(d).to_lowercase().contains(&search_query) ||      // category
            document_get_id(d).to_string().to_lowercase().contains(&search_query)   // id
        ) { continue; }

        i += 1;

        println!("| {:^8} | {:^8} | {:32} | {:24} | {:24} | {:^8} | {:16} |",
            i,
            document_get_id(d),
            document_get_title(d),
            document_get_author(d),
            document_get_category_label(d),
            document_get_year(d),
            document_get_availability(d)
        );
        println!("{horizontal_border}");
    }

    if i == 0 {
        println!("| ✕ No results found for keyword [{search_query}]");
        println!("{horizontal_border}");
    }

    true
}

fn main_menu_borrow_document(app_data: &mut AppData) -> bool {

    ui_print_subheading("Borrow a Document");

    let document_id = ui_prompt_user("Enter document ID") as DocumentId;

    for d in &mut app_data.1[..] {

        if 
            document_is_placeholder(d) || 
            *document_get_id(d) != document_id 
        { continue; }


        if document_is_borrowed(d) {
            ui_print_error_msg("Document is already borrowed");
            return true;
        }

        document_borrow(d);

        ui_print_info_msg(&std::format!(
            "Document [{}] with ID [{}] borrowed successfully",
            document_get_title(d), document_get_id(d)
        ));

        return true;
    }

    ui_print_error_msg(&std::format!(
        "No document with ID [{document_id}] found!"));

    true
}

fn main_menu_return_borrowed_document(app_data: &mut AppData) -> bool {

    ui_print_subheading("Return a Borrowed Document");

    let document_id = ui_prompt_user("Enter document ID") as DocumentId;

    for d in &mut app_data.1[..] {

        if 
            document_is_placeholder(d) ||
            *document_get_id(d) != document_id
        { continue; }

        if document_is_available(d) {
            ui_print_error_msg("Document is already available");
            return true;
        }

        document_return(d);

        ui_print_info_msg(&std::format!(
            "Document [{}] with ID [{}] returned successfully",
            document_get_title(d), document_get_id(d)
        ));

        return true;
    }

    ui_print_error_msg(&std::format!(
        "No document ID [{document_id}] found!"));

    true
}

fn main_menu_view_archive_stats(app_data: &AppData) -> bool {

    ui_print_subheading("Archive Statistics");

    let (
        total_count, available_count, borrowed_count, 
        by_categroy, by_category_percent
    ) = app_get_archive_statistics(app_data);

    println!("-> Total Number of Registered Documents - {}", total_count);
    println!("-> Total Number of Available  Documents - {}", available_count);
    println!("-> Total Number of Borrowed   Documents - {}", borrowed_count);
    println!();

    if total_count != 0 {

        let table_heading = std::format!(" {:24} | {:8} | {:8} ", 
            "Category", "Count", "%");
        let bottom_border = "-".repeat(table_heading.len());

        println!("{table_heading}");
        println!("{bottom_border}");

        println!(" {:24} | {:8} | {:8} ", "Research Paper",   by_categroy.0, &std::format!("{:.2}", by_category_percent.0));
        println!(" {:24} | {:8} | {:8} ", "Technical Report", by_categroy.1, &std::format!("{:.2}", by_category_percent.1));
        println!(" {:24} | {:8} | {:8} ", "Meeting Notes",    by_categroy.2, &std::format!("{:.2}", by_category_percent.2));
        println!(" {:24} | {:8} | {:8} ", "Internal Memo",    by_categroy.3, &std::format!("{:.2}", by_category_percent.3));
    }

    true
}

fn main_menu_exit_app() -> bool {
    ui_print_info_msg("Exiting application... Bye!!!");
    false
}


/*
 * -------------------------------------
 * APP DATA
 * ------------------------------------
 */


fn app_data_get_next_available_index(app_data: &AppData) -> usize {
    app_data.0
}

fn app_data_is_archive_full(app_data: &AppData) -> bool {
    app_data_get_next_available_index(app_data) >= MAX_NUM_OF_DOCUMENT
}

fn app_data_generate_document_id(app_data: &mut AppData, category: DocumentCategory) -> DocumentId {

    if app_data_is_archive_full(app_data) {
        panic!("Cannot generate document ID: Repository is full!");
    }

    let id = std::format!("DOC-{:03}-{:03}", category, 
        app_data_get_next_available_index(app_data));
    
    id
}

fn app_data_insert_document(app_data: &mut AppData, document: Document) -> bool {

    app_data.1[app_data_get_next_available_index(app_data)] = document;

    app_data.0 += 1;

    true
}

fn app_get_registered_document_count(app_data: &AppData) -> usize {

    let mut count = 0;
    for d in &app_data.1[..] {
        if ! document_is_placeholder(d) { count += 1 }
    }

    count
}

fn app_get_archive_statistics(app_data: &AppData) -> ArchiveStatistics {

    let mut stats: ArchiveStatistics = (0, 0, 0, (0, 0, 0, 0), (0.0, 0.0, 0.0, 0.0));

    for d in &app_data.1[..] {

        if document_is_placeholder(d) { continue; }

        stats.0 += 1; // total
        if document_is_available(d) { stats.1 += 1 }
        if document_is_borrowed(d)  { stats.2 += 1 }

        match document_get_category(d) {
            DOCUMENT_CATEGORY_RESEARCH_PAPER    => stats.3.0 += 1,
            DOCUMENT_CATEGORY_TECHNICAL_REPORT  => stats.3.1 += 1,
            DOCUMENT_CATEGORY_MEETING_NOTES     => stats.3.2 += 1,
            DOCUMENT_CATEGORY_INTERNAL_MEMO     => stats.3.3 += 1,
            c => panic!("Unknown category: {c}")
        };
    }

    if stats.0 != 0 {
        // fill in documents by percentage category
        stats.4.0 = stats.3.0 as f64 / stats.0 as f64 * 100.0;
        stats.4.1 = stats.3.1 as f64 / stats.0 as f64 * 100.0;
        stats.4.2 = stats.3.2 as f64 / stats.0 as f64 * 100.0;
        stats.4.3 = stats.3.3 as f64 / stats.0 as f64 * 100.0;
    }

    stats
}

/*
 * -------------------------------------
 * DOCUMENT
 * ------------------------------------
 */

fn document_create(
    id: &DocumentId,
    title: &str,
    author: &str,
    category: DocumentCategory,
    year: DateYear
) -> Document {
    
    let title = title.trim();
    if title.is_empty() { panic!("title cannot be empty"); }

    let author = author.trim();
    if author.is_empty() { panic!("author cannot be empty"); }

    if ! document_is_category_valid(category) {
        panic!("category is invalid");
    }

    if ! document_is_year_valid(year) {
        panic!("year is invalid");
    }

    (id.to_string(), String::from(title), String::from(author), category, year, false, false)
}

fn document_is_category_valid(category: DocumentCategory) -> bool {
    return DOCUMENT_CATEGORIES.contains(&category);
}

fn document_is_year_valid(year: DateYear) -> bool {
    year >= MIN_VALID_DOCUMENT_YEAR && year <= CURRENT_YEAR
}


fn document_category_prompt_category() -> DocumentCategory {

    const DOCUMENT_CATEGORY_MENU: &str = "
1. Research Paper
2. Technical Report.
3. Meeting Notes.
4. Internal Memo.
    ";

    const DOCUMENT_CATEGORY_MAX_OPTION: Option = 4;

    let option = ui_prompt_user_menu_option((
        DOCUMENT_CATEGORY_MENU.to_string(),
        DOCUMENT_CATEGORY_MAX_OPTION,
        "Document Category".to_string(),
        "Select document category".to_string()
    ));

    return match option {
        1 => DOCUMENT_CATEGORY_RESEARCH_PAPER,
        2 => DOCUMENT_CATEGORY_TECHNICAL_REPORT,
        3 => DOCUMENT_CATEGORY_MEETING_NOTES,
        4 => DOCUMENT_CATEGORY_INTERNAL_MEMO,
        _ => panic!("Unknown option: {option}")
    };

}

fn document_get_id(document: &Document) -> &DocumentId {
    &document.0
}

fn document_get_title(document: &Document) -> &String {
    &document.1
}

fn document_get_author(document: &Document) -> &String {
    &document.2
}

fn document_get_category(document: &Document) -> DocumentCategory {
    document.3
}

fn document_category_get_label(category: DocumentCategory) -> String {

    return String::from(match category {
        DOCUMENT_CATEGORY_RESEARCH_PAPER    => "Research Paper",
        DOCUMENT_CATEGORY_TECHNICAL_REPORT  => "Technical Report",
        DOCUMENT_CATEGORY_MEETING_NOTES     => "Meeting Notes",
        DOCUMENT_CATEGORY_INTERNAL_MEMO     => "Internal Memo",
        c => panic!("Unknown document category: {}", c)
    });
}

fn document_get_category_label(document: &Document) -> String {
    document_category_get_label(document_get_category(document))
}

fn document_get_year(document: &Document) -> DateYear {
    document.4
}

fn document_is_borrowed(document: &Document) -> bool {
    document.5
}

fn document_is_available(document: &Document) -> bool {
    ! document_is_borrowed(document)
}

fn document_get_availability(document: &Document) -> &str {
    if document_is_available(document) { "Available" } else { "Borrowed" }
}

fn document_set_availability(document: &mut Document, borrowed: bool) {
    if borrowed && document_is_borrowed(document) {
        panic!("Cannot borrow already borrowed document");
    }

    if ! borrowed && document_is_available(document) {
        panic!("Document is already available");
    }

    document.5 = borrowed;
}

fn document_borrow(document: &mut Document) {
    document_set_availability(document, true);
}

fn document_return(document: &mut Document) {
    document_set_availability(document, false);
}

fn document_is_placeholder(document: &Document) -> bool {
    document.6
}