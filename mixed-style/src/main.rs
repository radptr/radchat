use std::{io, time::Duration};

use ratatui::layout::Alignment;
use ratatui::prelude::StatefulWidget;
use ratatui::style::Stylize;
use ratatui::text::{Line, Span, Text};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect, Size},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};
use std::hash::Hash;
use std::hash::Hasher;
use tui_scrollview::{ScrollView, ScrollViewState};

#[derive(Clone)]
struct BacklogMessage {
    timestamp: &'static str,
    sigil: &'static str,
    nick: &'static str,
    message: &'static str,
}

impl BacklogMessage {
    fn new(
        timestamp: &'static str,
        sigil: &'static str,
        nick: &'static str,
        message: &'static str,
    ) -> Self {
        Self {
            timestamp,
            sigil,
            nick,
            message,
        }
    }
}

struct Buffer {
    network: &'static str,
    channel: &'static str,
    topic: &'static str,
    /// (sigil, nick), ordered by channel rank.
    nicks: Vec<(&'static str, &'static str)>,
    messages: Vec<BacklogMessage>,
}

impl Buffer {
    fn new(
        network: &'static str,
        channel: &'static str,
        topic: &'static str,
        nicks: Vec<(&'static str, &'static str)>,
        messages: Vec<BacklogMessage>,
    ) -> Self {
        Self {
            network,
            channel,
            topic,
            nicks,
            messages,
        }
    }
}

/// Backlogs keyed on buffer channel name.
/// Backlogs keyed on buffer channel name.
fn demo_buffers() -> Vec<Buffer> {
    vec![
        Buffer::new(
            "Libera",
            "#somechannel",
            "Welcome to #somechannel!",
            vec![
                ("&", "hallward"),
                ("@", "sibyl"),
                ("%", "masser"),
                ("+", "nyari"),
                ("+", "rp"),
                ("+", "corvid"),
                ("", "john"),
                ("", "quad"),
                ("", "vesper"),
                ("", "orin"),
                ("", "dahlia"),
                ("+", "Super_Long_Nickname_Yeah"),
                ("+", "Second_Super_Long_Nickname_Yeah"),
            ],
            somechannel_messages(),
        ),
        Buffer::new(
            "Libera",
            "#chatting",
            "Welcome to #chatting!",
            vec![
                ("&", "kelemenn"),
                ("@", "joe"),
                ("%", "zsofi"),
                ("", "nick1"),
                ("", "bence"),
                ("", "tamasb"),
            ],
            chatting_messages(),
        ),
        Buffer::new(
            "Libera",
            "#snailcatting",
            "Welcome to #snailcatting!",
            vec![
                ("~", "soodlim"),
                ("@", "limax"),
                ("+", "helixp"),
                ("", "moss"),
                ("", "petr"),
                ("", "vermes"),
                ("", "deutero"),
            ],
            snailcatting_messages(),
        ),
        Buffer::new(
            "Libera",
            "#music",
            "Next stream: mid-december!",
            vec![
                ("@", "crate"),
                ("+", "verdant"),
                ("+", "timpani"),
                ("", "pome"),
                ("", "sable"),
                ("", "goose"),
                ("", "nereid"),
                ("", "cassingle"),
            ],
            music_messages(),
        ),
        Buffer::new(
            "Libera",
            "#coffee",
            "Roast dates matter!",
            vec![
                ("@", "kaldi"),
                ("+", "basil"),
                ("", "tala"),
                ("", "moth"),
                ("", "percolata"),
                ("", "ginny"),
            ],
            coffee_messages(),
        ),
        Buffer::new(
            "OFTC",
            "#artix",
            "Welcome to #artix!",
            vec![
                ("@", "judd"),
                ("+", "cor"),
                ("+", "dinit"),
                ("", "sgn"),
                ("", "novak"),
                ("", "veselin"),
                ("", "vesuv"),
            ],
            artix_messages(),
        ),
        Buffer::new(
            "OFTC",
            "#debian",
            "Read the autoremove list before y+enter",
            vec![
                ("@", "aph"),
                ("+", "kev"),
                ("+", "sbuild"),
                ("", "wunn"),
                ("", "mifty"),
                ("", "tix"),
                ("", "guerin"),
                ("", "lintian"),
            ],
            debian_messages(),
        ),
    ]
}

