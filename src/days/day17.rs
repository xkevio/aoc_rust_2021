const INPUT: &str = "target area: x=60..94, y=-171..-136";

struct Probe {
    pos: (i64, i64),
    velocity: (i64, i64),
}

impl Probe {
    pub fn new(vx: i64, vy: i64) -> Self {
        Probe { pos: (0, 0), velocity: (vx, vy) }
    }

    pub fn step(&mut self) {
        self.pos.0 += self.velocity.0;
        self.pos.1 += self.velocity.1;

        if self.velocity.0 > 0 {
            self.velocity.0 -= 1;
        } else if self.velocity.0 < 0 {
            self.velocity.0 += 1;
        }

        self.velocity.1 -= 1;
    }

    pub fn get_pos(&self) -> &(i64, i64) {
        &self.pos
    }
    pub fn get_vel(&self) -> &(i64, i64) {
        &self.velocity
    }
}

fn get_range() -> (Vec<i64>, Vec<i64>) {
    let nums: Vec<i64> = INPUT
        .split(|c: char| if c != '-' { !c.is_numeric() } else { false })
        .flat_map(|f| f.parse())
        .collect();
    ((nums[0]..=nums[1]).collect(), (nums[2]..=nums[3]).collect())
}

fn launch_probe() -> (usize, usize) {
    let (x_range, y_range) = get_range();

    let mut y_list: Vec<i64> = Vec::new();
    let mut vel_list: Vec<(i64, i64)> = Vec::new();

    for vx in 0..200 {
        for vy in -200..200 {
            let mut probe = Probe::new(vx, vy);
            let mut local_list: Vec<i64> = Vec::new();

            let init_vel = (vx, vy);

            loop {
                probe.step();

                let (x, y) = probe.get_pos();
                local_list.push(*y);

                if x_range.iter().all(|f| *x > *f)
                    || (x_range.iter().any(|f| *x >= *f) && y_range.iter().all(|f| *y < *f)) {
                    break;
                }

                if probe.get_vel().0 == 0 && x_range.iter().all(|f| *x < *f) {
                    break;
                }

                if x_range.contains(x) && y_range.contains(y) {
                    y_list.append(&mut local_list);
                    vel_list.push(init_vel);
                    break;
                }
            }
        }
    }

    (*y_list.iter().max().unwrap() as usize, vel_list.len())
}

pub fn part1() -> usize {
    launch_probe().0
}

pub fn part2() -> usize {
    launch_probe().1
}
