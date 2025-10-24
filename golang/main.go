package main

import (
	"bufio"
	"encoding/csv"
	"flag"
	"fmt"
	"math"
	"math/rand"
	"os"
	"path/filepath"
	"runtime"
	"sync"
	"time"
)

/*
Double pendulum model (planar):
State: [θ1, ω1, θ2, ω2]
Parameters: m1, m2, L1, L2, g
Integrator: RK4
Parallelization: ensemble (independent scenarios) via goroutines + worker pool.
Outputs: CSV per run (t,theta1,omega1,theta2,omega2,energy), optional GIF (one scenario).
*/

type Params struct {
	m1, m2 float64
	L1, L2 float64
	g      float64
}

type State struct {
	Th1, Om1 float64
	Th2, Om2 float64
}

type Sample struct {
	T   float64
	S   State
	Eng float64
}

func deriv(p Params, s State) State {
	m1, m2, L1, L2, g := p.m1, p.m2, p.L1, p.L2, p.g
	th1, om1, th2, om2 := s.Th1, s.Om1, s.Th2, s.Om2

	delta := th2 - th1

	den1 := (m1+m2)*L1 - m2*L1*math.Cos(delta)*math.Cos(delta)
	den2 := (L2 / L1) * den1

	a1 := (m2*L1*om1*om1*math.Sin(delta)*math.Cos(delta) +
		m2*g*math.Sin(th2)*math.Cos(delta) +
		m2*L2*om2*om2*math.Sin(delta) -
		(m1+m2)*g*math.Sin(th1)) / den1

	a2 := (-m2*L2*om2*om2*math.Sin(delta)*math.Cos(delta) +
		(m1+m2)*(g*math.Sin(th1)*math.Cos(delta)-
			L1*om1*om1*math.Sin(delta)-g*math.Sin(th2))) / den2

	return State{
		Th1: om1,
		Om1: a1,
		Th2: om2,
		Om2: a2,
	}
}

func rk4Step(p Params, s State, dt float64) State {
	k1 := deriv(p, s)

	s2 := State{
		Th1: s.Th1 + 0.5*dt*k1.Th1,
		Om1: s.Om1 + 0.5*dt*k1.Om1,
		Th2: s.Th2 + 0.5*dt*k1.Th2,
		Om2: s.Om2 + 0.5*dt*k1.Om2,
	}
	k2 := deriv(p, s2)

	s3 := State{
		Th1: s.Th1 + 0.5*dt*k2.Th1,
		Om1: s.Om1 + 0.5*dt*k2.Om1,
		Th2: s.Th2 + 0.5*dt*k2.Th2,
		Om2: s.Om2 + 0.5*dt*k2.Om2,
	}
	k3 := deriv(p, s3)

	s4 := State{
		Th1: s.Th1 + dt*k3.Th1,
		Om1: s.Om1 + dt*k3.Om1,
		Th2: s.Th2 + dt*k3.Th2,
		Om2: s.Om2 + dt*k3.Om2,
	}
	k4 := deriv(p, s4)

	return State{
		Th1: s.Th1 + dt*(k1.Th1+2*k2.Th1+2*k3.Th1+k4.Th1)/6.0,
		Om1: s.Om1 + dt*(k1.Om1+2*k2.Om1+2*k3.Om1+k4.Om1)/6.0,
		Th2: s.Th2 + dt*(k1.Th2+2*k2.Th2+2*k3.Th2+k4.Th2)/6.0,
		Om2: s.Om2 + dt*(k1.Om2+2*k2.Om2+2*k3.Om2+k4.Om2)/6.0,
	}
}

func energy(p Params, s State) float64 {
	// Total mechanical energy
	m1, m2, L1, L2, g := p.m1, p.m2, p.L1, p.L2, p.g
	th1, om1, th2, om2 := s.Th1, s.Om1, s.Th2, s.Om2

	//x1 := L1 * math.Sin(th1)
	y1 := -L1 * math.Cos(th1)

	//x2 := x1 + L2*math.Sin(th2)
	y2 := y1 - L2*math.Cos(th2)

	vx1 := L1 * om1 * math.Cos(th1)
	vy1 := L1 * om1 * math.Sin(th1)

	vx2 := vx1 + L2*om2*math.Cos(th2)
	vy2 := vy1 + L2*om2*math.Sin(th2)

	KE := 0.5*m1*(vx1*vx1+vy1*vy1) + 0.5*m2*(vx2*vx2+vy2*vy2)
	PE := m1*g*(y1+L1) + m2*g*(y2+L1+L2) // shift so PE>=0
	return KE + PE
}

func simulate(p Params, s0 State, dt float64, steps int, recordEvery int) []Sample {
	out := make([]Sample, 0, steps/recordEvery+1)
	s := s0
	t := 0.0
	for i := 0; i < steps; i++ {
		if i%recordEvery == 0 {
			out = append(out, Sample{T: t, S: s, Eng: energy(p, s)})
		}
		s = rk4Step(p, s, dt)
		t += dt
	}
	// record last
	out = append(out, Sample{T: t, S: s, Eng: energy(p, s)})
	return out
}

