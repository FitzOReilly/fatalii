use crate::uci_move::UciMove;
use crate::uci_option::{OptionType, UciOption, OPTIONS};
use crate::uci_score::UciScore;
use engine::{EngineOptions, EngineOut, Variant};
use movegen::r#move::Move;
use search::search::SearchResult;
use std::error::Error;
use std::io::Write;
use std::sync::{Arc, Mutex};

struct UciOutInner {
    writer: Box<dyn Write + Send>,
    engine_version: String,
    engine_options: Arc<Mutex<EngineOptions>>,
    debug: bool,
}

#[derive(Clone)]
pub struct UciOut {
    inner: Arc<Mutex<UciOutInner>>,
}

impl EngineOut for UciOut {
    fn info_depth_finished(
        &self,
        search_result: Option<SearchResult>,
    ) -> Result<(), Box<dyn Error>> {
        match search_result {
            Some(res) => match self.inner.lock() {
                Ok(mut inner) => {
                    let move_to_str: Box<dyn Fn(Move) -> String> = match inner.engine_options.lock()
                    {
                        Ok(opt) => match opt.variant {
                            Variant::Standard => Box::new(UciMove::move_to_str),
                            Variant::Chess960(king_rook, queen_rook) => Box::new(move |m| {
                                UciMove::move_to_str_chess_960(m, king_rook, queen_rook)
                            }),
                        },
                        Err(e) => panic!("{}", e),
                    };
                    let pv_str = res
                        .principal_variation()
                        .iter()
                        .take_while(|m| **m != Move::NULL)
                        .map(|m| move_to_str(*m))
                        .collect::<Vec<String>>()
                        .join(" ");
                    Ok(writeln!(
                        inner.writer,
                        "info depth {} seldepth {} score {} nodes {} nps {} time {} hashfull {} pv {}",
                        res.depth(),
                        res.selective_depth(),
                        UciScore::from(res.score()),
                        res.nodes(),
                        res.nodes_per_second(),
                        res.time_ms(),
                        res.hash_load_factor_permille(),
                        pv_str
                    )?)
                }
                Err(e) => {
                    self.info_string(format!("{e}").as_str())?;
                    panic!("{e}")
                }
            },
            None => Ok(()),
        }
    }

    fn info_string(&self, s: &str) -> Result<(), Box<dyn Error>> {
        match self.inner.lock() {
            Ok(mut inner) => match inner.debug {
                true => Ok(writeln!(inner.writer, "info string {s}")?),
                false => Ok(()),
            },
            Err(e) => panic!("{e}"),
        }
    }

    fn best_move(&self, search_result: Option<Move>) -> Result<(), Box<dyn Error>> {
        match search_result {
            Some(res) => match self.inner.lock() {
                Ok(mut inner) => {
                    let move_to_str: Box<dyn Fn(Move) -> String> = match inner.engine_options.lock()
                    {
                        Ok(opt) => match opt.variant {
                            Variant::Standard => Box::new(UciMove::move_to_str),
                            Variant::Chess960(king_rook, queen_rook) => Box::new(move |m| {
                                UciMove::move_to_str_chess_960(m, king_rook, queen_rook)
                            }),
                        },
                        Err(e) => panic!("{e}"),
                    };
                    Ok(writeln!(inner.writer, "bestmove {}", move_to_str(res))?)
                }
                Err(e) => {
                    self.info_string(format!("{e}").as_str())?;
                    panic!("{e}");
                }
            },
            None => Ok(()),
        }
    }
}

impl UciOut {
    pub fn new(
        writer: Box<dyn Write + Send>,
        engine_version: &str,
        engine_options: Arc<Mutex<EngineOptions>>,
    ) -> Self {
        Self {
            inner: Arc::new(Mutex::new(UciOutInner {
                writer,
                engine_version: String::from(engine_version),
                debug: false,
                engine_options,
            })),
        }
    }

    pub fn set_debug(&self, tf: bool) {
        match self.inner.lock() {
            Ok(mut inner) => inner.debug = tf,
            Err(e) => panic!("{e}"),
        }
    }

    pub fn id(&mut self) -> Result<(), Box<dyn Error>> {
        match self.inner.lock() {
            Ok(mut inner) => {
                let version = inner.engine_version.clone();
                Ok(write!(
                    inner.writer,
                    "id name Fatalii {version}\nid author Patrick Heck\n",
                )?)
            }
            Err(e) => {
                self.info_string(format!("{e}").as_str())?;
                panic!("{e}")
            }
        }
    }

    pub fn all_options(&mut self) -> Result<(), Box<dyn Error>> {
        for opt in OPTIONS {
            self.option(&opt)?;
        }
        Ok(())
    }

    pub fn uci_ok(&mut self) -> Result<(), Box<dyn Error>> {
        match self.inner.lock() {
            Ok(mut inner) => Ok(writeln!(inner.writer, "uciok")?),
            Err(e) => {
                self.info_string(format!("{e}").as_str())?;
                panic!("{e}")
            }
        }
    }

    pub fn ready_ok(&mut self) -> Result<(), Box<dyn Error>> {
        match self.inner.lock() {
            Ok(mut inner) => Ok(writeln!(inner.writer, "readyok")?),
            Err(e) => {
                self.info_string(format!("{e}").as_str())?;
                panic!("{e}")
            }
        }
    }

    pub fn warn(&self, s: &str) -> Result<(), Box<dyn Error>> {
        match self.inner.lock() {
            Ok(mut inner) => Ok(writeln!(inner.writer, "info string warning: {s}")?),
            Err(e) => panic!("{e}"),
        }
    }

    fn option(&mut self, opt: &UciOption) -> Result<(), Box<dyn Error>> {
        match &opt.r#type {
            OptionType::Check(props) => match self.inner.lock() {
                Ok(mut inner) => writeln!(
                    inner.writer,
                    "option name {} type check default {}",
                    opt.name, props.default,
                )?,
                Err(e) => {
                    self.info_string(format!("{e}").as_str())?;
                    panic!("{e}")
                }
            },
            OptionType::Spin(props) => match self.inner.lock() {
                Ok(mut inner) => writeln!(
                    inner.writer,
                    "option name {} type spin default {} min {} max {}",
                    opt.name, props.default, props.min, props.max,
                )?,
                Err(e) => {
                    self.info_string(format!("{e}").as_str())?;
                    panic!("{e}")
                }
            },
            OptionType::Button | OptionType::Combo | OptionType::String => {
                unimplemented!();
            }
        }
        Ok(())
    }
}
