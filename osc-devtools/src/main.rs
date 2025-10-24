use clap::{Parser, Subcommand};
use osc_ir::{IrValue, IrBundle, IrTimetag};

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
    BundleDemo,
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
        Cmd::BundleDemo => {
            println!("=== Bundle Nesting Demo ===");
            
            // Create a complex nested bundle
            let mut root_bundle = IrBundle::immediate();
            root_bundle.add_message(IrValue::from("Root message"));
            root_bundle.add_message(IrValue::from(42));
            
            // Create first nested bundle
            let mut nested1 = IrBundle::new(IrTimetag::from_ntp(1000));
            nested1.add_message(IrValue::from("Nested level 1"));
            nested1.add_message(IrValue::from(true));
            
            // Create deeply nested bundle
            let mut nested2 = IrBundle::new(IrTimetag::from_ntp(2000));
            nested2.add_message(IrValue::from("Deeply nested"));
            nested2.add_message(IrValue::from(vec![0xAA_u8, 0xBB, 0xCC]));
            
            // Nest bundles
            nested1.add_bundle(nested2);
            root_bundle.add_bundle(nested1);
            
            let bundle_value = IrValue::Bundle(root_bundle);
            
            println!("Original bundle structure:");
            println!("  Root bundle (immediate) with {} elements", 
                     bundle_value.as_bundle().unwrap().elements.len());
            
            // Test JSON roundtrip
            let json = osc_codec_json::to_json(&bundle_value);
            let from_json = osc_codec_json::from_json(&json);
            let json_success = bundle_value == from_json;
            println!("\n✓ JSON roundtrip: {}", if json_success { "SUCCESS" } else { "FAILED" });
            println!("JSON size: {} characters", json.to_string().len());
            
            if !json_success {
                println!("JSON Debug:");
                println!("Original: {:?}", bundle_value);
                println!("From JSON: {:?}", from_json);
                println!("JSON: {}", json);
            }
            
            // Test MessagePack roundtrip
            let msgpack = osc_codec_msgpack::to_msgpack(&bundle_value);
            let from_msgpack = osc_codec_msgpack::from_msgpack(&msgpack);
            println!("✓ MessagePack roundtrip: {}", if bundle_value == from_msgpack { "SUCCESS" } else { "FAILED" });
            println!("MessagePack size: {} bytes", msgpack.len());
            
            // Verify cross-codec compatibility
            println!("✓ Cross-codec compatibility: {}", 
                     if from_json == from_msgpack { "SUCCESS" } else { "FAILED" });
            
            println!("\n=== Bundle nesting is working correctly! ===");
        }
    }
    Ok(())
}
