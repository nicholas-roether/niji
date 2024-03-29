use std::rc::Rc;

use thiserror::Error;

use crate::{
	config::{self, Config, Theme},
	file_manager::{self, FileManager},
	files::{self, Files},
	module_manager::{self, ModuleManager, ModuleManagerInit},
	theme_manager::{self, ThemeManager},
	utils::xdg::{self, XdgDirs}
};

#[derive(Debug, Error)]
#[error("{0}")]
pub enum Error {
	Xdg(#[from] xdg::Error),
	Files(#[from] files::Error),
	Config(#[from] config::Error),
	FileManager(#[from] file_manager::Error),
	ThemeManager(#[from] theme_manager::Error),
	ModuleManager(#[from] module_manager::Error)
}

pub struct NijiApp {
	_xdg: Rc<XdgDirs>,
	_files: Rc<Files>,
	config: Rc<Config>,
	_file_manager: Rc<FileManager>,
	theme_manager: Rc<ThemeManager>,
	module_manager: Rc<ModuleManager>
}

impl NijiApp {
	pub fn init() -> Result<Self, Error> {
		let xdg = Rc::new(XdgDirs::new()?);
		let files = Rc::new(Files::new(&xdg)?);
		let config = Rc::<Config>::new(config::read(files.config_file())?);
		let file_manager = Rc::new(FileManager::new(Rc::clone(&files))?);
		let theme_manager = Rc::new(ThemeManager::new(Rc::clone(&files)));
		let module_manager = Rc::new(ModuleManager::new(ModuleManagerInit {
			xdg: Rc::clone(&xdg),
			files: Rc::clone(&files),
			config: Rc::clone(&config),
			file_manager: Rc::clone(&file_manager)
		})?);

		Ok(Self {
			_xdg: xdg,
			_files: files,
			config,
			_file_manager: file_manager,
			theme_manager,
			module_manager
		})
	}

	pub fn current_theme(&self) -> Result<Theme, Error> {
		Ok(self.theme_manager.current_theme()?)
	}

	pub fn get_theme(&self, name: &str) -> Result<Theme, Error> {
		Ok(self.theme_manager.get_theme(name)?)
	}

	pub fn list_themes(&self) -> Vec<String> {
		self.theme_manager.list_themes()
	}

	pub fn apply(&self, reload: bool, modules: Option<&[String]>) -> Result<(), Error> {
		let theme = self.current_theme()?;
		self.module_manager
			.apply(&self.config, &theme, reload, modules)?;
		Ok(())
	}

	pub fn unset_theme(&self) -> Result<(), Error> {
		Ok(self.theme_manager.unset_theme()?)
	}

	pub fn set_theme(&self, name: &str) -> Result<(), Error> {
		self.theme_manager.set_theme(name.to_string())?;
		Ok(())
	}
}