func writeCSV(path string, samples []Sample) error {
	f, err := os.Create(path)
	if err != nil {
		return err
	}
	defer f.Close()
	w := csv.NewWriter(bufio.NewWriter(f))
	defer w.Flush()

	_ = w.Write([]string{"t", "theta1", "omega1", "theta2", "omega2", "energy"})
	for _, sm := range samples {
		rec := []string{
			fmt.Sprintf("%.9f", sm.T),
			fmt.Sprintf("%.9f", sm.S.Th1),
			fmt.Sprintf("%.9f", sm.S.Om1),
			fmt.Sprintf("%.9f", sm.S.Th2),
			fmt.Sprintf("%.9f", sm.S.Om2),
			fmt.Sprintf("%.12f", sm.Eng),
		}
		if err := w.Write(rec); err != nil {
			return err
		}
	}
	return nil
}

type job struct {
	id   int
	s0   State
	seed int64
}

type result struct {
	id       int
	duration time.Duration
	err      error
	path     string
}

func worker(p Params, dt float64, steps int, recordEvery int, outDir string, jobs <-chan job, results chan<- result) {
	for jb := range jobs {
		samples := simulate(p, jb.s0, dt, steps, recordEvery)
		path := filepath.Join(outDir, fmt.Sprintf("run_%04d.csv", jb.id))
		err := writeCSV(path, samples)
		results <- result{id: jb.id, duration: 0, err: err, path: path}
	}
}

func ensureDir(path string) error {
	if path == "" {
		return nil
	}
	return os.MkdirAll(path, 0o755)
}

func main() {
	// Physical/default params
	var (
		m1    = flag.Float64("m1", 1.0, "mass 1 (kg)")
		m2    = flag.Float64("m2", 1.0, "mass 2 (kg)")
		L1    = flag.Float64("L1", 1.0, "rod 1 length (m)")
		L2    = flag.Float64("L2", 1.0, "rod 2 length (m)")
		g     = flag.Float64("g", 9.81, "gravity (m/s^2)")
		th1   = flag.Float64("th1", math.Pi/2, "initial theta1 (rad)")
		th2   = flag.Float64("th2", math.Pi/2+0.01, "initial theta2 (rad)")
		om1   = flag.Float64("om1", 0.0, "initial omega1 (rad/s)")
		om2   = flag.Float64("om2", 0.0, "initial omega2 (rad/s)")
		dt    = flag.Float64("dt", 0.001, "time step (s)")
		steps = flag.Int("steps", 60000, "integration steps")
		// Recording / outputs
		recordEvery = flag.Int("record-every", 1, "record every N steps")
		outDir      = flag.String("out", "out_go", "output directory")
		// Ensemble / parallel
		runs    = flag.Int("runs", 1, "number of independent scenarios")
		jitter  = flag.Float64("jitter", 0.0, "random jitter added to th2 (±jitter radians)")
		workers = flag.Int("workers", 0, "goroutines (0 → GOMAXPROCS)")
		seed    = flag.Int64("seed", 42, "random seed")
	)
	flag.Parse()

	if *workers <= 0 {
		*workers = runtime.GOMAXPROCS(0)
	}
	if err := ensureDir(*outDir); err != nil {
		fmt.Println("failed to create out dir:", err)
		os.Exit(1)
	}

	p := Params{m1: *m1, m2: *m2, L1: *L1, L2: *L2, g: *g}
	base := State{Th1: *th1, Om1: *om1, Th2: *th2, Om2: *om2}

	fmt.Printf("Go double-pendulum | steps=%d dt=%.6f runs=%d workers=%d out=%s\n", *steps, *dt, *runs, *workers, *outDir)

	start := time.Now()

	// Build jobs
	rng := rand.New(rand.NewSource(*seed))
	jobsCh := make(chan job, *runs)
	resultsCh := make(chan result, *runs)

	// Workers
	wg := sync.WaitGroup{}
	wg.Add(*workers)
	for w := 0; w < *workers; w++ {
		go func() {
			defer wg.Done()
			worker(p, *dt, *steps, *recordEvery, *outDir, jobsCh, resultsCh)
		}()
	}

	// Enqueue jobs
	for i := 0; i < *runs; i++ {
		s0 := base
		if *jitter > 0 {
			// Slight change on θ2 per scenario (chaos showcase)
			d := (*jitter) * (2*rng.Float64() - 1)
			s0.Th2 += d
		}
		jobsCh <- job{id: i + 1, s0: s0, seed: rng.Int63()}
	}
	close(jobsCh)

	// Collect
	go func() {
		wg.Wait()
		close(resultsCh)
	}()

	ok := 0
	for res := range resultsCh {
		if res.err != nil {
			fmt.Printf("run %d FAILED: %v\n", res.id, res.err)
			continue
		}
		ok++
	}
	elapsed := time.Since(start)
	fmt.Printf("Finished %d/%d runs in %v\n", ok, *runs, elapsed)
}
