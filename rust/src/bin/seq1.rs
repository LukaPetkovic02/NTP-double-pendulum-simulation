use std::time::Instant;

#[derive(Clone, Copy)]
struct Params {
    m1: f64,
    m2: f64,
    l1: f64,
    l2: f64,
    g: f64,
}

fn derivatives(state: &[f64; 4], p: &Params) -> [f64; 4] {
    let (theta1, omega1, theta2, omega2) = (state[0], state[1], state[2], state[3]);
    let (m1, m2, l1, l2, g) = (p.m1, p.m2, p.l1, p.l2, p.g);

    let delta = theta2 - theta1;
    let sin_d = delta.sin();
    let cos_d = delta.cos();
    let denom = m1 + m2 * sin_d * sin_d;

    let dtheta1 = omega1;
    let dtheta2 = omega2;

    let domega1 = (
        m2 * l1 * omega1 * omega1 * sin_d * cos_d
            + m2 * g * theta2.sin() * cos_d
            + m2 * l2 * omega2 * omega2 * sin_d
            - (m1 + m2) * g * theta1.sin()
    ) / (l1 * denom);

    let domega2 = (
        -m2 * l2 * omega2 * omega2 * sin_d * cos_d
            + (m1 + m2)
                * (g * theta1.sin() * cos_d - l1 * omega1 * omega1 * sin_d - g * theta2.sin())
    ) / (l2 * denom);

    [dtheta1, domega1, dtheta2, domega2]
}

fn rk4_step(y: &[f64; 4], dt: f64, p: &Params) -> [f64; 4] {
    let k1 = derivatives(y, p);
    let y2 = [
        y[0] + 0.5 * dt * k1[0],
        y[1] + 0.5 * dt * k1[1],
        y[2] + 0.5 * dt * k1[2],
        y[3] + 0.5 * dt * k1[3],
    ];
    let k2 = derivatives(&y2, p);
    let y3 = [
        y[0] + 0.5 * dt * k2[0],
        y[1] + 0.5 * dt * k2[1],
        y[2] + 0.5 * dt * k2[2],
        y[3] + 0.5 * dt * k2[3],
    ];
    let k3 = derivatives(&y3, p);
    let y4 = [
        y[0] + dt * k3[0],
        y[1] + dt * k3[1],
        y[2] + dt * k3[2],
        y[3] + dt * k3[3],
    ];
    let k4 = derivatives(&y4, p);

    [
        y[0] + (dt / 6.0) * (k1[0] + 2.0 * k2[0] + 2.0 * k3[0] + k4[0]),
        y[1] + (dt / 6.0) * (k1[1] + 2.0 * k2[1] + 2.0 * k3[1] + k4[1]),
        y[2] + (dt / 6.0) * (k1[2] + 2.0 * k2[2] + 2.0 * k3[2] + k4[2]),
        y[3] + (dt / 6.0) * (k1[3] + 2.0 * k2[3] + 2.0 * k3[3] + k4[3]),
    ]
}

fn main() {
    let params = Params {
        m1: 1.0,
        m2: 1.0,
        l1: 1.0,
        l2: 1.0,
        g: 9.81,
    };
    let dt = 0.001;
    let steps = 600000;
    let mut y = [
        std::f64::consts::FRAC_PI_2,
        0.0,
        std::f64::consts::FRAC_PI_2 + 0.01,
        0.0,
    ];

    let start = Instant::now();
    for _ in 0..steps {
        y = rk4_step(&y, dt, &params);
    }
    let elapsed = start.elapsed().as_secs_f64();

    println!("Sekvencijalna simulacija završena za {:.4}s", elapsed);
}
