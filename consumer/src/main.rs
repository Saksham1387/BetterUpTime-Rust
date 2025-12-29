
use serde::{Deserialize, Serialize};
use store::{self, store::Store};
use redis::Value;
#[derive(Serialize,Deserialize)]
#[derive(Debug)]
pub struct Website {
    pub url:String,
    pub id:String
}


fn consume() -> redis::RedisResult<()>  {
    let mut s = Store::new().unwrap();
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    
    let result:redis::Value = redis::cmd("XREAD").arg("STREAMS").arg("betteruptime:website").arg("0").query(&mut con).unwrap();
    
    if let redis::Value::Array(streams) = result {
        for stream in streams {
            if let redis::Value::Array(stream_data) = stream {
                // stream_data[0] = stream name
                // stream_data[1] = array of messages
                if let Some(redis::Value::Array(messages)) = stream_data.get(1) {
                    for message in messages {
                        if let redis::Value::Array(msg_parts) = message {
                            // msg_parts[0] = message ID
                            // msg_parts[1] = field-value pairs
                            let msg_id = &msg_parts[0];
                            
                            if let Some(redis::Value::Array(fields)) = msg_parts.get(1) {
                                // fields come in pairs: [field_name, value, field_name, value, ...]
                                for chunk in fields.chunks(2) {
                                    if let [redis::Value::BulkString(field), redis::Value::BulkString(value)] = chunk {
                                        let field_str = String::from_utf8_lossy(field);
                                        let value_str = String::from_utf8_lossy(value);
                                        
                                        println!("Message ID: {:?}", msg_id);
                                        println!("Field: {}, Value: {}", field_str, value_str);
                                        
                                        // If the value is JSON, parse it
                                        if field_str == "data" {
                                            // Parse the JSON value here
                                            // let parsed: serde_json::Value = serde_json::from_str(&value_str)?;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    

    Ok(())

}

fn main() {

    let _ = consume();

    
   
}
