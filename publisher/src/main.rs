use serde::{Deserialize, Serialize};
use store::{self, store::Store};
use std::thread;
use std::time::Duration;

#[derive(Serialize,Deserialize)]
pub struct Website {
    pub url:String,
    pub id:String
}

fn do_something() -> redis::RedisResult<()> {
    let mut s = Store::new().unwrap();

    let all_websites = s.get_all_website().unwrap();
    
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    
    for website in all_websites {
        let w = Website {
            url:website.url,
            id:website.id
        };
    
        let payload =  serde_json::to_string(&w).unwrap();
    
        let id:String = redis::cmd("XADD").arg("betteruptime:website").arg("*").arg("data").arg(payload).query(&mut con).unwrap();
        
        println!("{}",id);

    }

    Ok(())
}


fn main() {
    loop {
        let _ = do_something();

        // Sleeps the current thread for 3 minutes (180 seconds)
        // But all the websites in the DB in the queue after every 3 minutes
        thread::sleep(Duration::from_secs(3 * 60));
    }
}

