fn trajectory_hits_target(
    target_x: (i32, i32),
    target_y: (i32, i32),
    mut x_vel: i32,
    mut y_vel: i32,
) -> (bool, i32) {
    let mut x = 0;
    let mut y = 0;
    let mut max_y = 0;
    // while we still _might_ be able to hit it...
    while y >= target_y.0 {
        if x >= target_x.0 && x <= target_x.1 && y >= target_y.0 && y <= target_y.1 {
            return (true, max_y);
        }
        x += x_vel;
        y += y_vel;
        if y >= max_y {
            max_y = y;
        }
        x_vel -= x_vel.signum();
        y_vel -= 1;
    }
    (false, max_y)
}

fn highest_y_position(x: (i32, i32), y: (i32, i32)) -> i32 {
    let mut max_y = i32::MIN;
    // if we go too high here we'll skip entirely
    for x_vel in 0..x.1 + 1 {
        for y_vel in y.0..-y.0 {
            let (reached, new_max) = trajectory_hits_target(x, y, x_vel, y_vel);
            if reached && new_max > max_y {
                max_y = new_max;
            }
        }
    }
    max_y
}

fn count_workable_vels(x: (i32, i32), y: (i32, i32)) -> i32 {
    let mut count = 0;
    // if we go too high here we'll skip entirely
    for x_vel in 0..x.1 + 1 {
        for y_vel in y.0..-y.0 {
            let (reached, _) = trajectory_hits_target(x, y, x_vel, y_vel);
            if reached {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let xs = (137, 171);
    let ys = (-98, -73);
    /*let xs = (20, 30);
    let ys = (-10, -5);*/
    println!("{}", highest_y_position(xs, ys));
    println!("{}", count_workable_vels(xs, ys));
}
