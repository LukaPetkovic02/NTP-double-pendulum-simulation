import numpy as np

# Funkcija koja računa derivacije i funkciju za energiju
def derivatives(state, params):
    """
    state = [theta1, omega1, theta2, omega2]
    params: dict with m1,m2,l1,l2,g
    returns np.array([dtheta1, domega1, dtheta2, domega2])
    """
    theta1, omega1, theta2, omega2 = state
    m1 = params.get("m1", 1.0)
    m2 = params.get("m2", 1.0)
    l1 = params.get("l1", 1.0)
    l2 = params.get("l2", 1.0)
    g  = params.get("g", 9.81)

    delta = theta2 - theta1
    sin_d = np.sin(delta)
    cos_d = np.cos(delta)
    denom = m1 + m2 * sin_d**2

    dtheta1 = omega1
    dtheta2 = omega2

    domega1 = (
        m2 * l1 * omega1**2 * sin_d * cos_d
        + m2 * g * np.sin(theta2) * cos_d
        + m2 * l2 * omega2**2 * sin_d
        - (m1 + m2) * g * np.sin(theta1)
    ) / (l1 * denom)

    domega2 = (
        - m2 * l2 * omega2**2 * sin_d * cos_d
        + (m1 + m2) * (g * np.sin(theta1) * cos_d - l1 * omega1**2 * sin_d - g * np.sin(theta2))
    ) / (l2 * denom)

    return np.array([dtheta1, domega1, dtheta2, domega2])


def energy(state, params):
    """Kinetička + potencijalna energija (koristi iste koordinate kao u derivatives)."""
    theta1, omega1, theta2, omega2 = state
    m1 = params.get("m1", 1.0)
    m2 = params.get("m2", 1.0)
    l1 = params.get("l1", 1.0)
    l2 = params.get("l2", 1.0)
    g  = params.get("g", 9.81)

    x1 = l1 * np.sin(theta1)
    y1 = -l1 * np.cos(theta1)
    x2 = x1 + l2 * np.sin(theta2)
    y2 = y1 - l2 * np.cos(theta2)

    v1x = l1 * omega1 * np.cos(theta1)
    v1y = l1 * omega1 * np.sin(theta1)
    v2x = v1x + l2 * omega2 * np.cos(theta2)
    v2y = v1y + l2 * omega2 * np.sin(theta2)

    T = 0.5 * m1 * (v1x**2 + v1y**2) + 0.5 * m2 * (v2x**2 + v2y**2)
    V = m1 * g * y1 + m2 * g * y2

    return T + V
