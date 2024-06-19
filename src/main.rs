use dirs::config_dir;
use serde::Serialize;
use std::io;

#[derive(Debug)]
struct Workspace {
    workspace_id: i64,
    local_paths: Vec<u8>,
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
        let path = String::from_utf8_lossy(&workspace.local_paths)
            .into_owned()
            .as_str()
            .to_owned();
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
            r#type: String::from("fileicon"),
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
    let path = config_dir().unwrap().to_str().unwrap().to_owned() + "/Zed/db/0-stable/db.sqlite";
    let connection = sqlite::open(path).unwrap();
    let stmt = connection.prepare(
        "SELECT workspace_id, local_paths
            FROM workspaces
            WHERE local_paths IS NOT NULL
            ORDER BY timestamp DESC",
    )?;
    let mut items = vec![];
    for row in stmt.into_iter().map(|row| row.unwrap()) {
        let workspace = Workspace {
            workspace_id: row.read::<i64, _>("workspace_id").to_owned(),
            local_paths: row.read::<&[u8], _>("local_paths").to_vec(),
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
