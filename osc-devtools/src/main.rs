use clap::{Parser, Subcommand};
use osc_ir::IrValue;

#[derive(Parser)]
#[command(name = "osc-dev")] 
#[command(about = "Round-trip demo for IR <-> JSON/MessagePack", long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    JsonRoundtrip,
    MsgpackRoundtrip,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let v = IrValue::String("hello".into());
    match cli.cmd {
        Cmd::JsonRoundtrip => {
            let j = osc_codec_json::to_json(&v);
            let v2 = osc_codec_json::from_json(&j);
            println!("{:?} -> {} -> {:?}", v, j, v2);
        }
        Cmd::MsgpackRoundtrip => {
            let bytes = osc_codec_msgpack::to_msgpack(&v);
            let v2 = osc_codec_msgpack::from_msgpack(&bytes);
            println!("{:?} -> {} bytes -> {:?}", v, bytes.len(), v2);
        }
    }
    Ok(())
}
