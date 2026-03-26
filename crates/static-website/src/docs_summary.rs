use ssg_whiz::summaries::{Category, PageSummary, Summary};

pub fn summary() -> Summary {
    Summary {
        source_folder: "docs",
        categories: vec![
            Category {
                name: "Getting Started".to_string(),
                pages: vec![PageSummary {
                    date: "2026-03-22",
                    title: "Getting Started",
                    description: "How customers get an API key, browse integrations, and launch the hosted connection flow.",
                    folder: "docs/getting-started",
                    markdown: include_str!("../content/docs/getting-started/index.md"),
                    image: None,
                    author: None,
                    author_image: None,
                }],
            },
            Category {
                name: "Integrations".to_string(),
                pages: vec![PageSummary {
                    date: "2026-03-26",
                    title: "Fetching Integrations",
                    description: "List integrations and current connection status for an end user with an org API key.",
                    folder: "docs/integrations/fetching-integrations",
                    markdown: include_str!("../content/docs/integrations/fetching-integrations.md"),
                    image: None,
                    author: None,
                    author_image: None,
                }],
            },
        ],
    }
}
