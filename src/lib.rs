use sysinfo::{Disks, System};

pub fn get_sys_info() {
  let mut system = System::new_all();
  system.refresh_all();

  println!("Total memory: {} KB", system.total_memory());
  println!("Available memory: {} KB", system.available_memory());

  let load_avg = System::load_average();
  println!(
      "Load Average: 1 min: {}, 5 min: {}, 15 min: {}",
      load_avg.one, load_avg.five, load_avg.fifteen
  );

  for (i, cpu) in system.cpus().iter().enumerate() {
      println!("CPU {} load: {}", i, cpu.cpu_usage());
  }

  let disks = Disks::new_with_refreshed_list();
  for disk in &disks {
      println!(
          "Disk {}: Total size: {} KB, Available: {} KB",
          disk.name().to_str().unwrap(),
          disk.total_space() / 1024,
          disk.available_space() / 1024,
      )
  }
}