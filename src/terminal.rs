#![allow(unused)]

fn is_kitty() -> bool
{
    std::env::var("KITTY_WINDOW_ID").is_ok()
}

fn is_vte() -> bool
{
    std::env::var("VTE_VERSION").is_ok()
}

fn is_mintty() -> bool
{
    std::env::var("TERM")
        .is_ok_and(|term| term == "mintty" || term == "xterm" || term == "xterm-direct" || term == "mintty-direct")
}

fn is_iterm() -> bool
{
    std::env::var("TERM_PROGRAM")
        .is_ok_and(|term| term == "iTerm.app")
}

pub fn supports_colored_underline() -> bool
{
    false //is_kitty() || is_vte() || is_mintty() || is_iterm()
}