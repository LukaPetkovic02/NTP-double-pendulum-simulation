import numpy as np

def rk4_step(y, dt, f, params):
    k1 = f(y, params)
    k2 = f(y + 0.5 * dt * k1, params)
    k3 = f(y + 0.5 * dt * k2, params)
    k4 = f(y + dt * k3, params)
    return y + (dt / 6.0) * (k1 + 2*k2 + 2*k3 + k4)


def integrate(y0, dt, steps, f, params, record_energy_fn=None):
    """
    returns times, traj (N+1 x 4), energies (or None)
    """
    y = np.array(y0, dtype=float).copy()
    traj = np.zeros((steps + 1, y.size))
    times = np.zeros(steps + 1)
    energies = None
    if record_energy_fn:
        energies = np.zeros(steps + 1)

    traj[0] = y
    if energies is not None:
        energies[0] = record_energy_fn(y, params)

    for i in range(1, steps + 1):
        y = rk4_step(y, dt, f, params)
        traj[i] = y
        times[i] = i * dt
        if energies is not None:
            energies[i] = record_energy_fn(y, params)

    return times, traj, energies
