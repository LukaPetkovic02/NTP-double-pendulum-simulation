# Izveštaj o simulaciji dvostrukog klatna

## 1. Opis rešenja

Simulacija dvostrukog klatna implementirana je u programskom jeziku **Python**, korišćenjem **Runge–Kutta metode četvrtog reda (RK4)** za numeričku integraciju sistema diferencijalnih jednačina.  
Model se rešava iterativno kroz vremenske korake `dt`, pri čemu se u svakoj iteraciji izračunavaju novi uglovi (`θ₁`, `θ₂`) i ugaone brzine (`ω₁`, `ω₂`).  
Rezultati se čuvaju u CSV datotekama koje sadrže vremensku evoluciju sistema.

## 2. Sekvencijalna implementacija

Sekvencijalna verzija (`simulate.py`) izvršava simulaciju jedne putanje klatna i zapisuje rezultate u datoteku `traj_000.csv`.  
Za parametre `dt = 0.001` i `steps = 100000` simulacija je završena za:

- **Vreme izvođenja:** `9.65 s`  
- **Relativna promena energije:** `-0.0001%`

Energija sistema je praktično očuvana, što potvrđuje da je **RK4 metoda stabilna i dovoljno precizna** za izabrane parametre.

## 3. Paralelizovana implementacija

Paralelizovana verzija (`parallel_sim.py`) koristi Python biblioteku **`multiprocessing`** kako bi se pokrenulo više nezavisnih simulacija istovremeno.  
Na računaru sa **16 procesorskih jedinica** pokrenuto je 16 paralelnih simulacija, svaka sa istim brojem koraka (`100000`) i malim perturbacijama početnih uslova.

Rezultati su sačuvani u odvojenim CSV datotekama (`traj_000.csv` – `traj_015.csv`).

- **Prosečno trajanje po simulaciji:** ≈ 15.3 s  
- **Ukupno paralelno vreme:** `18.51 s`  
- **Sekvencijalno vreme (16 simulacija zaredom):** `16 × 9.65 = 154.4 s`

Paralelizacijom je postignuto približno **8× ubrzanje** u odnosu na sekvencijalno izvođenje.

## 4. Poređenje i zaključak

| Karakteristika | Sekvencijalno | Paralelno (16 simulacija) |
|----------------|----------------|----------------------------|
| Broj simulacija | 1 | 16 |
| Ukupno vreme | 9.65 s | 18.51 s |
| Vreme po simulaciji | 9.65 s | 15.3 s |
| Efektivno ubrzanje (ukupno) | 1× | ~8× |
| Očuvanje energije | ✔ Minimalni gubitak (-0.0001%) | ✔ Isti trend po simulaciji |

Paralelizacija omogućava značajno smanjenje ukupnog vremena izvođenja prilikom pokretanja većeg broja simulacija, uz zadržavanje tačnosti numeričke metode i stabilnosti energije.

## 5. Zaključak

Implementirane su:
- Sekvencijalna RK4 simulacija dvostrukog klatna  
- Paralelizovana verzija sa više nezavisnih pokretanja

Rezultati potvrđuju da sistem funkcioniše stabilno i energetski konzervativno, dok paralelizacija donosi realno ubrzanje u ukupnom vremenu obrade.

