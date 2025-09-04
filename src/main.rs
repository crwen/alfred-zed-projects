use dirs::config_dir;
use serde::Serialize;
use std::io;

mod constants;
use crate::constants::*;

#[derive(Debug)]
struct Workspace {
    workspace_id: i64,
    local_paths: String,
}

#[derive(Serialize, Debug)]
struct Item {
    uid: i64,
    title: String,
    subtitle: String,
    icon: Icon,
    arg: String,
}

impl From<Workspace> for Item {
    fn from(workspace: Workspace) -> Self {
        let path = workspace.local_paths;
        let index = path.find('/').unwrap_or(path.len());
        let path = path.split_at(index).1;
        let name = path.trim_end_matches('/').split('/').last().unwrap_or("");
        Self {
            uid: workspace.workspace_id,
            title: name.to_owned(),
            subtitle: path.to_owned(),
            icon: Icon::new(path.to_owned()),
            arg: path.to_owned(),
        }
    }
}

#[derive(Serialize, Debug)]
struct Icon {
    path: String,
    r#type: String,
}

impl Icon {
    fn new(path: String) -> Icon {
        Icon {
            path,
            r#type: ICON_TYPE_FILE.to_string(),
        }
    }
}

#[derive(Serialize, Debug)]
struct Response {
    items: Vec<Item>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Alfred passes in a single argument for the user query.
    let query = std::env::args().nth(1);
    let path = config_dir().unwrap().to_str().unwrap().to_owned() + DB_PATH;
    let connection = sqlite::open(path).unwrap();
    let stmt = connection.prepare(format!(
        "SELECT {DB_FIELD_WORKSPACE_ID}, {DB_FIELD_PATHS}
            FROM workspaces
            WHERE {DB_FIELD_PATHS} IS NOT NULL
            ORDER BY timestamp DESC"
    ))?;
    let mut items = vec![];
    for row in stmt.into_iter().map(|row| row.unwrap()) {
        let workspace = Workspace {
            workspace_id: row.read::<i64, _>(DB_FIELD_WORKSPACE_ID).to_owned(),
            local_paths: row.read::<&str, _>(DB_FIELD_PATHS).to_string(),
        };
        let item = Item::from(workspace);
        items.push(item);
    }

    // filter by query
    if let Some(query) = query {
        let arg = query.to_lowercase();
        items.retain(|item| {
            item.title.to_lowercase().contains(&arg) || item.subtitle.to_lowercase().contains(&arg)
        });
    }

    // Output to Alfred!
    serde_json::to_writer(io::stdout(), &Response { items }).unwrap();
    Ok(())
}
