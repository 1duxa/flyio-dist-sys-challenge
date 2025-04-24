use std::io::{StdoutLock, Write};

use anyhow::{bail, Context};
use serde::{Deserialize, Serialize};

/*
{
  "src": "n1",
  "dest": "c1",
  "body": {
    "type": "echo_ok",
    "msg_id": 1,
    "in_reply_to": 1,
    "echo": "Please echo 35"
  }
}
*/
#[derive(Serialize, Deserialize, Debug, Clone)]

struct Message {
    src: String,
    dest: String,
    body: MessageBody,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct MessageBody {
    msg_id: Option<usize>,
    in_reply_to: Option<usize>,
    #[serde(flatten)]
    payload: Payload,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Echo { echo: String },
    EchoOk {echo: String},
    Init {node_id:String, node_ids:Vec<String>},
    InitOk
}

struct EchoNode {
    id:usize
}
impl EchoNode {
    pub fn step(
        &mut self,
        input: Message,
        output: &mut StdoutLock,
    ) -> anyhow::Result<()> {
        match input.body.payload {
            Payload::Init { .. } => {
                let reply = Message {
                    src:input.dest,
                    dest:input.src,
                    body: MessageBody { 
                        msg_id: Some(self.id), 
                        in_reply_to: input.body.msg_id, 
                        payload: Payload::InitOk  }
                };
                serde_json::to_writer(&mut *output, &reply).context("serialize response to echo fail")?;
                output.write_all(b"\n").context("Write empty line")?;
                self.id += 1;
            }
            Payload::Echo { echo } => {
                let reply = Message {
                    src:input.dest,
                    dest:input.src,
                    body: MessageBody { 
                        msg_id: Some(self.id), 
                        in_reply_to: input.body.msg_id, 
                        payload: Payload::EchoOk { echo  } }
                };
                serde_json::to_writer(&mut *output, &reply).context("serialize response to echo fail")?;
                output.write_all(b"\n").context("Write empty line")?;
                self.id += 1;

            },
            Payload::InitOk => (), 
            Payload::EchoOk { .. } => (),
        }

        Ok(())
    }
}



fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin().lock();
    let mut stdout = std::io::stdout().lock();
    let inputs = serde_json::Deserializer::from_reader(stdin).into_iter::<Message>();

    let mut state = EchoNode {id: 0};
    for input in inputs {
        let input = input.context("Maelstorm input deseriallize fail")?;
        state.step(input, &mut stdout).context("Step fail")?;
    }
    Ok(())
}