fn somechannel_messages() -> Vec<BacklogMessage> {
    vec![
        BacklogMessage::new(
            "06:35",
            "",
            "john",
            "don't think building space ships and submarines is illegal",
        ),
        BacklogMessage::new(
            "06:36",
            "",
            "john",
            "even an ICBM is probably legal as long as u don't attempt to make a warhead or sell the plans to enemy states",
        ),
        BacklogMessage::new("06:55", "+", "nyari", "!link"),
        BacklogMessage::new(
            "06:55",
            "+",
            "somebot",
            " ≨ 鹭 䖇^哮^㺯⋕∤鋇⊘玷⊽^⊸^⋒囟≽ https://some.com/link.txt  嬈 ^⋈䙇⋟㢗^⊨≎囟^⋖^蔰㲷晧㔇⋕^⊂^",
        ),
        BacklogMessage::new("06:56", "%", "masser", "what are good halloween movies?"),
        BacklogMessage::new("07:12", "", "quad", "that one"),
        BacklogMessage::new("07:12", "", "quad", "john: there it's a national sport"),
        BacklogMessage::new(
            "07:13",
            "+",
            "rp",
            "according to a random list I found online, the first 3 are pretty good",
        ),
        BacklogMessage::new("07:25", "%", "masser", "i mean movies for halloween"),
        BacklogMessage::new("07:25", "%", "masser", "not the halloween franchise itself"),
        BacklogMessage::new(
            "07:26",
            "+",
            "nyari",
            "masser: does the 1978 original count, or are we past that",
        ),
        BacklogMessage::new(
            "07:26",
            "%",
            "masser",
            "the 1978 one counts, everything after the third one is negotiable",
        ),
        BacklogMessage::new(
            "07:27",
            "",
            "quad",
            "watch something bad on purpose, way funnier with a group",
        ),
        BacklogMessage::new(
            "07:28",
            "+",
            "rp",
            "quad: we did that with The Room last year and nobody spoke to each other for a day",
        ),
        BacklogMessage::new(
            "07:29",
            "",
            "vesper",
            "try the 1922 Nosferatu, the public domain prints are on the archive",
        ),
        BacklogMessage::new(
            "07:31",
            "",
            "orin",
            "vampire movies with real sunlight rules only, no exceptions",
        ),
        BacklogMessage::new("08:02", "+", "nyari", "!link"),
        BacklogMessage::new(
            "08:02",
            "+",
            "somebot",
            " [wiki] https://en.wikipedia.org/wiki/Halloween_(1978_film)  release dates plus the whole reshoot story",
        ),
        BacklogMessage::new(
            "08:04",
            "",
            "dahlia",
            "the reshoot story is wild, they shot the whole thing in 21 days",
        ),
        BacklogMessage::new(
            "08:05",
            "&",
            "hallward",
            "quiet hours start at 22:00, please keep the screaming in the other channel",
        ),
        BacklogMessage::new(
            "08:06",
            "@",
            "sibyl",
            "hallward: the screaming is the whole point of the thread",
        ),
        BacklogMessage::new(
            "08:07",
            "",
            "corvid",
            "sibyl: it's an echo chamber with better lighting",
        ),
    ]
}

fn chatting_messages() -> Vec<BacklogMessage> {
    vec![
        BacklogMessage::new(
            "04:47",
            "",
            "nick1",
            "joe: irssi vagy weechat uit preferalod? esetleg mindketto? vagy hide buflist/nicklist ha nincs eleg hely?",
        ),
        BacklogMessage::new(
            "04:48",
            "@",
            "joe",
            "nekem a weechat ui nagy reszt jobban tetszett, de vegul meg is az irssinel maradtam",
        ),
        BacklogMessage::new(
            "04:48",
            "@",
            "joe",
            "a buflist es a nicklist is tetszett, bar a fuggoleges buflistben nem sok mindent latni egyszerre",
        ),
        BacklogMessage::new(
            "04:48",
            "@",
            "joe",
            "viszont az is a ui resze, hogy egy halozat serverlistaja egy vesszovel elvalasztott lista egy config stringben, es az annyira nem jott be",
        ),
        BacklogMessage::new("04:48", "", "nick1", "megcsinalom a main view mockupot"),
        BacklogMessage::new("04:49", "", "nick1", "toml lesz a config sztem"),
        BacklogMessage::new("04:49", "", "nick1", "vagy hocon"),
        BacklogMessage::new(
            "04:49",
            "@",
            "joe",
            "az meg meg a weechatbol is hianyzott, hogy hierarchikus legyen a buflist, tehat hogy a network ala legyen rendelve a channel",
        ),
        BacklogMessage::new("04:49", "", "nick1", "az lesz ofc"),
        BacklogMessage::new("04:49", "", "nick1", "nem tudom miert nem olyan weechatben"),
        BacklogMessage::new(
            "04:49",
            "@",
            "joe",
            "gondolom, tul kozelrol kovettek az irssit",
        ),
        BacklogMessage::new(
            "04:50",
            "@",
            "nick1",
            "arra gondoltam meg hogy lehetne egy overview screen ahol kiirja az osszes buffert + hany olvasatlan uzeneted van",
        ),
        BacklogMessage::new(
            "04:50",
            "@",
            "joe",
            "az hatarozottan elonyos a weechatben, hogy ujra tudja formazni az egesz buffert, pl. ha theme-et valtasz",
        ),
        BacklogMessage::new("04:50", "@", "joe", "az irssi nem tud ilyet"),
        BacklogMessage::new(
            "04:50",
            "",
            "nick1",
            "es akkor egyszeruen atvaltasz arra binddel",
        ),
        BacklogMessage::new("04:50", "@", "joe", "jol hangzik"),
        BacklogMessage::new(
            "04:51",
            "%",
            "zsofi",
            "en inkabb arra vagyok kivancsi, mi lesz ha tul sok az olvasatlan buffer, ki kell-e nyomni valahol",
        ),
        BacklogMessage::new(
            "04:51",
            "@",
            "joe",
            "gondolom a headerbe kerul egy counter, bufferenkent mutatva",
        ),
        BacklogMessage::new(
            "04:52",
            "",
            "bence",
            "nekem az kell, hogy keybinddel a kovetkezo olvasatlan bufferre tudjak ugrani",
        ),
        BacklogMessage::new(
            "04:52",
            "",
            "tamasb",
            "weechatben a buffers.pl csinalja, de az egy konyvtari szintu hack inkabb",
        ),
        BacklogMessage::new(
            "04:53",
            "",
            "nick1",
            "berakjuk a state-be az unread countot bufferenkent, az a legkisebb valtozas",
        ),
        BacklogMessage::new("04:53", "@", "joe", "jo, es a header azt olvassa le"),
        BacklogMessage::new(
            "04:54",
            "%",
            "zsofi",
            "meg szinezze a buffer nevet a listaban, piros ha mention volt",
        ),
        BacklogMessage::new(
            "04:55",
            "",
            "bence",
            "mention az ha a nickem szerepel, highlight meg lehet barmi regex",
        ),
        BacklogMessage::new(
            "04:56",
            "",
            "tamasb",
            "akkor ket szint kell a listaban: olvasatlan es mention",
        ),
        BacklogMessage::new(
            "04:57",
            "@",
            "joe",
            "fel is irom: unread count bufferenkent, mention count, es a buffernev szine",
        ),
        BacklogMessage::new("04:58", "", "nick1", "ez mind bekerul a mockupba estere"),
        BacklogMessage::new(
            "04:58",
            "&",
            "kelemenn",
            "akkor holnap a theme rendszert beszeljuk meg, az a kovetkezo nagy tema",
        ),
    ]
}

