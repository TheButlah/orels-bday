use std::io::Write as _;
use std::io::stdin;
use std::io::stdout;
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;
use std::time::Duration;

use color_eyre::{Result, eyre::Context};
use crossterm::terminal::Clear;
use crossterm::{QueueableCommand, terminal::ClearType};
use image::{AnimationDecoder as _, DynamicImage, codecs::gif::GifDecoder};
use owo_colors::OwoColorize as _;
use rascii_art::RenderOptions;

const BEAR_HUG: &[u8] =
	include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/bear-hug.gif"));
const POP_CAT: &[u8] =
	include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/pop-cat.gif"));
const CANADA: &[u8] =
	include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/canada.gif"));
const BREAD: &[u8] =
	include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/bread-plane.gif"));
const CAKE: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/cake.gif"));

fn main() -> Result<()> {
	color_eyre::install()?;

	stdout().queue(Clear(ClearType::FromCursorUp))?;
	println!("{}", "HAPPY BIRTHDAY ORBLES!".bold().bright_red());
	println!(
		"{}",
		"press enter to cycle through pages of the card"
			.italic()
			.dimmed()
	);
	wait_for_enter().recv().ok();

	println!(
		"I remember meeting you a long time ago in an instance with Paly. I really enjoyed talking and cuddling {}",
		":3".bold().bright_red()
	);
	wait_for_enter().recv().ok();

	render_gif(wait_for_enter(), &BEAR_HUG).call()?;

	println!(
		"After a while, we ended up talking a lot about various {}",
		"programming things.".bold().bright_yellow()
	);
	wait_for_enter().recv().ok();
	println!(
		"{}{}",
		"I especially enjoyed our lil yap sessions about tech art".bold(),
		" - its not really an area I know much about and its very endearing hearing you info dump about it."
	);
	wait_for_enter().recv().ok();

	render_gif(wait_for_enter(), &POP_CAT).call()?;

	println!("{}", "(That was you yapping)".italic().dimmed());
	wait_for_enter().recv().ok();
	println!(
		"You are a very fun guy to yap to! {}{}{}",
		"One might say you are a ".bright_green().bold(),
		"cool".cyan().italic().bold(),
		" bean.".bright_green().bold(),
	);
	wait_for_enter().recv().ok();
	stdout().queue(Clear(ClearType::FromCursorUp))?;
	print!(
		"Eventually I got to meet your other friends in discord and became friends with them too. \
        Its clear how much {}, and its because you're {} ",
		"everyone appreciates and values you".bold().bright_green(),
		"pretty".bold().bright_magenta()
	);
	stdout().flush().ok();
	let pos = crossterm::cursor::position()?;
	wait_for_enter().recv().ok();
	stdout()
		.queue(crossterm::cursor::MoveTo(pos.0, pos.1 - 1))?
		.flush()?;
	println!("{}", "great".bold().bright_magenta());
	wait_for_enter().recv().ok();
	stdout().queue(Clear(ClearType::FromCursorUp))?;

	print!(
		"{}{}{}",
		"One of the high points of the last few months was ",
		"being able to finally meet you IRL! ".bright_green().bold(),
		"I traveled to the land of "
	);
	canada(&mut stdout())?;
	println!(
		", {}{}",
		"braving the cold".bright_blue(),
		" after I forgot my jacket."
	);
	wait_for_enter().recv().ok();

	render_gif(wait_for_enter(), &CANADA).call()?;

	println!(
		"I {}{}{}{}",
		"barely survived".italic().bright_purple(),
		" the figid lands with my life intact, but luckily the ",
		"bread".bold().bright_yellow(),
		" saved me."
	);
	wait_for_enter().recv().ok();

	render_gif(wait_for_enter(), &BREAD).reversed(true).call()?;

	println!(
		"{} for making the last couple years special, and I hope your birthday today {}",
		"is as special as you are.".bright_magenta(),
		"Thanks".bold(),
	);
	wait_for_enter().recv().ok();
	render_gif(wait_for_enter(), &CAKE).reversed(true).call()?;

	println!("- Butlah {}", "<3".bold().bright_red());

	Ok(())
}

fn canada(out: &mut impl std::io::Write) -> std::io::Result<()> {
	let alternate = [true, false];
	let canada = "Canadia"; // intentional
	for (c, is_red) in canada.chars().zip(alternate.into_iter().cycle()) {
		if is_red {
			write!(out, "{}", c.bright_red().bold())?;
		} else {
			write!(out, "{}", c.bright_white().bold())?;
		}
	}

	Ok(())
}

fn wait_for_enter() -> mpsc::Receiver<()> {
	let (tx, rx) = mpsc::channel();
	std::thread::spawn(move || {
		stdin().read_line(&mut String::new()).ok();
		std::thread::sleep(Duration::from_millis(100));
		drop(tx)
	});
	rx
}

struct LineWrap;
impl LineWrap {
	fn enable() -> Self {
		stdout().queue(crossterm::terminal::EnableLineWrap).ok();
		Self
	}
}
impl Drop for LineWrap {
	fn drop(&mut self) {
		stdout()
			.queue(crossterm::terminal::EnableLineWrap)
			.and_then(|s| s.flush())
			.ok();
	}
}

#[bon::builder]
fn render_gif(
	#[builder(start_fn)] stop_signal: mpsc::Receiver<()>,
	#[builder(start_fn)] gif: &[u8],
	#[builder(default = u64::MAX)] n_loop: u64,
	#[builder(default)] reversed: bool,
) -> Result<()> {
	let mut frames = GifDecoder::new(gif)
		.map(|d| d.into_frames())
		.and_then(|f| f.collect_frames())
		.wrap_err("failed to decode gif")?;

	let mut rendered = String::new();
	let mut stdout = std::io::stdout();
	let _line_wrap = LineWrap::enable();
	if reversed {
		frames.reverse();
	}
	let it = frames
		.iter()
		.cycle()
		.take(frames.len().saturating_mul(n_loop as usize));
	for frame in it {
		let Err(TryRecvError::Empty) = stop_signal.try_recv() else {
			break;
		};
		rendered.clear();
		let rgba = frame.buffer();
		let width = rgba.width();
		let height = rgba.height();
		// terminal cells are approximately 2:1 aspect ratio, so squish the image
		// vertically first.
		let width = width * 2;
		let delay: Duration = frame.delay().into();
		std::thread::sleep(delay);

		let img = DynamicImage::from(frame.clone().into_buffer());
		let (t_width, t_height) =
			crossterm::terminal::size().wrap_err("failed to get term size")?;
		let new_height = (height as f32 / width as f32 * t_width as f32).ceil() as u16;
		let new_height = t_height.min(new_height);
		let new_width = width as f32 / height as f32 * new_height as f32;

		rascii_art::render_image_to(
			&img,
			&mut rendered,
			&RenderOptions::new()
				.width(new_width.ceil() as u32 - 4)
				.height(new_height as u32 - 4)
				.colored(true)
				.charset(rascii_art::charsets::RUSSIAN),
		)
		.wrap_err("failed to render ascii")?;

		stdout.queue(Clear(ClearType::FromCursorUp))?;
		write!(&mut stdout, "{rendered}")?;
		stdout.flush()?;
	}
	stdout.queue(Clear(ClearType::FromCursorUp))?;
	println!();

	Ok(())
}
