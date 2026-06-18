use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;

const COLORS_OPTIONS: [Color; 7] =
  [Color::Green, Color::Yellow, Color::Red, Color::Blue, Color::Magenta, Color::Cyan, Color::Reset];

pub(crate) const TUI_MIN_MS: u32 = 250;
pub(crate) const TUI_MAX_MS: u32 = 10_000;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ViewType {
  Sparkline,
  Gauge,
}

/// Local serde-friendly mirror of `macmon_graph::SymbolSet`.
///
/// We keep a separate enum (rather than serializing `SymbolSet` directly) so
/// the config format stays decoupled from the library's type — the library
/// makes no serde guarantees, and this lets us evolve the two independently.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum GraphSymbol {
  Braille,
  Block,
  TTY,
}

impl GraphSymbol {
  pub fn to_symbol_set(self) -> macmon_graph::SymbolSet {
    match self {
      Self::Braille => macmon_graph::SymbolSet::Braille,
      Self::Block => macmon_graph::SymbolSet::Block,
      Self::TTY => macmon_graph::SymbolSet::TTY,
    }
  }
}

/// Local serde-friendly mirror of `macmon_graph::ColorPalette`.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy, Default)]
pub enum GraphColorPalette {
  #[default]
  Btop,
  Synthwave,
  Hacker,
  Aqua,
}

impl GraphColorPalette {
  pub fn to_palette(self) -> macmon_graph::ColorPalette {
    match self {
      Self::Btop => macmon_graph::ColorPalette::Btop,
      Self::Synthwave => macmon_graph::ColorPalette::Synthwave,
      Self::Hacker => macmon_graph::ColorPalette::Hacker,
      Self::Aqua => macmon_graph::ColorPalette::Aqua,
    }
  }
}

#[serde_inline_default]
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  #[serde_inline_default(ViewType::Sparkline)]
  pub view_type: ViewType,

  #[serde_inline_default(COLORS_OPTIONS[0])]
  pub color: Color,

  #[serde_inline_default(1000)]
  pub interval: u32,

  #[serde_inline_default(false)]
  pub per_core_view: bool,

  #[serde_inline_default(GraphSymbol::Braille)]
  pub graph_symbol: GraphSymbol,

  #[serde_inline_default(true)]
  pub colored_graphs: bool,

  #[serde_inline_default(GraphColorPalette::Btop)]
  pub color_palette: GraphColorPalette,
}

impl Default for Config {
  fn default() -> Self {
    serde_json::from_str("{}").unwrap()
  }
}

impl Config {
  fn normalize(mut self) -> Self {
    self.interval = self.interval.clamp(TUI_MIN_MS, TUI_MAX_MS);
    self
  }

  fn get_config_path() -> Option<String> {
    let home = match std::env::var("HOME") {
      Ok(home) => home,
      Err(_) => return None,
    };

    let filepath = format!("{}/.config/macmon.json", home);
    let _ = std::fs::create_dir_all(std::path::Path::new(&filepath).parent().unwrap());
    Some(filepath)
  }

  pub fn load() -> Self {
    if let Some(path) = Self::get_config_path() {
      let file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(_) => return Self::default().normalize(),
      };

      let reader = std::io::BufReader::new(file);
      let cfg: Self = serde_json::from_reader(reader).unwrap_or_default();
      return cfg.normalize();
    }