fn snailcatting_messages() -> Vec<BacklogMessage> {
    vec![
        BacklogMessage::new(
            "21:48",
            "",
            "moss",
            "everyone be quiet the snail is on camera",
        ),
        BacklogMessage::new("21:48", "", "soodlim", "hi what did I miss"),
        BacklogMessage::new("21:48", "", "moss", "THE SNAIL IS ON CAMERA RIGHT NOW"),
        BacklogMessage::new("21:49", "", "petr", "gastropod cinema is back on the air"),
        BacklogMessage::new(
            "21:51",
            "",
            "soodlim",
            "why does it look like it yawned. can snails yawn",
        ),
        BacklogMessage::new(
            "21:51",
            "",
            "petr",
            "they do a lil stretch of the mouthparts yes",
        ),
        BacklogMessage::new(
            "21:55",
            "~",
            "soodlim",
            "mandatory watch: https://video.example.org/w/snail-yawned-169h4t",
        ),
        BacklogMessage::new("21:56", "", "moss", "re WATCHING WITH SOUND ON"),
        BacklogMessage::new(
            "21:57",
            "",
            "vermes",
            "he has been sitting on that same lettuce leaf for six hours",
        ),
        BacklogMessage::new("21:57", "", "petr", "that's commitment to a bit"),
        BacklogMessage::new(
            "21:58",
            "",
            "moss",
            "petition to have the camera running 24/7",
        ),
        BacklogMessage::new(
            "21:59",
            "@",
            "limax",
            "the bandwidth bill says no but the heart says yes",
        ),
        BacklogMessage::new(
            "22:01",
            "",
            "deutero",
            "fun fact: snails can sleep for like three days straight",
        ),
        BacklogMessage::new(
            "22:01",
            "+",
            "helixp",
            "so it's not dead, and yes I checked three times",
        ),
        BacklogMessage::new(
            "22:02",
            "~",
            "soodlim",
            "we have all been there, sometimes a snail just needs a long weekend",
        ),
        BacklogMessage::new("22:04", "", "petr", "he moved!! everyone act natural"),
        BacklogMessage::new(
            "22:05",
            "",
            "moss",
            "INCHING TOWARDS THE CUCUMBER, LADIES AND GENTLEMEN",
        ),
        BacklogMessage::new(
            "22:07",
            "@",
            "limax",
            "slow news day and I mean that in the most literal sense",
        ),
        BacklogMessage::new("22:09", "", "deutero", "clip it or it didn't happen"),
        BacklogMessage::new(
            "22:10",
            "+",
            "helixp",
            "soodlim: recording the full session for the archive",
        ),
    ]
}

fn artix_messages() -> Vec<BacklogMessage> {
    vec![
        BacklogMessage::new(
            "09:12",
            "",
            "sgn",
            "so it's arch without systemd, right? anything else I should know before switching?",
        ),
        BacklogMessage::new(
            "09:14",
            "+",
            "cor",
            "openrc, runit, s6 and dinit are all supported, you pick one at install time",
        ),
        BacklogMessage::new(
            "09:15",
            "",
            "sgn",
            "dinit looks nice, is the wiki any good?",
        ),
        BacklogMessage::new(
            "09:15",
            "+",
            "cor",
            "most of the arch wiki still applies, only service management differs",
        ),
        BacklogMessage::new("09:16", "", "novak", "how do I get sway on here?"),
        BacklogMessage::new("09:17", "", "sgn", "pacman -S sway, same as arch"),
        BacklogMessage::new("09:21", "", "cor", "classic novak"),
        BacklogMessage::new(
            "09:23",
            "",
            "veselin",
            "is the qlite installer still the recommended path, or should I hand-partition",
        ),
        BacklogMessage::new(
            "09:24",
            "+",
            "cor",
            "qlite is fine for a basic setup, cfdisk works if you like pain",
        ),
        BacklogMessage::new(
            "09:25",
            "",
            "sgn",
            "I picked runit on the desktop and s6 on the laptop, both just work",
        ),
        BacklogMessage::new(
            "09:26",
            "@",
            "judd",
            "the point of the project is choice, pick what your fingers remember",
        ),
        BacklogMessage::new(
            "09:27",
            "",
            "novak",
            "waydroid question: does anything init-specific break",
        ),
        BacklogMessage::new(
            "09:28",
            "",
            "vesuv",
            "novak: waydroid is fine, it's the old nvidia cards that suffer",
        ),
        BacklogMessage::new(
            "09:29",
            "",
            "sgn",
            "artix is basically arch where you own your init",
        ),
        BacklogMessage::new(
            "09:30",
            "+",
            "dinit",
            "and no systemd creep, that is the entire hill",
        ),
        BacklogMessage::new(
            "09:31",
            "+",
            "cor",
            "dinit has no bias here, banner still says artix either way",
        ),
    ]
}

