use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::{self, Write};

#[derive(Serialize, Deserialize)]
struct UnlockRequest {
    username: String,
    password: String,
    items: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct UnlockResponse {
    success: bool,
    message: String,
}

struct App {
    client: Client,
}

impl App {
    async fn unlock_items(&self, username: &str, password: &str, items: Vec<String>) -> Result<UnlockResponse, Box<dyn Error>> {
        let request = UnlockRequest {
            username: username.to_string(),
            password: password.to_string(),
            items,
        };

        let response = self.client.post("https://api.fortnite.com/unlock")
            .json(&request)
            .send()
            .await?;

        let unlock_response: UnlockResponse = response.json().await?;
        Ok(unlock_response)
    }

    fn run(&self) {
        let username = self.prompt("Enter your Fortnite username: ");
        let password = self.prompt("Enter your Fortnite password: ");
        let items = self.prompt("Enter items to unlock (comma separated): ")
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let unlock_response = tokio::runtime::Runtime::new().unwrap().block_on(self.unlock_items(&username, &password, items)).unwrap();

        if unlock_response.success {
            println!("Items unlocked successfully: {}", unlock_response.message);
        } else {
            println!("Failed to unlock items: {}", unlock_response.message);
        }
    }

    fn prompt(&self, message: &str) -> String {
        print!("{}", message);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
}

#[tokio::main]
async fn main() {
    let app = App {
        client: Client::new(),
    };
    app.run();
}