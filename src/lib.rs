#![no_std]
#![allow(unsafe_op_in_unsafe_fn)]
use core::sync::atomic::{AtomicU32, Ordering};
use forge::log::prelude::*;
mod rules;
static BASE: AtomicU32 = AtomicU32::new(0);
unsafe fn r(a: u32) -> u32 { core::ptr::read_volatile(a as *const u32) }
unsafe fn byte(a: u32) -> u8 { core::ptr::read_volatile(a as *const u8) }
unsafe fn w(a: u32, v: u32) { core::ptr::write_volatile(a as *mut u32, v); }
fn addr(off: u32) -> u32 { BASE.load(Ordering::Relaxed) + off }
fn valid(p: u32) -> bool { p >= 0x08000000 && p < 0xa8000000 && p & 3 == 0 }

unsafe fn active() -> bool {
    let net = r(addr(0x18843f0));
    let game = r(addr(0x188ca48));
    let players = r(addr(0x1885b58));
    if !valid(net) || !valid(game) || !valid(players) || byte(net + 1598) != 0 || byte(players + 40028) > 1 { return false; }
    let quest: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(addr(0x3f8634));
    quest(game) == 1 && (0..4).filter(|i| valid(r(players + 28 + 4*i))).count() == 1
}
unsafe fn status(enemy: u32) -> u32 { if valid(enemy) { r(enemy + 0x1428) } else { 0 } }
unsafe fn clear(enemy: u32) {
    let s = status(enemy);
    if valid(s) { for i in 0..4 { w(s + 0x264 + 4*i, 0); } }
}

#[forge::hook(offset = 0xbb7a0)]
fn add_hate(enemy: u32, slot: u32, amount: u32) -> u32 {
    unsafe {
        let s = status(enemy);
        if active() && valid(s) && slot < 4 {
            let p = s + 0x264 + 4*slot;
            w(p, 0); return p;
        }
    }
    original!(enemy, slot, amount)
}
#[forge::hook(offset = 0xbbf98)]
fn update_hate(enemy: u32) -> u32 {
    let enabled = unsafe { active() };
    if enabled { unsafe { clear(enemy); } }
    let result = original!(enemy);
    if enabled { unsafe { clear(enemy); } }
    result
}
#[forge::hook(offset = 0xc5b14)]
fn select(enemy: u32, kind: u32, mode: u32, index: u32, bank: u32) -> u32 {
    let result = original!(enemy, kind, mode, index, bank);
    unsafe {
        if active() && byte(kind) == 2 {
            // The eligibility helper also drives companion combat/support awareness: leave it native.
            let request = rules::hunter_request([byte(kind), byte(mode), byte(index)]);
            for (p, v) in [(kind, request[0]), (mode, request[1]), (index, request[2])] {
                core::ptr::write_volatile(p as *mut u8, v);
            }
            return original!(enemy, kind, mode, index, bank);
        }
    }
    result
}
#[forge::hook(offset = 0xc6e14)]
fn target(enemy: u32, kind: u32, mode: u32, index: u32, bank: u32) -> u32 {
    let result = original!(enemy, kind, mode, index, bank);
    unsafe {
        let s = status(enemy);
        if bank < 2 && active() && valid(s) && byte(s + 560 + bank) == 2 {
            // Ask the native selector for a hunter; 255 remains no target when none is eligible.
            let request = rules::hunter_request([byte(s+560+bank), byte(s+562+bank), byte(s+564+bank)]);
            return original!(enemy, request[0] as u32, request[1] as u32, request[2] as u32, bank);
        }
    }
    result
}

#[forge::entry]
fn main() {
    BASE.store(forge::mem::text_addr(), Ordering::Relaxed);
    unsafe {
        for (off, want) in SIGNATURES {
            if r(addr(off)) != want { error!("[nocataggro] unsupported/conflicting code {:x}, disabled", off); return; }
        }
    }
    forge::install_hook!(forge::mem::text_addr(), add_hate);
    forge::install_hook!(forge::mem::text_addr(), update_hate);
    forge::install_hook!(forge::mem::text_addr(), select);
    forge::install_hook!(forge::mem::text_addr(), target);
    info!("[nocataggro] v0.1.0 offline solo Hunter/Prowler: AI companions excluded, incidental damage unchanged");
}

const SIGNATURES: [(u32, u32); 4] = [(0xbb7a0, 0xe3013428), (0xbbf98, 0xe92d4ff0), (0xc5b14, 0xe92d4ff0), (0xc6e14, 0xe92d4ff0)];