fn music_messages() -> Vec<BacklogMessage> {
    vec![
        BacklogMessage::new(
            "13:01",
            "+",
            "verdant",
            "playlist for friday's stream is locked, thanks for all the submissions",
        ),
        BacklogMessage::new(
            "13:02",
            "",
            "pome",
            "did my hour of embarrassingly loud grindcore make the cut?",
        ),
        BacklogMessage::new(
            "13:03",
            "+",
            "verdant",
            "yes, right after the whale sounds interlude",
        ),
        BacklogMessage::new(
            "13:03",
            "",
            "sable",
            "btw that whale interlude goes HARD, don't change a thing",
        ),
        BacklogMessage::new(
            "13:05",
            "",
            "pome",
            "the transition from gurgling back into the blast beats was our best work",
        ),
        BacklogMessage::new(
            "13:07",
            "",
            "sable",
            "when is the winter stream? I want to finish mine early this time",
        ),
        BacklogMessage::new(
            "13:08",
            "+",
            "verdant",
            "mid-december, post songs in the usual spreadsheet",
        ),
        BacklogMessage::new(
            "13:09",
            "",
            "pome",
            "sable: whale sounds then grindcore is a valid album arc, fight me",
        ),
        BacklogMessage::new(
            "13:10",
            "@",
            "crate",
            "the spreadsheet has a covers tab now, if anyone's doing covers",
        ),
        BacklogMessage::new(
            "13:11",
            "+",
            "timpani",
            "crate: do timpani solos count as covers or as war crimes",
        ),
        BacklogMessage::new("13:12", "@", "crate", "timpani: yes"),
        BacklogMessage::new(
            "13:13",
            "",
            "goose",
            "my submission is 40 minutes of one sustained synth note, for artistic reasons",
        ),
        BacklogMessage::new(
            "13:14",
            "",
            "nereid",
            "goose I listened to the demo and my dog left the room",
        ),
        BacklogMessage::new(
            "13:16",
            "",
            "cassingle",
            "does anyone have the bitrate settings for the rip, I lost mine",
        ),
        BacklogMessage::new(
            "13:17",
            "@",
            "crate",
            "cassingle: whatever you use, don't submit a wav the size of a house",
        ),
        BacklogMessage::new(
            "13:19",
            "",
            "sable",
            "reminder that all timings are in UTC and 'whenever' is not a timezone",
        ),
        BacklogMessage::new("13:20", "", "pome", "'whenever' is GMT+me"),
        BacklogMessage::new(
            "13:21",
            "+",
            "verdant",
            "locked at that attitude, the interlude slot is yours",
        ),
    ]
}

fn coffee_messages() -> Vec<BacklogMessage> {
    vec![
        BacklogMessage::new(
            "17:40",
            "",
            "tala",
            "pulled a shot that tasted like battery acid, did I overextract?",
        ),
        BacklogMessage::new("17:41", "+", "basil", "how long were the beans roasted?"),
        BacklogMessage::new(
            "17:42",
            "",
            "tala",
            "roast date says may, so yeah that would explain it",
        ),
        BacklogMessage::new(
            "17:43",
            "+",
            "basil",
            "may is ancient for espresso, that explains the battery acid",
        ),
        BacklogMessage::new(
            "17:44",
            "",
            "tala",
            "fresh beans will fix 90% of my espresso crimes then, ordering today",
        ),
        BacklogMessage::new(
            "17:45",
            "+",
            "basil",
            "good, report back with a grind setting and a new shot",
        ),
        BacklogMessage::new(
            "17:46",
            "",
            "tala",
            "ordered: 2kg Ethiopian from a roaster 40 minutes away, roast date last tuesday",
        ),
        BacklogMessage::new(
            "17:47",
            "+",
            "basil",
            "that's the move, use it up inside three weeks",
        ),
        BacklogMessage::new(
            "17:48",
            "@",
            "kaldi",
            "if the roaster is that close, ask what they pull for their own espresso",
        ),
        BacklogMessage::new(
            "17:49",
            "",
            "moth",
            "my barista says espresso blends want 5-10 days of rest after the roast",
        ),
        BacklogMessage::new(
            "17:50",
            "",
            "percolata",
            "resting is real, day-2 fresh shots taste like carbonation and regret",
        ),
        BacklogMessage::new(
            "17:51",
            "",
            "ginny",
            "meanwhile I drink a dark roast that's been in my freezer since the last eclipse",
        ),
        BacklogMessage::new(
            "17:52",
            "+",
            "basil",
            "ginny the freezer question starts wars in here, tread carefully",
        ),
        BacklogMessage::new(
            "17:53",
            "@",
            "kaldi",
            "freezing is fine IF it's airtight and you thaw it zero times",
        ),
        BacklogMessage::new(
            "17:54",
            "",
            "ginny",
            "it's in the original valve bag, the valve is a decoration now",
        ),
        BacklogMessage::new(
            "17:55",
            "",
            "tala",
            "new beans arrived mid-conversation, grinding coarser tonight",
        ),
        BacklogMessage::new("17:56", "", "moth", "update us with photos of the puck"),
        BacklogMessage::new(
            "17:58",
            "",
            "tala",
            "puck came out clean, shot is chocolatey, everything is forgiven",
        ),
    ]
}

