
use clap::{AppSettings, Clap};

#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "Stavenko V. G. <stavenko@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
  // #[clap(short)]
  // ins: Vec<u8>,
  // #[clap(short)]
  // outs: Vec<u8>,
  #[clap(long)]
  x_enb: u8,
  #[clap(long)]
  x_end: u8,
  #[clap(long)]
  x_clk: u8,
  #[clap(long)]
  x_dir: u8,

  #[clap(long)]
  y_enb: u8,
  #[clap(long)]
  y_end: u8,
  #[clap(long)]
  y_clk: u8,
  #[clap(long)]
  y_dir: u8,

  #[clap(long)]
  z_enb: u8,
  #[clap(long)]
  z_end: u8,
  #[clap(long)]
  z_clk: u8,
  #[clap(long)]
  z_dir: u8,
}

/*
loadrt hal_ext_gpio dir=78855 exclude=32918520

addf hal_ext_gpio.read base-thread
addf hal_ext_gpio.write base-thread

net Xen => hal_ext_gpio.pin-03-out
*/


static RPI2_GPIOS: [u8; 26] = [2, 3, 4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 22, 21, 23, 24, 25, 26, 27];
static RPI2_PINS: [u8; 26] =  [3, 5, 7, 29, 31, 26, 24, 21, 19, 23, 32, 33,  8, 10, 36, 11, 12, 35, 38, 15, 40, 16, 18, 22, 37, 13 ];


fn calculate(arr: &[u8]) -> u32 {
  let mut summ = 0;
  for i in 0..arr.len() {
    if arr[i] == 1 {
      summ += 2_u32.pow(i as u32);
    }
  }
  summ
}

fn pin(gpio: u8) -> u8 {
  if let Some(ix) = RPI2_GPIOS.iter().position(|j| *j == gpio) {
    return RPI2_PINS[ix];
  }
  panic!("no such gpio found");
}

fn main() {
  let opts: Opts = Opts::parse();
  let mut exclude = Vec::new();
  let mut setup = Vec::new();
  for _ in 0..26 {
    exclude.push(1);
    setup.push(0);
  }

  for i in [opts.x_end, opts.y_end, opts.z_end].iter() {
    if let Some(ix)= RPI2_GPIOS.iter().position(|j| *j == *i){
      exclude[ix] = 0;
    }
  }

  for i in [
    opts.x_dir, opts.x_enb, opts.x_clk,
    opts.y_dir, opts.y_enb, opts.y_clk,
    opts.z_dir, opts.z_enb, opts.z_clk,
  ].iter() {
    if let Some(ix)= RPI2_GPIOS.iter().position(|j| *j == *i){
      setup[ix] = 1;
      exclude[ix] = 0;
    }
  }

  println!("loadrt hal_ext_gpio dir={} exclude={}", calculate(&setup), calculate(&exclude));
  println!("addf hal_ext_gpio.read base-thread");
  println!("addf hal_ext_gpio.write base-thread");

  println!("net Xen => hal_ext_gpio.pin-{}-out", pin(opts.x_enb));
  println!("net Yen => hal_ext_gpio.pin-{}-out", pin(opts.y_enb));
  println!("net Zen => hal_ext_gpio.pin-{}-out", pin(opts.z_enb));

  println!("net Xstep => hal_ext_gpio.pin-{}-out", pin(opts.x_clk));
  println!("net Ystep => hal_ext_gpio.pin-{}-out", pin(opts.y_clk));
  println!("net Zstep => hal_ext_gpio.pin-{}-out", pin(opts.z_clk));

  println!("net Xdir => hal_ext_gpio.pin-{}-out", pin(opts.x_dir));
  println!("net Ydir => hal_ext_gpio.pin-{}-out", pin(opts.y_dir));
  println!("net Zdir => hal_ext_gpio.pin-{}-out", pin(opts.z_dir));
  
  println!("net Xhome hal_ext_gpio.pin-{}-in => joint.0.home-sw-in joint.0.neg-lim-sw-in", pin(opts.x_end));
  println!("net Yhome hal_ext_gpio.pin-{}-in => joint.1.home-sw-in joint.1.neg-lim-sw-in", pin(opts.y_end));
  println!("net Zhome hal_ext_gpio.pin-{}-in => joint.2.home-sw-in joint.2.neg-lim-sw-in", pin(opts.z_end));
}
