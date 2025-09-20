# Simulacija dvostrukog klatna
**Ciljna ocena: 10**

## Opis problema
Dvostruko klatno predstavlja haotični sistem koji se sastoji od dva povezana klatna. 
Prvi segment klatna je okačen za fiksnu tačku, dok je drugi segment okačen za kraj prvog segmenta.
Sistem pokazuje veoma kompleksno, nepredvidivo ponašanje uprkos jednostavnoj strukturi.

Problem se rešava numeričkim integrisanjem sistema od 4 diferencijalne jednačine prvog reda:
- θ₁, θ₂ - uglovi segmenata
- ω₁, ω₂ - ugaone brzine segmenata

## Metode rešavanja

### Numerička integracija
- Koristi se Runge-Kutta metoda 4. reda za numeričku integraciju
- Sistem diferencijalnih jednačina se rešava iterativno kroz vremenske korake

### Paralelizacija
**Python implementacija:**
- Sekvencijalna verzija - direktna implementacija RK4 metode
- Paralelna verzija - koristi multiprocessing za simulaciju različitih početnih uslova

**Rust implementacija:**
- Sekvencijalna verzija - optimizovana implementacija
- Paralelna verzija - koristi thread-ove za podelu vremenskih koraka ili početnih uslova
