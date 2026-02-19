use boj_client::catalog;

fn parse_table_rows(path: &str) -> Vec<Vec<String>> {
    let content = std::fs::read_to_string(path).expect("table file should be readable");
    content
        .lines()
        .filter(|line| line.starts_with('|') && !line.starts_with("|---"))
        .map(split_markdown_row)
        .collect()
}

fn split_markdown_row(line: &str) -> Vec<String> {
    let trimmed = line.trim().trim_start_matches('|').trim_end_matches('|');

    let mut cells = Vec::new();
    let mut current = String::new();
    let mut escaped = false;

    for ch in trimmed.chars() {
        if escaped {
            if ch == '|' || ch == '\\' {
                current.push(ch);
            } else {
                current.push('\\');
                current.push(ch);
            }
            escaped = false;
            continue;
        }

        if ch == '\\' {
            escaped = true;
            continue;
        }

        if ch == '|' {
            cells.push(current.trim().to_string());
            current.clear();
            continue;
        }

        current.push(ch);
    }

    if escaped {
        current.push('\\');
    }

    cells.push(current.trim().to_string());
    cells
}

#[test]
fn catalog_db_entries_match_appendix_a() {
    let rows = parse_table_rows("docs/api-manual/appendix-db-list.md")
        .into_iter()
        .filter(|cols| cols.len() >= 3 && cols[1] != "DB名")
        .collect::<Vec<_>>();

    let entries = catalog::databases();
    assert_eq!(entries.len(), 50);
    assert_eq!(rows.len(), entries.len());

    for (row, entry) in rows.iter().zip(entries.iter()) {
        assert_eq!(row[0], entry.category_ja);
        assert_eq!(row[1], entry.code);
        assert_eq!(row[2], entry.name_ja);
    }
}

#[test]
fn catalog_message_entries_match_appendix_b() {
    let rows = parse_table_rows("docs/api-manual/appendix-message-codes.md")
        .into_iter()
        .filter(|cols| cols.len() >= 4 && cols[0] != "STATUS")
        .collect::<Vec<_>>();

    let entries = catalog::message_codes();
    assert_eq!(entries.len(), 24);
    assert_eq!(rows.len(), entries.len());

    for (row, entry) in rows.iter().zip(entries.iter()) {
        let status = row[0]
            .parse::<u16>()
            .expect("STATUS column should be numeric");
        assert_eq!(status, entry.status);
        assert_eq!(row[1], entry.message_id);
        assert_eq!(row[2], entry.message);
        assert_eq!(row[3], entry.note);
    }
}

#[test]
fn db_lookup_is_case_insensitive_and_fail_open() {
    let bp01 = catalog::find_db("bp01").expect("known db should exist");
    assert_eq!(bp01.code, "BP01");

    assert!(!catalog::is_known_db("UNKNOWN_DB"));
}