    Self::default().normalize()
  }

  pub fn save(&self) {
    if let Some(path) = Self::get_config_path() {
      let file = match std::fs::File::create(path) {
        Ok(file) => file,
        Err(_) => return,
      };

      let writer = std::io::BufWriter::new(file);
      let _ = serde_json::to_writer_pretty(writer, self);
    }
  }

  pub fn next_color(&mut self) {
    self.color = match COLORS_OPTIONS.iter().position(|&c| c == self.color) {
      Some(idx) => COLORS_OPTIONS[(idx + 1) % COLORS_OPTIONS.len()],
      None => COLORS_OPTIONS[0],
    };
    self.save();
  }

  pub fn next_view_type(&mut self) {
    self.view_type = match self.view_type {
      ViewType::Sparkline => ViewType::Gauge,
      ViewType::Gauge => ViewType::Sparkline,
    };
    self.save();
  }

  pub fn dec_interval(&mut self) {
    let step = 250;
    self.interval = (self.interval.saturating_sub(step).div_ceil(step) * step).max(TUI_MIN_MS);
    self.save();
  }

  pub fn inc_interval(&mut self) {
    let step = 250;
    self.interval = (self.interval.saturating_add(step) / step * step).min(TUI_MAX_MS);
    self.save();
  }

  pub fn toggle_per_core_view(&mut self) {
    self.per_core_view = !self.per_core_view;
    self.save();
  }

  pub fn next_graph_symbol(&mut self) {
    self.graph_symbol = match self.graph_symbol {
      GraphSymbol::Braille => GraphSymbol::Block,
      GraphSymbol::Block => GraphSymbol::TTY,
      GraphSymbol::TTY => GraphSymbol::Braille,
    };
    self.save();
  }

  pub fn next_graph_color(&mut self) {
    if !self.colored_graphs {
      self.colored_graphs = true;
      self.color_palette = GraphColorPalette::Btop;
    } else if self.color_palette == GraphColorPalette::Btop {
      self.color_palette = GraphColorPalette::Synthwave;
    } else if self.color_palette == GraphColorPalette::Synthwave {
      self.color_palette = GraphColorPalette::Hacker;
    } else if self.color_palette == GraphColorPalette::Hacker {
      self.color_palette = GraphColorPalette::Aqua;
    } else {
      self.colored_graphs = false;
      self.color_palette = GraphColorPalette::Btop;
    }
    self.save();
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use macmon_graph::SymbolSet;

  // An existing macmon.json predates the new keys. serde_inline_default must
  // fill them so older config files keep loading without error.
  #[test]
  fn defaults_apply_when_keys_absent() {
    let cfg: Config = serde_json::from_str("{}").unwrap();
    assert_eq!(cfg.graph_symbol, GraphSymbol::Braille);
    assert!(cfg.colored_graphs);
    assert_eq!(cfg.color_palette, GraphColorPalette::Btop);
  }

  // A config that only sets the old keys must still get the new defaults,
  // proving forward-compatible reads of pre-Phase-3 files.
  #[test]
  fn partial_config_gets_graph_defaults() {
    let cfg: Config = serde_json::from_str(r#"{"color":"Green","interval":1000}"#).unwrap();
    assert_eq!(cfg.graph_symbol, GraphSymbol::Braille);
    assert!(cfg.colored_graphs);
    assert_eq!(cfg.color_palette, GraphColorPalette::Btop);
  }

  #[test]
  fn new_keys_parse() {
    let cfg: Config =
      serde_json::from_str(r#"{"graph_symbol":"TTY","colored_graphs":false}"#).unwrap();
    assert_eq!(cfg.graph_symbol, GraphSymbol::TTY);
    assert!(!cfg.colored_graphs);
    assert_eq!(cfg.color_palette, GraphColorPalette::Btop);
  }

  #[test]
  fn graph_color_cycles_through_modes() {
    let mut cfg = Config::default();
    assert!(cfg.colored_graphs);
    assert_eq!(cfg.color_palette, GraphColorPalette::Btop);

    cfg.next_graph_color();
    assert!(cfg.colored_graphs);
    assert_eq!(cfg.color_palette, GraphColorPalette::Synthwave);

    cfg.next_graph_color();
    assert!(cfg.colored_graphs);
    assert_eq!(cfg.color_palette, GraphColorPalette::Hacker);

    cfg.next_graph_color();
    assert!(cfg.colored_graphs);
    assert_eq!(cfg.color_palette, GraphColorPalette::Aqua);

    cfg.next_graph_color();
    assert!(!cfg.colored_graphs);
    assert_eq!(cfg.color_palette, GraphColorPalette::Btop);

    cfg.next_graph_color();
    assert!(cfg.colored_graphs);
    assert_eq!(cfg.color_palette, GraphColorPalette::Btop);
  }

  #[test]
  fn graph_symbol_maps_to_library_symbol_set() {
    assert_eq!(GraphSymbol::Braille.to_symbol_set(), SymbolSet::Braille);
    assert_eq!(GraphSymbol::Block.to_symbol_set(), SymbolSet::Block);
    assert_eq!(GraphSymbol::TTY.to_symbol_set(), SymbolSet::TTY);
  }
}