fn debian_messages() -> Vec<BacklogMessage> {
    vec![
        BacklogMessage::new(
            "11:02",
            "",
            "wunn",
            "apt upgrade wanted to remove 300 packages, is that normal?",
        ),
        BacklogMessage::new(
            "11:04",
            "+",
            "kev",
            "you probably have a big autoremove pending, read the list before y+enter",
        ),
        BacklogMessage::new(
            "11:05",
            "",
            "wunn",
            "half of it is old kernels, thank god, the other half is php",
        ),
        BacklogMessage::new(
            "11:06",
            "",
            "mifty",
            "anyone running the new release on a thinkpad yet?",
        ),
        BacklogMessage::new("11:07", "+", "kev", "t14 gen 5 works out of the box"),
        BacklogMessage::new(
            "11:09",
            "",
            "wunn",
            "suspend broke on mine but the usual firmware package fixed it",
        ),
        BacklogMessage::new(
            "11:10",
            "",
            "guerin",
            "wunn: check /etc/apt/sources.list.d before blaming apt, a stale third-party repo does exactly that",
        ),
        BacklogMessage::new(
            "11:11",
            "",
            "tix",
            "is firmware-nonfree still held back or is that sorted in stable now",
        ),
        BacklogMessage::new(
            "11:12",
            "+",
            "kev",
            "tix: non-free-firmware is its own component since bookworm, add it and the installer stops crying",
        ),
        BacklogMessage::new(
            "11:14",
            "@",
            "aph",
            "if you run testing on a stable box, pin it properly, half-pinned is how you get 300 removals",
        ),
        BacklogMessage::new(
            "11:14",
            "",
            "lintian",
            "saw that exact thing at a release party once, we ordered pizza and waited",
        ),
        BacklogMessage::new(
            "11:15",
            "",
            "mifty",
            "thinkpad update: battery dropped 8% during suspend, worth a bug report?",
        ),
        BacklogMessage::new(
            "11:16",
            "+",
            "kev",
            "check the kernel changelog first, there was a powersave regression tagged for this month",
        ),
        BacklogMessage::new(
            "11:18",
            "",
            "wunn",
            "upgraded clean after tidying the sources, thanks all",
        ),
    ]
}

#[derive(Clone, Copy, PartialEq)]
enum ViewMode {
    Normal,
    /// Two buffers stacked top / bottom.
    SplitHorizontal,
    /// Two buffers side by side.
    SplitVertical,
    /// Four buffers in a 2x2 grid.
    SplitQuad,
}

struct State {
    view: ViewMode,
    /// Index into the buffer list of the focused buffer.
    focused: usize,
    buffers: Vec<Buffer>,
}

impl State {
    fn focused_buffer(&self) -> &Buffer {
        &self.buffers[self.focused.min(self.buffers.len() - 1)]
    }

    fn move_focus(&mut self, key: KeyCode) {
        match self.view {
            // Cycle through the whole buffer list.
            ViewMode::Normal => {
                let n = self.buffers.len();
                match key {
                    KeyCode::Left | KeyCode::Up => {
                        self.focused = (self.focused + n - 1) % n;
                    }
                    KeyCode::Right | KeyCode::Down => {
                        self.focused = (self.focused + 1) % n;
                    }
                    _ => {}
                }
            }
            // Two visible panes: any arrow jumps to the other one.
            ViewMode::SplitHorizontal | ViewMode::SplitVertical => {
                self.focused = 1 - self.focused.min(1);
            }
            // 2x2 grid: bit 0 is the column, bit 1 the row.
            ViewMode::SplitQuad => match key {
                KeyCode::Left | KeyCode::Right => self.focused ^= 1,
                KeyCode::Up | KeyCode::Down => self.focused ^= 2,
                _ => {}
            },
        }
    }
}

#[derive(Default)]
struct RedrawControl {
    /// The rendered frame is out of date and the terminal should be redrawn.
    dirty: bool,
    /// A burst of Resize events is still arriving; drawing during it causes
    /// flicker, so frames are suppressed until the burst settles.
    resizing: bool,
    /// Size of the last frame actually drawn.
    drawn_size: Option<(u16, u16)>,
    /// Size from the latest resize event while a burst is in flight.
    pending_size: Option<(u16, u16)>,
}

impl RedrawControl {
    const IDLE_POLL: Duration = Duration::from_millis(500);

    /// The terminal starts with no frame presented yet.
    fn unrendered() -> Self {
        Self {
            dirty: true,
            resizing: false,
            drawn_size: None,
            pending_size: None,
        }
    }

    /// Any key press invalidates the frame.
    fn key_event(&mut self) {
        self.dirty = true;
    }

    /// Resize events arrive in bursts; swallow them until the burst ends.
    /// Only the final size matters, so each event overwrites the pending one.
    fn resize_event(&mut self, width: u16, height: u16) {
        self.resizing = true;
        self.pending_size = Some((width, height));
    }

    /// Poll quickly to drain the resize queue, slowly when idle.
    fn poll_timeout(&self) -> Duration {
        if self.resizing {
            Duration::ZERO
        } else {
            Self::IDLE_POLL
        }
    }

    /// An idle poll means no more resize events are queued: the burst is
    /// over. Redraw only if the final size differs from the drawn frame (or
    /// something else made the frame stale in the meantime).
    fn poll_timed_out(&mut self) {
        if self.resizing {
            self.resizing = false;
            if self.drawn_size != self.pending_size {
                self.dirty = true;
            }
            self.pending_size = None;
        }
    }

    fn should_draw(&self) -> bool {
        self.dirty && !self.resizing
    }

    fn drawn(&mut self, width: u16, height: u16) {
        self.dirty = false;
        self.drawn_size = Some((width, height));
    }
}

fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut state = State {
        view: ViewMode::Normal,
        focused: 0,
        buffers: demo_buffers(),
    };

    let mut control = RedrawControl::unrendered();

    loop {
        if control.should_draw() {
            terminal.draw(|frame| render(frame, &state))?;
            let size = terminal.size()?;
            control.drawn(size.width, size.height);
        }

        if event::poll(control.poll_timeout())? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        let toggle_to = |current, target| {
                            if current == target {
                                ViewMode::Normal
                            } else {
                                target
                            }
                        };

                        match key.code {
                            KeyCode::Char('q') => break,
                            KeyCode::Char('h') => {
                                state.view = toggle_to(state.view, ViewMode::SplitHorizontal);
                                state.focused = state.focused.min(1);
                            }
                            KeyCode::Char('v') => {
                                state.view = toggle_to(state.view, ViewMode::SplitVertical);
                                state.focused = state.focused.min(1);
                            }
                            KeyCode::Char('g') => {
                                state.view = toggle_to(state.view, ViewMode::SplitQuad);
                                state.focused = state.focused.min(3);
                            }
                            KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down => {
                                state.move_focus(key.code)
                            }
                            _ => {}
                        }

                        control.key_event();
                    }
                }
                Event::Resize(width, height) => control.resize_event(width, height),
                _ => {}
            }
        } else {
            control.poll_timed_out();
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    ratatui::run(run)
}

fn dark_background() -> Color {
    Color::Rgb(40, 40, 40)
}

fn box_background() -> Color {
    Color::Rgb(29, 32, 33)
}

fn selection_background() -> Color {
    Color::Rgb(60, 56, 54)
}

fn selection_foreground() -> Color {
    Color::White
}

const TIMESTAMP_WIDTH: u16 = 6;

/// Widest `<{sigil}{nick}> ` line over all messages: nick + sigil + "<>" and a
/// trailing space.
fn max_nick_len(messages: &[BacklogMessage]) -> u16 {
    messages.iter().fold(0u16, |accum, item| {
        let styling_len = 3 + item.sigil.chars().count() as u16;
        let curr_len = item.nick.chars().count() as u16 + styling_len;
        if curr_len > accum { curr_len } else { accum }
    })
}

fn max_nicklist_nick_len(nicks: &[(&'static str, &'static str)]) -> u16 {
    nicks.iter().fold(20u16, |accum, &(sigil, nick)| {
        let styling_len = sigil.chars().count() as u16;
        let curr_len = nick.chars().count() as u16 + styling_len;
        if curr_len > accum { curr_len } else { accum }
    })
}

fn color_for_nick(nick: &str) -> Color {
    let colors: Vec<Color> = (17u8..=230).map(Color::Indexed).collect();

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    nick.hash(&mut hasher);
    let hash = hasher.finish() as usize;

    colors[hash % colors.len()]
}

fn render_buffer(frame: &mut Frame, buffer: &Buffer, area: Rect, _focused: bool) {
    let vert_layout =
        Layout::vertical(vec![Constraint::Length(1), Constraint::Fill(1)]).split(area);

    let horiz_layout =
        Layout::horizontal(vec![Constraint::Fill(1), Constraint::Length(20)]).split(vert_layout[1]);

    let block = Block::new()
        .borders(Borders::LEFT | Borders::RIGHT)
        .border_style(Style::default().fg(dark_background()))
        .bg(box_background());

    let mut nicklist_constraints = vec![];

    for &(sigil, nick) in &buffer.nicks {
        let lines = Paragraph::new(format!("{}{}", sigil, nick)).line_count(20);
        nicklist_constraints.push(Constraint::Length(lines as u16));
    }

    let mut y = horiz_layout[1].y;

    frame.render_widget(block, horiz_layout[1]);

    for (i, &(sigil, nick)) in buffer.nicks.iter().enumerate() {
        let color = color_for_nick(nick);
        let para = Paragraph::new(Text::from(vec![Line::from(vec![
            Span::raw(sigil),
            Span::styled(nick, Style::default().fg(color)),
        ])]))
        .wrap(Wrap { trim: false });
        let lines = para.line_count(20) as u16;

        let rect = Rect::new(
            horiz_layout[1].x + 1,
            y,
            horiz_layout[1].width.saturating_sub(3),
            lines,
        );

        frame.render_widget(
            para,
            rect,
        );

        y += lines;
    }

    let mut scroll_view = ScrollView::new(Size::new(horiz_layout[0].width, horiz_layout[0].height))
        .scrollbars_visibility(tui_scrollview::ScrollbarVisibility::Never);

    let mut scroll_state = ScrollViewState::default();

    // The message column is what the text actually wraps into; compute it from
    // the scroll view width minus the timestamp and nick columns. line_count()
    // must be called with this exact width or wrapped lines get cut off.
    let scroll_width = horiz_layout[0].width;
    let nick_len = max_nick_len(&buffer.messages);
    let message_width = scroll_width
        .saturating_sub(TIMESTAMP_WIDTH)
        .saturating_sub(nick_len);

    let paragraphs: Vec<_> = buffer
        .messages
        .iter()
        .map(|message| {
            let timestamp = Paragraph::new(message.timestamp);
            let sigil = message.sigil;
            let nick = message.nick;
            let message = Paragraph::new(message.message).wrap(Wrap { trim: false });

            let height = message
                .line_count(message_width)
                .clamp(1, u16::MAX as usize) as u16;

            (timestamp, sigil, nick, message, height)
        })
        .collect();

    let mut y = 0_u16;
    for (timestamp, sigil, nick, message, height) in paragraphs {
        let layout = Layout::horizontal(vec![
            Constraint::Length(TIMESTAMP_WIDTH),
            Constraint::Length(nick_len),
            // Constraint::Length((nick.len() + sigil.len() + 3) as u16),
            Constraint::Fill(1),
        ])
        .split(Rect::new(0, y, scroll_width, height));

        let color = color_for_nick(nick);
        let sigil = Span::styled(sigil, Style::default().fg(Color::White));

        let nick = if nick == "nick1" {
            Span::styled(nick, Style::default().fg(color).bold())
        } else {
            Span::styled(nick, Style::default().fg(color).not_bold())
        };

        scroll_view.render_widget(timestamp, layout[0]);
        scroll_view.render_widget(
            Paragraph::new(Text::from(vec![Line::from(vec![
                Span::raw("<"),
                sigil,
                nick,
                Span::raw("> "),
            ])]))
            .alignment(Alignment::Right),
            layout[1],
        );

        // let sep = Paragraph::new("│ ");
        // scroll_view.render_widget(sep, layout[2]);
        scroll_view.render_widget(message, layout[2]);

        y = y.saturating_add(height);
    }

    render_topic_bar(frame, buffer.topic, vert_layout[0]);

    let area = scroll_view.buf_mut().area;
    scroll_view
        .buf_mut()
        .set_style(area, Style::default().bg(box_background()));
    scroll_view.render(vert_layout[1], frame.buffer_mut(), &mut scroll_state);
}

