//! baka
//!

#![allow(incomplete_features)]
#![feature(
    asm_const,
    asm_sym,
    asm_unwind,
    asm_experimental_arch,
    cfg_sanitize,
    cfg_target_abi,
    cfg_target_compact,
    cfg_target_has_atomic,
    cfg_target_has_atomic_equal_alignment,
    cfg_target_thread_local,
    cfg_version,
    async_closure,
    unboxed_closures,
    closure_lifetime_binder,
    closure_track_caller,
    extern_types,
    generic_arg_infer,
    generic_associated_types,
    const_async_blocks,
    const_eval_limit,
    const_extern_fn,
    const_fn_floating_point_arithmetic,
    const_for,
    const_mut_refs,
    const_precise_live_drops,
    const_refs_to_cell,
    const_trait_impl,
    const_try,
    generators,
    generator_trait,
    deprecated_safe,
    deprecated_suggestion,
    auto_traits,
    fn_traits,
    inline_const,
    inline_const_pat,
    decl_macro,
    box_syntax,
    box_patterns,
    try_blocks,
    if_let_guard,
    let_else,
    negative_impls,
    yeet_expr,
    exclusive_range_pattern,
    half_open_range_patterns,
    exhaustive_patterns,
    arbitrary_enum_discriminant,
    c_unwind,
    c_variadic
)]

use bakalib::command::CommandParser;
use bakalib::extensions::string::StringExtension;
use bakalib::io::{Read, Send};
use bakalib::protoutils;
use bakalib::socket::{Events, Socket, SocketBuilder};
use bakaproto::proto::*;

use std::io::prelude::*;
use std::sync::Mutex;

use lazy_static::lazy_static;
use protobuf::Message;

lazy_static! {
    static ref BOT_ID: Mutex<String> = Mutex::new(String::random(6));
    static ref BOT_NICKNAME: Mutex<String> =
        Mutex::new(format!("bot_{0}", BOT_ID.lock().unwrap().clone()).to_string());
    static ref BOT_TARGET: Mutex<String> =
        Mutex::new(format!(":{0}", BOT_NICKNAME.lock().unwrap().clone()).to_string());
}

fn main() -> std::io::Result<()> {
    let mut server = SocketBuilder::new(
        "127.0.0.1:20030", // obfstr::obfstr!("127.0.0.1:20030")
        Events {
            on_connect:
                box |s: &mut Socket, data: Result<message::Message, bakalib::socket::Error>| {
                    println!("Succefully connected!");

                    s.send_bytes(
                        protoutils::BakaMessage {
                            author: BOT_NICKNAME.lock().unwrap().clone(),
                            content: ":userserv register-session".to_string(),
                        }
                        .build()
                        .write_to_bytes()
                        .unwrap()
                        .to_vec(),
                    );
                },
            on_disconnect:
                box |s: &mut Socket, data: Result<message::Message, bakalib::socket::Error>| {
                    println!("Disconnected!");
                },
            on_error:
                box |s: &mut Socket, data: Result<message::Message, bakalib::socket::Error>| {
                    println!("Error");
                },
            on_message:
                box |s: &mut Socket, data: Result<message::Message, bakalib::socket::Error>| {
                    if let Ok(message) = data {
                        let mut parser = CommandParser::new(message.content.clone());
                        let bot_target = BOT_TARGET.lock().unwrap().as_str();

                        println!("#{0}@{1} [{2}] {3}", parser.target(), parser.command(),
                                                       message.author.as_str(), message.content.as_str());

                        match parser.target() {
                            bot_target => match parser.command() {
                                "enable-feature" => println!("+++ Server requests the client to enable this feature: {}", parser.args()[0].clone()),
                                "disable-feature" => println!("+++ Server requests the client to disable this feature: {}", parser.args()[0].clone())
                            }
                        }
                    }
                },
        },
    );

    server.startup();

    Ok(())
}
