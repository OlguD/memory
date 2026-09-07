use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Priority { High, Low }

struct Task { title: String, priority: Priority }

fn render(list_name: &str, tasks: &[Task]) -> String {
    let mut out = String::new();
    out += &format!("# Tasks: {}\n", list_name);

    let sections = [
        (Priority::High, "High priority"),
        (Priority::Low, "Low priority"),
    ];

    for (priority, title) in sections {
        let items: Vec<&Task> = tasks.iter().filter(|t| t.priority == priority).collect();
        if items.is_empty() {
            continue;
        }
        out += &format!("\n## {}\n", title);
        for t in items {
            out += &format!("- {}\n", t.title);
        }
    }

    out
}

fn write(dir: &str, content: &str) -> std::io::Result<()> {
    let path = Path::new(dir).join("TASKS.md");
    std::fs::write(path, content)
}