fn render_topic_bar(frame: &mut Frame, topic: &str, area: Rect) {
    let topic = Paragraph::new(topic)
        .bg(dark_background())
        .fg(Color::White)
        .bold();

    frame.render_widget(topic, area);
}

fn render_status_bar(frame: &mut Frame, channel: &str, users: usize, area: Rect) {
    let status = Paragraph::new(Line::from(vec![
        Span::styled(format!("[{channel}]"), Style::default().bold()),
        Span::raw(" (+ntls)"),
        Span::styled(format!(" {users} "), Style::default().bold()),
        Span::raw("users"),
        Span::styled(" 4 ", Style::default().bold()),
        Span::raw("opped"),
        Span::styled(" 13 ", Style::default().bold()),
        Span::raw("voiced"),
    ]))
    .bg(dark_background())
    .fg(Color::White);

    frame.render_widget(status, area);
}

fn render_input_bar(frame: &mut Frame, area: Rect) {
    let inputbar = Paragraph::new(Text::from(Line::from(vec![
        Span::raw("> "),
        Span::raw("hello, world!"),
    ])))
    .bg(box_background());

    frame.render_widget(inputbar, area);
}

fn render_buffer_list(frame: &mut Frame, state: &State, area: Rect) {
    let block = Block::new()
        .borders(Borders::LEFT | Borders::RIGHT)
        .border_style(Style::default().fg(dark_background()))
        .bg(box_background());

    // One row per buffer, with a network header before each new network.
    let mut rows = vec![];
    let mut last_network: Option<&'static str> = None;

    for (i, buffer) in state.buffers.iter().enumerate() {
        if last_network != Some(buffer.network) {
            last_network = Some(buffer.network);
            rows.push(Row::Header(buffer.network));
        }
        rows.push(Row::Channel(i));
    }

    let layout = Layout::vertical(
        rows.iter()
            .map(|_| Constraint::Length(1))
            .collect::<Vec<_>>(),
    )
    .split(block.inner(area));

    frame.render_widget(block, area);

    for (i, row) in rows.iter().enumerate() {
        match *row {
            Row::Header(network) => {
                let para = Paragraph::new(network)
                    .bg(dark_background())
                    .fg(Color::White);
                frame.render_widget(para, layout[i]);
            }
            Row::Channel(index) => {
                let buffer = &state.buffers[index];
                let para = if index == state.focused {
                    Paragraph::new(format!("  {}", buffer.channel))
                        .fg(selection_foreground())
                        .bg(selection_background())
                } else {
                    Paragraph::new(format!("  {}", buffer.channel))
                };
                frame.render_widget(para, layout[i]);
            }
        }
    }
}

enum Row {
    Header(&'static str),
    Channel(usize),
}

fn render_normal_view(frame: &mut Frame, state: &State) {
    let layout = Layout::vertical(vec![
        Constraint::Fill(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .split(frame.area());

    let mut messages = Vec::new();
    for buffer in &state.buffers {
        messages.extend(buffer.messages.iter().cloned());
    }

    let main_view_layout =
        Layout::horizontal(vec![Constraint::Max(20), Constraint::Fill(1)]).split(layout[0]);

    render_buffer_list(frame, state, main_view_layout[0]);

    render_buffer(frame, state.focused_buffer(), main_view_layout[1], true);

    let focused = state.focused_buffer();
    render_status_bar(frame, focused.channel, focused.nicks.len(), layout[1]);
    render_input_bar(frame, layout[2]);
}

fn render_split_view(frame: &mut Frame, state: &State, mode: ViewMode) {
    let layout = Layout::vertical(vec![
        Constraint::Fill(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .split(frame.area());

    let main_view_layout =
        Layout::horizontal(vec![Constraint::Max(20), Constraint::Fill(1)]).split(layout[0]);

    render_buffer_list(frame, state, main_view_layout[0]);

    let panes = match mode {
        ViewMode::SplitQuad => 4,
        _ => 2,
    };

    let areas: Vec<Rect> = match mode {
        ViewMode::SplitHorizontal => {
            Layout::vertical(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(main_view_layout[1])
                .to_vec()
        }
        ViewMode::SplitVertical => {
            Layout::horizontal(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(main_view_layout[1])
                .to_vec()
        }
        ViewMode::SplitQuad => {
            let halves =
                Layout::vertical(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(main_view_layout[1]);

            let mut quads =
                Layout::horizontal(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(halves[0])
                    .to_vec();
            quads.extend(
                Layout::horizontal(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(halves[1])
                    .to_vec(),
            );
            quads
        }
        ViewMode::Normal => return,
    };

    for (i, area) in areas.iter().enumerate().take(panes) {
        render_buffer(frame, &state.buffers[i], *area, i == state.focused);
    }

    let focused = state.focused_buffer();
    render_status_bar(frame, focused.channel, focused.nicks.len(), layout[1]);
    render_input_bar(frame, layout[2]);
}

fn render(frame: &mut Frame, state: &State) {
    match state.view {
        ViewMode::Normal => render_normal_view(frame, state),
        other => render_split_view(frame, state, other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{Terminal, backend::TestBackend};

    /// Concatenates the non-whitespace characters of every cell in the test
    /// backend, row by row.
    fn rendered_chars(backend: &TestBackend) -> String {
        let mut out = String::new();
        for y in 0..backend.buffer().area.height {
            for x in 0..backend.buffer().area.width {
                let symbol = backend.buffer()[(x, y)].symbol().to_string();
                out.push_str(symbol.trim());
            }
        }
        out
    }

    fn squeezed(s: &str) -> String {
        s.chars().filter(|c| !c.is_whitespace()).collect::<String>()
    }

    /// Every message must be fully visible when rendered at its computed
    /// width and height. If line_count() were called with a wider width than
    /// the render area (the old bug), the wrapped tail would get cut off.
    #[test]
    fn wrapped_messages_fully_visible() {
        let scroll_width: u16 = 100;

        for buffer in demo_buffers() {
            let nick_len = max_nick_len(&buffer.messages);
            let message_width = scroll_width
                .saturating_sub(TIMESTAMP_WIDTH)
                .saturating_sub(nick_len);

            for message in &buffer.messages {
                let paragraph = Paragraph::new(message.message).wrap(Wrap { trim: false });
                let height = paragraph
                    .line_count(message_width)
                    .clamp(1, u16::MAX as usize) as u16;

                let backend = TestBackend::new(message_width.max(1), height);
                let mut terminal = Terminal::new(backend).unwrap();
                terminal
                    .draw(|frame| frame.render_widget(&paragraph, frame.area()))
                    .unwrap();

                assert_eq!(
                    rendered_chars(terminal.backend()),
                    squeezed(message.message),
                    "message cut off in {} at width {}",
                    buffer.channel,
                    message_width,
                );
            }
        }
    }

    /// Frames are drawn only when the frame is stale AND no resize burst is
    /// in flight; a settled resize triggers exactly one redraw, and only when
    /// the final size actually differs from the drawn frame.
    #[test]
    fn redraw_control_debounce() {
        // Initial frame.
        let mut c = RedrawControl::unrendered();
        assert!(c.should_draw());
        c.drawn(80, 24);
        assert!(!c.should_draw());

        // Idle without events: no redraws.
        assert!(!c.should_draw());

        // Resize to the SAME size: settling does not redraw on its own.
        c.resize_event(80, 24);
        assert_eq!(c.poll_timeout(), Duration::ZERO);
        c.poll_timed_out();
        assert!(!c.resizing);
        assert_eq!(c.poll_timeout(), RedrawControl::IDLE_POLL);
        assert!(!c.should_draw());

        // A key during that same-size burst still forces the redraw.
        c.resize_event(80, 24);
        c.key_event();
        c.poll_timed_out();
        assert!(c.should_draw());
        c.drawn(80, 24);
        assert!(!c.should_draw());

        // Resize to a NEW size: exactly one redraw at the final size.
        c.resize_event(100, 30);
        c.resize_event(100, 30);
        c.poll_timed_out();
        assert!(c.should_draw());
        c.drawn(100, 30);
        assert!(!c.should_draw());

        // Idle polls before any new event: no redraws.
        c.poll_timed_out();
        assert!(!c.should_draw());
    }

    /// The full buffer render (scroll view included) must contain each
    /// message's tail in the message column, proving no overlap clipping
    /// between backlog rows. Tall backend so vertical scrolling can't clip.
    #[test]
    fn buffer_render_keeps_message_tails() {
        let sizes: [(u16, u16); 3] = [(140, 200), (80, 200), (55, 200)];
        let nicklist_width: u16 = 20;

        for &(width, height) in &sizes {
            for buffer in demo_buffers() {
                let scroll_width = width.saturating_sub(nicklist_width);
                let nick_len = max_nick_len(&buffer.messages);
                let message_width = scroll_width
                    .saturating_sub(TIMESTAMP_WIDTH)
                    .saturating_sub(nick_len);
                let message_col = u16::from(TIMESTAMP_WIDTH + nick_len).min(scroll_width);

                let mut rendered = String::new();
                let backend = TestBackend::new(width, height);
                let mut terminal = Terminal::new(backend).unwrap();
                terminal
                    .draw(|frame| render_buffer(frame, &buffer, frame.area(), false))
                    .unwrap();

                for y in 1..height {
                    for x in message_col..message_col.saturating_add(message_width) {
                        let symbol = terminal.backend().buffer()[(x, y)].symbol().to_string();
                        rendered.push_str(symbol.trim());
                    }
                }

                for message in &buffer.messages {
                    let tail: String = message
                        .message
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .rev()
                        .take(8)
                        .collect::<Vec<_>>()
                        .into_iter()
                        .rev()
                        .collect();

                    assert!(
                        rendered.contains(&tail),
                        "tail {:?} of message in {} missing at {}x{}",
                        tail,
                        buffer.channel,
                        width,
                        height,
                    );
                }
            }
        }
    }
}
